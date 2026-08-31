@echo off
setlocal EnableExtensions
chcp 65001 >nul

cd /d "%~dp0"

set "TargetDir=.build-target"
set "Out=opencode-info.sdPlugin"
set "Assets=assets"
set "Target=x86_64-pc-windows-msvc"

rem allow override: build-windows.bat gn  -> GNU target
if /i "%~1"=="gn" set "Target=x86_64-pc-windows-gnu"

echo ==^> Building (release, %Target%)^...
set "CARGO_TARGET_DIR=%TargetDir%"
call cargo build --release --target %Target% || goto :err

echo ==^> Assembling plugin directory from assets^...
if exist "%Out%" rmdir /S /Q "%Out%"
xcopy /E /I /Y /Q "%Assets%" "%Out%" >nul || goto :err

echo ==^> Copying binaries^...
copy /Y "%TargetDir%\%Target%\release\opencode-info.exe" "%Out%\opencode-info-%Target%.exe" >nul || goto :err
copy /Y "%TargetDir%\%Target%\release\opencode-info.exe" "%Out%\opencode-info.exe" >nul || goto :err

echo ==^> Packaging streamDeckPlugin^...
if exist "opencode-info.streamDeckPlugin" del /Q "opencode-info.streamDeckPlugin"
if exist "opencode-info.zip" del /Q "opencode-info.zip"

set "Tmp=%TEMP%\ocinfo-%RANDOM%"
mkdir "%Tmp%" 2>nul
xcopy /E /I /Y /Q "%Out%" "%Tmp%\opencode-info.sdPlugin" >nul || goto :err
powershell -NoProfile -Command "Compress-Archive -Path '%Tmp%\opencode-info.sdPlugin' -DestinationPath '%~dp0opencode-info.zip' -CompressionLevel Optimal" || goto :err
rmdir /S /Q "%Tmp%"

rem .streamDeckPlugin is a zip with a different extension
move /Y "opencode-info.zip" "opencode-info.streamDeckPlugin" >nul || goto :err

echo ==^> Done: %CD%\opencode-info.streamDeckPlugin
echo     Binary: %Out%\opencode-info-%Target%.exe
exit /b 0

:err
echo [ERROR] Build failed.
exit /b 1
