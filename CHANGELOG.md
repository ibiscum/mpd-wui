# Changelog

## Unreleased

- Hardened album-art cache validators by switching ETag generation from a 
  64-bit non-cryptographic hash to SHA-256 hex output in
  [crates/mpd-api/src/cache.rs](./crates/mpd-api/src/cache.rs).
- Hardened cache invalidation safety by clearing the album-art and library
  caches when the MPD event stream reports lag (dropped events), preventing
  stale data if a `DatabaseChanged` notification was skipped.
- Made fuzzy search result ordering deterministic for score ties in
  [crates/mpd-api/src/search.rs](./crates/mpd-api/src/search.rs) by adding
  stable kind-specific tie-breakers, preventing run-to-run ordering drift.
- Hardened library snapshot refresh consistency in
  [crates/mpd-api/src/store.rs](./crates/mpd-api/src/store.rs) by rejecting
  stale async writes after invalidation and gating background refresh spawns
  to a single queued/running task.
- Hardened WebSocket snapshot consistency in
  [crates/mpd-api/src/ws.rs](./crates/mpd-api/src/ws.rs) by forcing a snapshot
  resync when the event stream lags and adding warning logs when fresh initial
  snapshots fail and fallback to cached state.
- Hardened MPD command I/O reliability in
  [crates/mpd-client/src/client.rs](./crates/mpd-client/src/client.rs) by
  bounding command-queue sends and session reads/writes with command timeouts,
  and by rejecting invalid binary payload delimiters in response framing.
- Hardened album-art protocol parsing in
  [crates/mpd-client/src/protocol.rs](./crates/mpd-client/src/protocol.rs) by
  requiring a trailing `OK`/`ACK` terminator after binary payloads.
- Hardened mock MPD fidelity in
  [crates/mpd-mock/src/lib.rs](./crates/mpd-mock/src/lib.rs) by enforcing
  password authentication before non-password commands and deriving `next_id`
  from the highest existing playlist id.
- Fixed keyboard volume adjustments in
  [web/js/app.js](./web/js/app.js) to preserve MPD volumes above 100 instead
  of force-clamping them to 100.
- Added a shared configurable frontend volume cap via
  [web/js/config.js](./web/js/config.js) (`meta[name="volume-max"]`), and
  aligned slider + keyboard volume controls in
  [web/js/nowplaying.js](./web/js/nowplaying.js) and
  [web/js/app.js](./web/js/app.js) with that max while preserving higher live
  server-reported values.
- Hardened queue row action safety in
  [web/js/queue.js](./web/js/queue.js) by treating missing playlist IDs as
  non-removable/non-movable (instead of coercing to `0`), while keeping play
  fallback by position.
- Normalized library search route parsing in
  [web/js/router.js](./web/js/router.js) by trimming `q` so whitespace-only
  queries fall back to the library default route.
- Hardened settings update polling in
  [web/js/settings.js](./web/js/settings.js) to prevent overlapping `/status`
  poll requests when a previous poll is still in flight.
