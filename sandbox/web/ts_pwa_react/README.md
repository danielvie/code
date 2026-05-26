# notes

install vite pwa
```powershell
npm install -D vite-plugin-pwa
```

- serve worker service will prefetch the assets to make them available offine



# usb debugging

1. Enable USB Debugging on Android:
- Go to Settings > Developer Options (enable this menu by tapping "Build Number" 7 times in "About Phone" if needed).
- Turn on USB Debugging.
- Connect your phone to your PC via USB.
Set up Port Forwarding (on PC):

2. Open Brave (or Chrome/Edge) on your PC.
- Go to brave://inspect/#devices (or chrome://inspect/#devices).
- Check the box "Port forwarding".
- Click "Configure...".
- Add a rule: Port 5173 maps to localhost:5173.
- Click Done.

3. Run the App:
- Restart your server: npm run dev
- On your Android phone, open Brave and go to http://localhost:5173.