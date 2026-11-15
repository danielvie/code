import test from 'node:test';
import assert from 'node:assert/strict';
import { runInNewContext } from 'node:vm';
import { mediaKind, normalizeCandidates, safeFilename } from '../extension/media.js';
import { inspectFrame } from '../extension/scanner.js';

test('recognizes direct videos, streams, MIME-only videos, embeds and blobs', () => {
  for (const extension of ['mp4', 'webm', 'mov', 'm4v', 'ogg', 'ogv']) {
    assert.equal(mediaKind(`https://cdn.example/lesson.${extension}?signed=1`), 'file');
  }
  assert.equal(mediaKind('https://cdn.example/lesson.MP4#play'), 'file');
  assert.equal(mediaKind('https://cdn.example/play', 'video/mp4; codecs="avc1"'), 'file');
  assert.equal(mediaKind('https://cdn.example/play', 'VIDEO/WEBM'), 'file');
  assert.equal(mediaKind('https://cdn.example/master.m3u8?token=secret'), 'stream');
  assert.equal(mediaKind('https://cdn.example/manifest.mpd'), 'stream');
  assert.equal(mediaKind('https://cdn.example/play', 'application/vnd.apple.mpegurl'), 'stream');
  assert.equal(mediaKind('https://cdn.example/play', 'video/x-mpegURL'), 'stream');
  assert.equal(mediaKind('https://cdn.example/play.mp4', 'application/dash+xml'), 'stream');
  assert.equal(mediaKind('https://player.vimeo.com/video/123?h=private'), 'embed');
  assert.equal(mediaKind('blob:https://learn.delligattiassociates.com/123'), 'blob');
});

test('rejects executable schemes, unrelated URLs and deceptive media suffixes', () => {
  for (const url of [
    'javascript:alert(1)', 'data:video/mp4;base64,abc', 'file:///movie.mp4',
    'ftp://cdn.example/movie.mp4', '/relative.mp4', 'not a URL',
    'https://cdn.example/player.js', 'https://cdn.example/styles.css?file=movie.mp4',
    'https://cdn.example/movie.mp4.js', 'https://player.vimeo.com.evil.test/video/123',
    'https://player.vimeo.com/api/player.js', 'https://cdn.example/audio.mp3',
  ]) assert.equal(mediaKind(url), null, url);
  assert.equal(mediaKind('javascript:alert(1)', 'video/mp4'), null);
});

test('normalization deduplicates exact URLs without altering signed queries', () => {
  const signed = 'https://cdn.example/movie.mp4?token=a%2Fb+%3D&expires=123&x=1&x=2';
  const other = signed.replace('expires=123', 'expires=124');
  const actual = normalizeCandidates([
    { url: signed },
    { url: signed, title: 'Lesson one', mime: 'video/mp4' },
    { url: other, title: 'Other signature' },
    { url: 'https://cdn.example/player.js' },
    { url: 'javascript:alert(1)', mime: 'video/mp4' },
    { url: 'https://cdn.example/play', mime: 'video/webm' },
    null,
  ]);
  assert.equal(actual.length, 3);
  assert.deepEqual(actual[0], { url: signed, title: 'Lesson one', kind: 'file', mime: 'video/mp4' });
  assert.equal(actual[1].url, other);
  assert.equal(actual[2].kind, 'file');
});

test('filenames preserve URL basenames and extension casing without signed queries', () => {
  assert.equal(safeFilename('https://cdn.example/m02s14.mp4?token=a%2Fb&expires=123'), 'm02s14.mp4');
  assert.equal(safeFilename('https://cdn.example/27c0bc14-5f5f7fc9.mp4'), '27c0bc14-5f5f7fc9.mp4');
  for (const extension of ['MP4', 'WebM', 'MOV', 'm4V', 'OGG', 'OgV']) {
    assert.equal(safeFilename(`https://cdn.example/Lesson%20one.${extension}?token=x#play`), `Lesson one.${extension}`);
  }
  assert.equal(safeFilename('https://cdn.example/opaque'), 'opaque.mp4');
  assert.equal(safeFilename('https://cdn.example/Lesson%20.MP4'), 'Lesson .MP4');
  assert.equal(safeFilename('https://cdn.example/lesson.txt'), 'lesson.txt.mp4');
  assert.equal(safeFilename('https://cdn.example/folder/movie.mp4///'), 'movie.mp4');
  assert.equal(safeFilename(`https://cdn.example/${'a'.repeat(200)}.MP4`), `${'a'.repeat(200)}.MP4`);
  assert.ok(safeFilename(`https://cdn.example/${'a'.repeat(300)}.MP4`).length <= 255);
});

test('filenames remove Windows-invalid characters and reserved basenames', () => {
  const unsafe = 'Lesson: "one" / two\\three? <four>|*\u0000\u001f\u007f';
  assert.equal(safeFilename(`https://cdn.example/${encodeURIComponent(unsafe)}.webm`),
    'Lesson_ _one_ _ two_three_ _four______.webm');
  for (const name of ['CON', 'prn.mp4', 'AUX', 'NUL', 'COM1', 'LPT9', 'COM¹', 'LPT³', 'con.hidden', 'CON .mp4']) {
    assert.ok(safeFilename(`https://cdn.example/${encodeURIComponent(name)}`).startsWith('_'), name);
  }
  assert.equal(safeFilename('https://cdn.example/%20..%20'), 'video.mp4');
  assert.equal(safeFilename('https://cdn.example/movie.MP4.%20'), 'movie.MP4');
});

test('filenames decode safely, prevent paths and use safe fallbacks', () => {
  assert.equal(safeFilename('https://cdn.example/%2F..%5Csecret.webm'), '_.._secret.webm');
  assert.equal(safeFilename('https://cdn.example/..%2F..%5Csecret.mp4'), '_.._secret.mp4');
  assert.equal(safeFilename('https://cdn.example/bad%ZZ.mp4'), 'bad%ZZ.mp4');
  assert.equal(safeFilename('https://cdn.example/bad%E0%A4.mp4'), 'bad%E0%A4.mp4');
  for (const url of ['not a URL', '/relative.mp4', 'https://cdn.example', 'https://cdn.example///?token=movie.mp4',
    'https://cdn.example/%2e%2e', 'https://cdn.example/%20%20']) {
    assert.equal(safeFilename(url), 'video.mp4', url);
  }
});

function element(tagName, properties = {}, attributes = {}) {
  return {
    tagName: tagName.toUpperCase(),
    getAttribute(name) { return attributes[name] ?? null; },
    querySelectorAll() { return []; },
    ...properties,
  };
}

function scan(nodes, windowOverrides = {}, title = 'Current lesson') {
  const document = {
    title,
    baseURI: 'https://learn.delligattiassociates.com/lessons/one',
    querySelectorAll() { return nodes; },
  };
  const window = { performance: { getEntriesByType() { return []; } }, ...windowOverrides };
  // Match executeScript's serialization boundary: no module scope is available.
  return runInNewContext(`(${inspectFrame.toString()})()`, { document, window, URL });
}

test('serialized scanner finds video sources, downloads, Vimeo and open shadow roots', () => {
  const signed = 'https://cdn.example/video.mp4?signature=a%2Fb&expires=123';
  const source = element('source', { src: signed, type: 'video/mp4' });
  const shadowVideo = element('video', { currentSrc: 'blob:https://cdn.example/id', src: '' });
  const nodes = [
    element('video', { currentSrc: signed, src: '/original.webm', querySelectorAll() { return [source]; } }),
    element('audio', { currentSrc: 'https://cdn.example/audio.ogg' }),
    element('iframe', { src: 'https://player.vimeo.com/video/123?h=abc' }),
    element('iframe', { src: 'https://unrelated.example/frame' }),
    element('a', { href: 'https://cdn.example/file.pdf' }, { download: 'notes.pdf' }),
    element('a', { href: 'https://cdn.example/download', textContent: 'Download lesson' }, { download: 'lesson.mp4' }),
    element('a', { href: 'https://cdn.example/playlist' }, { download: 'lesson.m3u8' }),
    element('a', { href: 'javascript:alert(1)' }, { download: 'lesson.mp4' }),
    element('div', { shadowRoot: { querySelectorAll() { return [shadowVideo]; } } }),
  ];
  const result = normalizeCandidates(scan(nodes));
  assert.deepEqual(result.map(item => item.url), [signed, 'https://learn.delligattiassociates.com/original.webm',
    'https://player.vimeo.com/video/123?h=abc', 'https://cdn.example/download', 'https://cdn.example/playlist',
    'blob:https://cdn.example/id']);
  assert.equal(result[0].title, 'Current lesson');
  assert.equal(result.find(item => item.url.endsWith('/download')).mime, 'video/mp4');
  assert.equal(result.find(item => item.url.endsWith('/playlist')).kind, 'stream');
});

test('scanner reads loaded Vimeo progressive/download config and only media performance entries', () => {
  const progressive = 'https://vimeocdn.example/play?token=abc%2Bdef&expires=123';
  const downloaded = 'https://vimeocdn.example/download?signature=xyz';
  const result = normalizeCandidates(scan([], {
    playerConfig: { request: { files: { progressive: [{ url: progressive, mime: 'video/mp4' }],
      hls: { cdns: { private: { url: 'https://private.example/not-read' } } } } } },
    vimeo: { config: { download: [{ link: downloaded, type: 'video/mp4' }] } },
    performance: { getEntriesByType(type) {
      assert.equal(type, 'resource');
      return [
        { name: 'https://cdn.example/movie.webm?token=signed' },
        { name: 'https://cdn.example/master.m3u8' },
        { name: 'https://cdn.example/player.js?movie=foo.mp4' },
        { name: 'https://cdn.example/poster.jpg' },
      ];
    } },
  }));
  assert.deepEqual(result.map(item => item.url), [progressive, downloaded,
    'https://cdn.example/movie.webm?token=signed', 'https://cdn.example/master.m3u8']);
  assert.equal(result.at(-1).kind, 'stream');
});

test('scanner parses JSON config without executing scripts or traversing unrelated globals', () => {
  const config = { request: { files: { progressive: [{ url: 'https://cdn.example/json.mp4' }] } } };
  const nodes = [
    element('script', { textContent: JSON.stringify({ playerConfig: config }) }),
    element('script', { textContent: `window.playerConfig = ${JSON.stringify(config)};` }),
    element('script', { textContent: 'window.executed = true; /* progressive */' }),
  ];
  const window = { unrelated: { request: { files: { progressive: [{ url: 'https://cdn.example/unrelated.mp4' }] } } } };
  const result = normalizeCandidates(scan(nodes, window));
  assert.equal(result.length, 1);
  assert.equal(result[0].url, 'https://cdn.example/json.mp4');
  assert.equal(window.executed, undefined);
});

test('scanner uses video title when document title is absent', () => {
  const result = normalizeCandidates(scan([
    element('video', { src: 'https://cdn.example/video.mp4' }, { title: 'Video title' }),
  ], {}, ''));
  assert.equal(result[0].title, 'Video title');
});
