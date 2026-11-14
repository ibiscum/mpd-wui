// Runtime configuration for the mpd-wui frontend.
// All endpoints are same-origin; no external hosts are ever contacted.
// The display name (wordmark + tab title) is injected by the server into the
// `app-title` meta tag; it is configurable and defaults to "MPD: <host>".
const appTitle =
  document.querySelector('meta[name="app-title"]')?.content || "mpd-wui";
const volumeMax = Number.parseInt(document.querySelector('meta[name="volume-max"]')?.content || "", 10);

export const config = {
  apiBase: "/api",
  appTitle,
  volumeMax: Number.isFinite(volumeMax) && volumeMax > 0 ? volumeMax : 100,
  wsUrl: (proto, host) =>
    `${proto === "https:" ? "wss" : "ws"}://${host}/ws`,
};
