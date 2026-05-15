docker cleanup


```powershell
docker system prune -a --volumes
```

how to reduce the size of the vhdx file

locate the VHDX file: typically at:
%LOCALAPPDATA%\Docker\wsl\data\ext4.vhdx 

```powershell
wsl --shutdown

diskpart
select vdisk file="C:\Users\YourName\AppData\Local\Docker\wsl\data\ext4.vhdx"
attach vdisk readonly
compact vdisk
detach vdisk
exit
```

