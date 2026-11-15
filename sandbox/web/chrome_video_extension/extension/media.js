const FILE_EXTENSION = /\.(mp4|webm|mov|m4v|ogg|ogv)$/i;
const STREAM_EXTENSION = /\.(m3u8|mpd)$/i;

export function mediaKind(url, mime = '') {
  let parsed;
  try {
    parsed = new URL(url);
  } catch {
    return null;
  }
  if (parsed.protocol === 'blob:') return 'blob';
  if (!['http:', 'https:'].includes(parsed.protocol)) return null;

  const type = String(mime).split(';')[0].trim().toLowerCase();
  if (type.includes('mpegurl') || type === 'application/dash+xml' || type === 'video/mp2t') return 'stream';
  if (STREAM_EXTENSION.test(parsed.pathname) || /\.(ts|m4s|cmfv|cmfa)$/i.test(parsed.pathname)) return 'stream';
  if (type.startsWith('video/') || type === 'application/mp4' || FILE_EXTENSION.test(parsed.pathname)) return 'file';
  if (parsed.hostname === 'player.vimeo.com' && /^\/video\/\d+(?:\/|$)/.test(parsed.pathname)) {
    return 'embed';
  }
  return null;
}

export function normalizeCandidates(rawArray) {
  const candidates = new Map();
  for (const raw of rawArray ?? []) {
    if (!raw || typeof raw.url !== 'string') continue;
    const url = raw.url.trim();
    const mime = typeof raw.mime === 'string' ? raw.mime.trim() : '';
    const kind = mediaKind(url, mime);
    if (!kind) continue;
    const title = typeof raw.title === 'string' ? raw.title.trim() : '';
    const previous = candidates.get(url);
    if (previous) {
      if (!previous.mime && mime) {
        previous.mime = mime;
        previous.kind = kind;
      }
      if (previous.title === 'Video' && title) previous.title = title;
    } else {
      candidates.set(url, { url, title: title || 'Video', kind, mime });
    }
  }
  return [...candidates.values()];
}

export function safeFilename(url) {
  let name = '';
  try {
    name = new URL(url).pathname.split('/').filter(Boolean).pop() || '';
    try {
      name = decodeURIComponent(name);
    } catch {
      // A malformed escape should not prevent saving the file.
    }
  } catch {
    return 'video.mp4';
  }
  name = name.replace(/[<>:"/\\|?*\u0000-\u001f\u007f]/g, '_')
    .replace(/^[. ]+|[. ]+$/g, '');
  if (!name) return 'video.mp4';
  const extension = name.match(FILE_EXTENSION)?.[1] || 'mp4';
  let stem = name.replace(FILE_EXTENSION, '');
  if (!stem) stem = 'video';
  if (/^(con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³]) *(?:\.|$)/i.test(stem)) stem = `_${stem}`;
  stem = stem.slice(0, 255 - extension.length - 1);
  return `${stem}.${extension}`;
}
