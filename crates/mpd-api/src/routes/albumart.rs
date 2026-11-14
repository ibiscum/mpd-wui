use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};

use axum::body::Bytes;

use crate::AppState;
use crate::error::ApiError;

use super::param;

const ART_CACHE_CONTROL: &str = "public, max-age=3600";

pub async fn albumart(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let uri = param(&params, "uri").ok_or_else(|| ApiError::bad_request("missing ?uri="))?;
    let key = format!("art:{uri}");
    let if_none_match = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok());

    if let Some(hit) = state.cache.get_art(&key) {
        if etag_matches(if_none_match, &hit.etag) {
            return not_modified(&hit.etag);
        }
        let mime = hit
            .mime
            .unwrap_or_else(|| "application/octet-stream".to_string());
        return art_response(hit.value, &hit.etag, &mime);
    }

    let (bytes, mime) = match state.client.read_picture(uri).await {
        Some(art) => art,
        None => return Err(ApiError::not_found("album art not found")),
    };
    let hit = state.cache.store_art(&key, Bytes::from(bytes), &mime);
    if etag_matches(if_none_match, &hit.etag) {
        return not_modified(&hit.etag);
    }
    art_response(hit.value, &hit.etag, &mime)
}

fn art_response(value: Bytes, etag: &str, mime: &str) -> Result<Response, ApiError> {
    let content_type = HeaderValue::from_str(mime).map_err(|error| {
        tracing::warn!(%mime, %error, "invalid album art MIME type");
        ApiError::new(StatusCode::BAD_GATEWAY, "invalid album art metadata")
    })?;
    let etag = HeaderValue::from_str(etag).map_err(|error| {
        tracing::warn!(%etag, %error, "invalid album art ETag");
        ApiError::new(StatusCode::BAD_GATEWAY, "invalid album art metadata")
    })?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type);
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static(ART_CACHE_CONTROL),
    );
    headers.insert(header::ETAG, etag);
    Ok((headers, value).into_response())
}

fn not_modified(etag: &str) -> Result<Response, ApiError> {
    let etag = HeaderValue::from_str(etag).map_err(|error| {
        tracing::warn!(%etag, %error, "invalid album art ETag");
        ApiError::new(StatusCode::BAD_GATEWAY, "invalid album art metadata")
    })?;
    let mut headers = HeaderMap::new();
    headers.insert(header::ETAG, etag);
    Ok((StatusCode::NOT_MODIFIED, headers).into_response())
}

fn etag_matches(if_none_match: Option<&str>, etag: &str) -> bool {
    let value = match if_none_match {
        Some(value) => value,
        None => return false,
    };
    let value = value.trim();
    if value == "*" {
        return true;
    }
    value.split(',').any(|candidate| candidate.trim() == etag)
}
