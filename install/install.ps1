#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

if ($v) {
  $Version = "v${v}"
}
if ($args.Length -eq 1) {
  $Version = $args.Get(0)
}

$OsmonInstall = $env:OSMON_INSTALL
$BinDir = if ($OsmonInstall) {
  "$OsmonInstall\bin"
} else {
  "$Home\.osmon\bin"
}

$OsmonZip = "$BinDir\osmon.zip"
$OsmonExe = "$BinDir\osmon.exe"
$Target = 'x86_64-pc-windows-msvc'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$OsmonUri = if (!$Version) {
  "https://github.com/osmon-lang/osmon/releases/latest/download/osmon-${Target}.zip"
} else {
  "https://github.com/osmon-lang/osmon/releases/download/${Version}/osmon-${Target}.zip"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $OsmonUri -OutFile $OsmonZip -UseBasicParsing

if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
  Expand-Archive $OsmonZip -Destination $BinDir -Force
} else {
  if (Test-Path $OsmonExe) {
    Remove-Item $OsmonExe
  }
  Add-Type -AssemblyName System.IO.Compression.FileSystem
  [IO.Compression.ZipFile]::ExtractToDirectory($OsmonZip, $BinDir)
}

Remove-Item $OsmonZip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Output "Osmon $OsmonExe da muvaffaqiyatli o'rnatildi!"
Write-Output "Ko'proq ma'lumot uchun 'osmon --help' buyruq satrini ishga tushuring"
