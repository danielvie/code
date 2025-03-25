with cuda
```powershell
ffmpeg -hwaccel cuda -i input.mp4 -c:v hevc_nvenc -b:v 1000k -c:a mp3 output.mp4
```


reduce bitrate
```powershell
ffmpeg -i input.mp4 -vcodec libx264 -b:v 1000k -acodec mp3 output.mp4
```

change resolution
```powershell
ffmpeg -i input.mp4 -vf scale=1280:720 -vcodec libx264 -acodec mp3 output.mp4
```

use H.265
```powershell
ffmpeg -i input.mp4 -vcodec libx265 -crf 28 -acodec mp3 output.mp4
```

combine options
```powershell
ffmpeg -i input.mp4 -vcodec libx265 -vf scale=854:480 -b:v 500k -acodec mp3 output.mp4
```

command windows
```powershell
ffmpeg -i input_video.mp4 -b 1500k -vcodec wmv2 -acodec wmav2 -crf 19 -filter:v fps=fps=30 output.wmv
```
