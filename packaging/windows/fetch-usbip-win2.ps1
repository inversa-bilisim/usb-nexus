# SPDX-License-Identifier: GPL-3.0-or-later
# Downloads the usbip-win2 installer that the USB Nexus setup bundles and
# offers to install. The file is checked against the SHA-256 digest GitHub
# records for the release asset.
#
#   pwsh packaging/windows/fetch-usbip-win2.ps1 [-Version 0.9.8.0]
#
# Set GH_TOKEN (or GITHUB_TOKEN) to avoid the anonymous API rate limit.

param(
    [string]$Version = '0.9.8.0',
    [string]$Arch = 'x64'
)

$ErrorActionPreference = 'Stop'
$dest = Join-Path $PSScriptRoot 'usbip-win2\usbip-win2-setup.exe'

$headers = @{ 'User-Agent' = 'usbnexus-build'; 'Accept' = 'application/vnd.github+json' }
$token = if ($env:GH_TOKEN) { $env:GH_TOKEN } else { $env:GITHUB_TOKEN }
if ($token) { $headers['Authorization'] = "Bearer $token" }

$release = Invoke-RestMethod -Headers $headers "https://api.github.com/repos/vadimgrn/usbip-win2/releases/tags/v.$Version"
$assets = @($release.assets | Where-Object { $_.name -match "-$Arch-release\.exe$" })
if ($assets.Count -ne 1) {
    throw "expected one $Arch installer in usbip-win2 $Version, found: $($release.assets.name -join ', ')"
}
$asset = $assets[0]
if (-not $asset.digest -or -not $asset.digest.StartsWith('sha256:')) {
    throw "GitHub has no SHA-256 digest for $($asset.name)"
}
$expected = $asset.digest.Substring(7).ToUpperInvariant()

Invoke-WebRequest -Headers @{ 'User-Agent' = 'usbnexus-build' } -OutFile $dest $asset.browser_download_url
$actual = (Get-FileHash -Algorithm SHA256 $dest).Hash
if ($actual -ne $expected) {
    Remove-Item $dest
    throw "checksum mismatch for $($asset.name): expected $expected, got $actual"
}
Write-Host "$($asset.name) -> $dest (sha256 $actual)"
