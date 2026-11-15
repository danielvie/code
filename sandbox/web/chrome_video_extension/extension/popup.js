import { safeFilename } from './media.js';

const api = globalThis.browser ?? globalThis.chrome;
const $ = id => document.getElementById(id);

function status(text, error = false) {
  $('status').textContent = text;
  $('status').classList.toggle('error', error);
}

async function download(video, button) {
  button.disabled = true;
  button.setAttribute('aria-busy', 'true');
  try {
    const result = await api.runtime.sendMessage({ type: 'download', video });
    if (!result.ok) throw new Error(result.error);
    status(`Download started: ${safeFilename(video.url)}. Check your browser's Downloads panel for progress.`);
  } catch (error) { status(error.message, true); }
  finally {
    button.disabled = false;
    button.removeAttribute('aria-busy');
  }
}

function renderVideos(videos) {
  const labels = {
    stream: ['Segmented stream', 'HLS and DASH downloads are not supported.'],
    embed: ['Embedded player', 'Start playback and scan again to find the file.'],
    blob: ['Browser playback', 'Scan during playback to look for a direct file.']
  };
  $('videos').replaceChildren();
  for (const video of videos) {
    const item = document.createElement('li');
    const info = document.createElement('div');
    info.className = 'video-info';
    const title = document.createElement('span');
    title.className = 'video-title';
    const detail = document.createElement('span');
    detail.className = 'video-detail';
    if (video.kind === 'file') {
      const filename = safeFilename(video.url);
      title.textContent = filename;
      detail.textContent = new URL(video.url).hostname;
      const button = document.createElement('button');
      button.className = 'download-button';
      button.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false"><path d="M12 3v12m-5-5 5 5 5-5M4 16v4h16v-4"/></svg>';
      button.title = `Download ${filename}`;
      button.setAttribute('aria-label', `Download ${filename}`);
      button.addEventListener('click', () => download(video, button));
      item.append(info, button);
    } else {
      item.className = 'unsupported';
      [title.textContent, detail.textContent] = labels[video.kind];
      item.append(info);
    }
    info.append(title, detail);
    $('videos').append(item);
  }
}

$('scan').addEventListener('click', async () => {
  $('scan').disabled = true;
  $('scan').textContent = 'Scanning...';
  $('videos').replaceChildren();
  $('videos').setAttribute('aria-busy', 'true');
  status('Checking the page and its video players...');
  try {
    const [tab] = await api.tabs.query({ active: true, currentWindow: true });
    if (!tab?.id) throw new Error('No active lesson tab was found.');
    const result = await api.runtime.sendMessage({ type: 'scan', tabId: tab.id });
    if (!result.ok) throw new Error(result.error);
    const videos = result.videos.sort((a, b) => (a.kind !== 'file') - (b.kind !== 'file'));
    renderVideos(videos);
    const count = videos.filter(video => video.kind === 'file').length;
    const message = count ? `${count} video file${count === 1 ? '' : 's'} found. Choose a file to download.`
      : 'No direct files found. Play the video, wait a few seconds, then scan again.';
    status(message + (result.skippedFrames ? ` ${result.skippedFrames} frame(s) could not be inspected.` : ''));
  } catch (error) { status(error.message, true); }
  finally {
    $('scan').disabled = false;
    $('scan').textContent = 'Scan videos';
    $('videos').setAttribute('aria-busy', 'false');
  }
});
