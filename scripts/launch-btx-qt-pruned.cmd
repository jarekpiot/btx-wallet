@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "BTX_QT=%SCRIPT_DIR%..\bin\btx-qt.exe"

if not exist "%BTX_QT%" (
  set "BTX_QT=%SCRIPT_DIR%btx-qt.exe"
)

if not exist "%BTX_QT%" (
  echo error: btx-qt.exe not found next to the release launcher 1>&2
  exit /b 1
)

"%BTX_QT%" -prune=4096 -pruneduringinit=4096 -retainshieldedcommitmentindex=1 -listen=0 -natpmp=0 -upnp=0 %*
