// Chrome serializes this function for MAIN-world execution. Keep every helper inside it.
export function inspectFrame() {
  const results = [];
  const pageTitle = document.title?.trim() || '';
  const mediaPath = /\.(mp4|webm|mov|m4v|ogg|ogv|m3u8|mpd)$/i;
  const base = document.baseURI || window.location.href;

  function parseUrl(value) {
    if (typeof value !== 'string' || !value.trim()) return null;
    try {
      const url = new URL(value.trim(), base);
      return ['http:', 'https:', 'blob:'].includes(url.protocol) ? url : null;
    } catch {
      return null;
    }
  }

  function add(value, mime = '', title = '') {
    const parsed = parseUrl(value);
    if (!parsed) return;
    // Do not rewrite signed absolute URLs, including their query strings.
    const url = /^(https?:|blob:)/i.test(value.trim()) ? value.trim() : parsed.href;
    results.push({ url, title: pageTitle || title || 'Video', mime });
  }

  function isMedia(value) {
    const parsed = parseUrl(value);
    return parsed && mediaPath.test(parsed.pathname);
  }

  function readConfig(config) {
    if (!config || typeof config !== 'object') return;
    const title = config.video?.title || '';
    const files = config.request?.files;
    const groups = [files?.progressive, files?.download, config.download, config.video?.download];
    for (const entries of groups) {
      if (!Array.isArray(entries)) continue;
      for (const entry of entries) {
        if (!entry || typeof entry !== 'object') continue;
        add(entry.url || entry.link || entry.download_url, entry.mime || entry.type || 'video/mp4', title);
      }
    }
  }

  function readConfigContainer(value) {
    readConfig(value);
    readConfig(value?.playerConfig);
    readConfig(value?.config);
    readConfig(value?.player?.config);
  }

  function readScript(script) {
    const text = script.textContent?.trim();
    if (!text || !/progressive|download/.test(text)) return;
    // Parse JSON only. Never evaluate page scripts or reconstruct private requests.
    const assignment = text.match(/^(?:(?:var|let|const)\s+)?(?:(?:window\.)?playerConfig|(?:window\.)?config|window\.__PLAYER_CONFIG__)\s*=\s*(\{[\s\S]*\})\s*;?$/);
    try {
      readConfigContainer(JSON.parse(assignment ? assignment[1] : text));
    } catch {
      // Most inline scripts are executable JavaScript, not JSON config.
    }
  }

  const roots = [document];
  while (roots.length) {
    const root = roots.pop();
    for (const node of root.querySelectorAll('*')) {
      if (node.shadowRoot) roots.push(node.shadowRoot);
      const tag = node.tagName?.toLowerCase();
      if (tag === 'video') {
        const title = node.getAttribute('title') || '';
        const sources = [...node.querySelectorAll('source')];
        const currentType = sources.find(source => source.src === node.currentSrc)?.type || node.getAttribute('type') || '';
        add(node.currentSrc, currentType, title);
        add(node.src, node.getAttribute('type') || '', title);
        for (const source of sources) add(source.src, source.type || '', title);
      } else if (tag === 'iframe') {
        const url = parseUrl(node.src);
        if (url?.hostname === 'player.vimeo.com' && /^\/video\/\d+(?:\/|$)/.test(url.pathname)) {
          add(node.src, '', node.title);
        }
      } else if (tag === 'a') {
        const mime = node.getAttribute('type') || '';
        const download = node.getAttribute('download') || '';
        if (isMedia(node.href) || /^video\//i.test(mime) || mediaPath.test(download)) {
          let downloadMime = '';
          if (mediaPath.test(download) && !isMedia(node.href)) {
            downloadMime = /\.m3u8$/i.test(download) ? 'application/vnd.apple.mpegurl'
              : /\.mpd$/i.test(download) ? 'application/dash+xml' : 'video/mp4';
          }
          add(node.href, mime || downloadMime, node.textContent?.trim());
        }
      } else if (tag === 'script') {
        readScript(node);
      }
    }
  }

  readConfigContainer(window.playerConfig);
  readConfigContainer(window.vimeo?.config);
  readConfigContainer(window.__PLAYER_CONFIG__);
  for (const entry of window.performance?.getEntriesByType('resource') || []) {
    if (isMedia(entry.name)) add(entry.name);
  }
  return results;
}
