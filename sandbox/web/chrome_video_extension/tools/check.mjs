import { readFile, readdir, access } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';

const manifest = JSON.parse(await readFile('extension/manifest.json', 'utf8'));
if (manifest.manifest_version !== 3) throw new Error('Expected Manifest V3.');
if (!manifest.background.scripts?.includes(manifest.background.service_worker) ||
    manifest.background.type !== 'module' || !manifest.browser_specific_settings?.gecko?.id) {
  throw new Error('Expected shared Chrome/Firefox module backgrounds and a stable Firefox ID.');
}
for (const file of new Set([manifest.background.service_worker, ...manifest.background.scripts, manifest.action.default_popup])) {
  await access(`extension/${file}`);
}
for (const icons of [manifest.icons, manifest.action.default_icon]) {
  for (const [size, file] of Object.entries(icons || {})) {
    const png = await readFile(`extension/${file}`);
    if (png.toString('hex', 0, 8) !== '89504e470d0a1a0a' ||
        png.readUInt32BE(16) !== Number(size) || png.readUInt32BE(20) !== Number(size)) {
      throw new Error(`Expected a ${size}x${size} PNG: ${file}`);
    }
  }
}
for (const file of await readdir('extension')) {
  if (file.endsWith('.js')) {
    const result = spawnSync(process.execPath, ['--check', `extension/${file}`], { encoding: 'utf8' });
    if (result.status !== 0) throw new Error(result.stderr);
  }
  if (file.endsWith('.html')) {
    const html = await readFile(`extension/${file}`, 'utf8');
    for (const match of html.matchAll(/(?:src|href)="([^"#:]+)"/g)) await access(`extension/${match[1]}`);
  }
}
console.log('Manifest references, local assets, and extension JavaScript syntax passed.');
