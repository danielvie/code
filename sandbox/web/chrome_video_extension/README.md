# Lesson video saver

A compact Chrome and Firefox extension for the currently open Delligatti lesson. Click **Scan videos**, then the download icon beside a file. Files save to your browser's configured download folder using the filename in the video URL, such as `m02s14.mp4` or `27c0bc14-5f5f7fc9.mp4`. The lesson title is not used, and no new hash is calculated.

## Install or update in Chrome

Requires Chrome 121 or newer.

1. Open `chrome://extensions` and enable **Developer mode**.
2. Click **Load unpacked** and select `C:\SANDBOX\BOEING\STUDY\delligatti\extension`.
3. If already installed, click **Reload** on the extension's card instead.
4. Reload your lesson at https://learn.delligattiassociates.com/ and start playback.
5. Open **Lesson video saver**, click **Scan videos**, then **Download**.

No build or npm install is needed. Your browser's Downloads panel shows progress and errors. Downloads continue after the popup closes.

## Install in Firefox

Requires Firefox 140 or newer. The same `extension/` folder supports both browsers.

### Temporary installation, no signing required

1. Open `about:debugging#/runtime/this-firefox` in Firefox.
2. Click **Load Temporary Add-on...**.
3. Select `C:\SANDBOX\BOEING\STUDY\delligatti\extension\manifest.json`, or the generated `artifacts/lesson-video-saver-firefox-unsigned.xpi`.
4. Grant access to the requested lesson/player sites. If needed, review the extension's permissions in `about:addons`.
5. Open and reload a Delligatti lesson, start playback, then use **Scan videos** and the download icon.

Firefox removes temporary add-ons when it restarts. Use **Reload** beside the add-on in `about:debugging` after editing files.

### Permanent installation

The included XPI is **unsigned**. Standard Firefox will not accept it as a permanent add-on through `about:addons`.

Run `npm run package:firefox` to rebuild it, then submit the package to [Mozilla Add-ons](https://addons.mozilla.org/developers/) for signing. Choose self-distribution if you do not want a public listing. After Mozilla signs it, install the signed XPI using **about:addons > gear menu > Install Add-on From File...**. No package has been submitted or signed automatically.

## Layout and filenames

The popup is 560px wide, with the scan button beside the heading and compact file rows below. Longer lists scroll instead of making the popup taller. Each row shows the filename and video server, without the URL's signed query parameters.

The last nonempty URL path component supplies the filename. Percent-encoded names are decoded, Windows-invalid characters are replaced, and reserved names are made safe. Extensionless paths receive `.mp4`; a missing basename falls back to `video.mp4`. Existing video extension casing is preserved. Signed download URLs remain unchanged.

Existing files are not overwritten. The browser adds a suffix such as `m02s14 (1).mp4` when necessary.

The unsupported native folder picker and destination controls have been removed. Old extension folder preferences are ignored. To change the destination, use Chrome's **Settings > Downloads** or Firefox's **Settings > General > Files and Applications > Downloads**. The browser may still prompt if its setting to ask where to save files is enabled.

## Detection and limits

The scanner checks video/source elements, open shadow roots, permitted frames, exposed Vimeo progressive/download configuration, and relevant resource timing entries. It also observes successful media responses after installation. Up to 150 unique recent URLs per lesson tab stay in session storage and are cleared on navigation or tab closure.

Only direct video files are offered for download. HLS/DASH manifests, recognizable stream fragments, blob playback URLs, and player embeds are shown as unsupported or as playback hints. No segmented-stream merging, DRM decryption, login bypass, or course-wide crawling is included. Only save videos you have permission to download.

Signed links can expire. Play the video and scan again for a fresh link. Different qualities or signatures can appear as separate results. Servers requiring special headers may refuse browser-managed downloads. A misleading response or login redirect can produce a non-video file, so check your browser's Downloads panel and the saved file.

The authenticated Delligatti player has not been inspected directly. Detection is based on standard video elements, Vimeo, and the supplied screenshot showing an exposed `.mp4` filename. This does not guarantee every lesson exposes a downloadable file.

## Privacy and permissions

There is no telemetry or external service. Host access is limited to the learning site, Vimeo's player, and common Vimeo CDN domains. No optional all-site host permission or filesystem access is requested.

Permissions support current-tab and frame scanning, media response metadata, session storage, and browser downloads. Request cookies and bodies are not read. Observed URLs may contain temporary access tokens, so do not share them. No automatically injected content scripts are installed.

## Development

Plain JavaScript, Manifest V3, no runtime dependencies. Node.js 22 or newer is needed only for checks. Firefox packaging and native smoke checks additionally use Python 3's standard library.

| Command | Purpose |
| --- | --- |
| `npm run run` or `task run` | Print installation instructions and open Chrome's extension manager on Windows |
| `npm test` or `task test` | Focused source detection, filename and download-option tests |
| `npm run check` or `task check` | Check syntax, manifest and local asset references |
| `npm run smoke` or `task smoke` | Check the unpacked extension in disposable Chromium |
| `npm run smoke:firefox` or `task smoke-firefox` | Check native Firefox installation, messaging and downloads in a disposable profile |
| `npm run package:firefox` or `task package-firefox` | Build the unsigned Firefox XPI under `artifacts/` |
| `npm run clean` or `task clean` | Remove generated artifacts, never downloaded videos |
| `task` | List tasks |

The smoke script uses a locally installed Playwright Chromium executable without depending on its library. Otherwise set `CHROMIUM_PATH` to an extension-capable Chromium executable. It never uses your normal profile or login. Synthetic lessons and a local server exercise network/frame detection, displayed URL filenames, the scan/download buttons, default-folder downloads, ignored legacy folder settings, duplicate filenames, and compact popup dimensions. The temporary profile and downloaded fixtures are deleted; a screenshot and report are written under `artifacts/`.

The Firefox smoke uses native Firefox and its Marionette interface with a disposable profile, temporary add-on and loopback server. It verifies that the add-on is active, the popup communicates with its module background, unsupported-page scanning reports the expected error, and an actual browser download saves the original filename and fixture bytes. It does not inspect your normal profile or install an add-on permanently. The Firefox executable currently defaults to `C:\Program Files\Mozilla Firefox\firefox.exe` in this Windows-focused test tool.

Real authenticated lesson downloads, Firefox site-permission prompts, MAIN-world scanning on the learning site, and Firefox toolbar-panel presentation still require manual verification.

## Structure

- `extension/background.js`: media metadata observation, permitted-frame scanning and download handoff. Chrome uses its service worker; Firefox uses a module event page. Both use the same source and select `browser` or `chrome` APIs at runtime.
- `extension/scanner.js`: self-contained MAIN-world scanner for exposed player configuration and DOM sources.
- `extension/media.js` and `downloads.js`: media classification, original filename sanitization and Chrome download options.
- `extension/popup.*` and `styles.css`: compact scan-and-download interface.
- `extension/icons/`: selected gallery concept 09, "From this tab," with its SVG source and 16px, 32px, 48px and 128px PNGs for both browsers.
- `tests/` and `tools/`: focused checks and disposable-browser harness.

## API references

- [Chrome scripting and execution worlds](https://developer.chrome.com/docs/extensions/reference/api/scripting)
- [Chrome webRequest and host permissions](https://developer.chrome.com/docs/extensions/reference/api/webRequest)
- [Chrome downloads, relative filenames and conflict handling](https://developer.chrome.com/docs/extensions/reference/api/downloads#type-DownloadOptions)
- [MDN shared Chrome/Firefox background configuration](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/background)
- [MDN browser namespace and Promise APIs](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API)
- [Firefox extension ID and data collection metadata](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/browser_specific_settings)
- [Mozilla signing and self-distribution](https://extensionworkshop.com/documentation/publish/signing-and-distribution-overview/)
