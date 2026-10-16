Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

$base = $PSScriptRoot

Write-Host "Bundling extension.js from extension.original.js..."
$entryFile = Join-Path $base "extension.original.js"
$outFile = Join-Path $base "extension.js"
$esbuildResult = npx esbuild $entryFile --bundle --platform=node --external:vscode "--outfile=$outFile"
if ($LASTEXITCODE -ne 0) {
    throw "Failed to bundle extension.js with esbuild: $esbuildResult"
}

$binDir = Join-Path $base "bin"
if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir -Force | Out-Null
}

$targetExe = Join-Path $binDir "haetae.exe"
$sourceExe = Join-Path $base "..\01_바이너리_실행파일\haetae.exe"
if ((Test-Path $sourceExe) -and (-not (Test-Path $targetExe))) {
    Copy-Item -Path $sourceExe -Destination $targetExe -Force
}

$targetLinux = Join-Path $binDir "haetae"
$sourceLinux = Join-Path $base "..\01_바이너리_실행파일\haetae-linux-arm64"
if ((Test-Path $sourceLinux) -and (-not (Test-Path $targetLinux))) {
    Copy-Item -Path $sourceLinux -Destination $targetLinux -Force
}

$zipFile = Join-Path $base "haetae-0.1.0.vsix"
$aliasFile = Join-Path $base "haetae.vsix"
if (Test-Path $zipFile) { Remove-Item -Force $zipFile }
if (Test-Path $aliasFile) { Remove-Item -Force $aliasFile }

$zip = [System.IO.Compression.ZipFile]::Open($zipFile, [System.IO.Compression.ZipArchiveMode]::Create)

$null = [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, (Join-Path $base "[Content_Types].xml"), "[Content_Types].xml")
$null = [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, (Join-Path $base "extension.vsixmanifest"), "extension.vsixmanifest")

$filesToAdd = @(
    "package.json",
    "extension.js",
    "config.js",
    "README.md",
    "LICENSE",
    "bin/haetae.exe",
    "bin/haetae",
    "keybindings/haetae-keybindings.json"
)
foreach ($f in $filesToAdd) {
    $fullPath = Join-Path $base $f
    if (Test-Path $fullPath) {
        $entryName = "extension/" + ($f -replace "\\", "/")
        $null = [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $fullPath, $entryName)
    }
}

$zip.Dispose()

Copy-Item -Force $zipFile $aliasFile

$binaryDistDir = Join-Path $base "..\01_바이너리_실행파일"
if (Test-Path $binaryDistDir) {
    Copy-Item -Force $zipFile (Join-Path $binaryDistDir "haetae-0.1.0.vsix")
    Copy-Item -Force $aliasFile (Join-Path $binaryDistDir "haetae.vsix")
}

Get-Item $zipFile, $aliasFile | Select-Object Name, Length, @{Name="SizeMB";Expression={[math]::Round($_.Length / 1MB, 2)}}, LastWriteTime
