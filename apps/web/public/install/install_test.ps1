#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

# Test that we can install the latest version at the default location.
Remove-Item "~\.osmon" -Recurse -Force -ErrorAction SilentlyContinue
$env:OSMON_INSTALL = ""
$v = $null; .\install.ps1
~\.osmon\bin\osmon.exe --version

# Test that we can install a specific version at a custom location.
Remove-Item "~\osmon-0.4.0" -Recurse -Force -ErrorAction SilentlyContinue
$env:OSMON_INSTALL = "$Home\osmon-0.4.0"
$v = "v0.4.0"; .\install.ps1
$OsmonVersion = ~\osmon-0.4.0\bin\osmon.exe --version
if (!($OsmonVersion -like '*0.4.0*')) {
  throw $OsmonVersion
}

# Test that we can install at a relative custom location.
Remove-Item "bin" -Recurse -Force -ErrorAction SilentlyContinue
$env:OSMON_INSTALL = "."
$v = "v0.4.0"; .\install.ps1
$OsmonVersion = bin\osmon.exe --version
if (!($OsmonVersion -like '*0.4.0*')) {
  throw $OsmonVersion
}

# Test that the old temp file installer still works.
Remove-Item "~\osmon-0.4.0" -Recurse -Force -ErrorAction SilentlyContinue
$env:OSMON_INSTALL = "$Home\osmon-0.4.0"
$v = $null; .\install.ps1 v0.4.0
$OsmonVersion = ~\osmon-0.4.0\bin\osmon.exe --version
if (!($OsmonVersion -like '*0.4.0*')) {
  throw $OsmonVersion
}
