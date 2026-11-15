import { resolve } from 'node:path';
import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';

console.log(`Load this folder as an unpacked extension:\n${resolve('extension')}`);
console.log('Chrome > chrome://extensions > Developer mode > Load unpacked.');
console.log('Then open a Delligatti lesson, start the video, and click Lesson video saver > Scan page.');
const chrome = process.env.CHROME_PATH || 'C:/Program Files/Google/Chrome/Application/chrome.exe';
if (existsSync(chrome)) {
  const child = spawn(chrome, ['chrome://extensions/'], { detached: true, stdio: 'ignore' });
  child.on('error', error => console.error(`Open chrome://extensions manually: ${error.message}`));
  child.unref();
}
