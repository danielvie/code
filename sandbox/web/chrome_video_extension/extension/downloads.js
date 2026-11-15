import { mediaKind, safeFilename } from './media.js';

export function downloadOptions(video) {
  if (!video || mediaKind(video.url, video.mime) !== 'file') {
    throw new Error('Only direct video files can be downloaded. Play the lesson and scan again.');
  }
  return {
    url: video.url,
    filename: safeFilename(video.url),
    conflictAction: 'uniquify',
    saveAs: false
  };
}
