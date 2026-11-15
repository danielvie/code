import test from 'node:test';
import assert from 'node:assert/strict';
import { downloadOptions } from '../extension/downloads.js';
import { mediaKind } from '../extension/media.js';

test('Chrome download options preserve signed links, ignore titles and use the default folder', () => {
  const url = 'https://cdn.example/m02s14.mp4?token=a%2Fb+%3D&x=1&x=2';
  assert.deepEqual(downloadOptions({ url, title: 'Lesson: one' }), {
    url, filename: 'm02s14.mp4', conflictAction: 'uniquify', saveAs: false
  });
  const hashed = 'https://cdn.example/27c0bc14-5f5f7fc9.mp4?signature=secret';
  assert.equal(downloadOptions({ url: hashed, title: 'Different lesson' }).filename, '27c0bc14-5f5f7fc9.mp4');
});

test('MIME-only video filenames use the URL basename or a safe fallback', () => {
  assert.equal(downloadOptions({ url: 'https://cdn.example/play?token=x', mime: 'video/mp4' }).filename, 'play.mp4');
  assert.equal(downloadOptions({ url: 'https://cdn.example/?token=x', mime: 'video/mp4' }).filename, 'video.mp4');
  assert.equal(downloadOptions({ url: 'https://cdn.example/Lesson%20one.MP4' }).filename, 'Lesson one.MP4');
});

test('downloads reject nonfiles, streams, blobs and embeds', () => {
  for (const url of ['javascript:alert(1)', 'https://cdn.example/master.m3u8', 'https://cdn.example/manifest.mpd',
    'https://player.vimeo.com/video/123', 'blob:https://lesson.example/a', 'https://cdn.example/player.js']) {
    assert.throws(() => downloadOptions({ url }), /Only direct/, url);
  }
  assert.throws(() => downloadOptions(), /Only direct/);
  assert.throws(() => downloadOptions({}), /Only direct/);
  assert.throws(() => downloadOptions({ url: 'https://cdn.example/play.mp4', mime: 'application/dash+xml' }), /Only direct/);
});

test('stream fragments are never offered as standalone videos', () => {
  assert.equal(mediaKind('https://cdn.example/segment.ts', 'video/mp2t'), 'stream');
  assert.equal(mediaKind('https://cdn.example/segment.m4s', 'video/mp4'), 'stream');
  assert.equal(mediaKind('https://cdn.example/segment.cmfv'), 'stream');
  for (const video of [
    { url: 'https://cdn.example/segment.ts', mime: 'video/mp2t' },
    { url: 'https://cdn.example/segment.m4s', mime: 'video/mp4' },
    { url: 'https://cdn.example/segment.cmfv' },
    { url: 'https://cdn.example/segment.cmfa' },
  ]) assert.throws(() => downloadOptions(video), /Only direct/);
});
