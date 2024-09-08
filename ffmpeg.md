command windows
```powershell
ffmpeg -i input_video.mp4 -b 1500k -vcodec wmv2 -acodec wmav2 -crf 19 -filter:v fps=fps=30 output.wmv
```
