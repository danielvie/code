with cuda
```powershell
ffmpeg -hwaccel cuda -i input.mp4 -c:v hevc_nvenc -b:v 3000k -c:a mp3 output_3000k.mp4
```

cut time frame
```powershell
ffmpeg -i input.mp4 -ss 00:31:40 -to 00:33:00 -c:v copy -c:a copy -map 0:v -map 0:a -map 0:5 output.mp4
```

```powershell
ffmpeg -ss 00:00:46 -i .\input.mp4 -to 00:05:00 -c copy output.mp4
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

join multiple files

1. create a text file (e.g `filelist.txt`):
```powershell
file 'video1.mp4'
file 'video2.mp4'
file 'video3.mp4'
```

2. run ffmpeg command:
```powershell
ffmpeg -f concat -safe 0 -i filelist.txt -c copy output.mp4
```
