@echo off
setlocal
set "TLAPS_EXPERIMENT=%~dp0"
set "PATH=%TLAPS_EXPERIMENT%bundle\bin;%TLAPS_EXPERIMENT%bundle\lib\tlaps\bin;%TLAPS_EXPERIMENT%runtime\bin;%SystemRoot%\System32;%SystemRoot%"
set "HOME=%TLAPS_EXPERIMENT%scratch\home"
set "USERPROFILE=%TLAPS_EXPERIMENT%scratch\userprofile"
set "APPDATA=%TLAPS_EXPERIMENT%scratch\appdata"
set "LOCALAPPDATA=%TLAPS_EXPERIMENT%scratch\localappdata"
set "TEMP=%TLAPS_EXPERIMENT%scratch\temp"
set "TMP=%TLAPS_EXPERIMENT%scratch\tmp"
for %%D in ("%HOME%" "%USERPROFILE%" "%APPDATA%" "%LOCALAPPDATA%" "%TEMP%" "%TMP%" "%TLAPS_EXPERIMENT%runtime\tmp") do if not exist "%%~D" mkdir "%%~D"
"%TLAPS_EXPERIMENT%bundle\bin\tlapm.exe" %*
exit /b %ERRORLEVEL%
