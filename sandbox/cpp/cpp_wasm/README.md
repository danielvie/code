
Get the emsdk repo
```powershell
git clone https://github.com/emscripten-core/emsdk.git
```

Enter that directory
```powershell
cd emsdk
```

Fetch the latest version of the emsdk (not needed the first time you clone)
```powershell
git pull
```

Download and install the latest SDK tools.
```powershell
./emsdk install latest
```

Make the "latest" SDK "active" for the current user. (writes .emscripten file)
```powershell
./emsdk activate latest
```

Activate PATH and other environment variables in the current terminal
```powershell
source ./emsdk_env.sh
```