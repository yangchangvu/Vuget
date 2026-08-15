# Build + kill + restart RedWidget
# Cách dùng: ./build-restart.ps1

$exe = "src-tauri\target\release\redwidget.exe"
$manifest = "src-tauri\Cargo.toml"

Write-Host "Building..." -ForegroundColor Cyan
cargo build --release --manifest-path $manifest
if ($LASTEXITCODE -ne 0) { Write-Host "Build failed" -ForegroundColor Red; exit 1 }

Write-Host "Killing old process..." -ForegroundColor Yellow
Get-Process -Name redwidget -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 500

Write-Host "Launching..." -ForegroundColor Green
Start-Process $exe
Write-Host "Done. RedWidget restarted." -ForegroundColor Green
