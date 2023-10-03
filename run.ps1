Write-Host "輸入以下選項："
Write-Host "1) [core]: 在命令列試玩井字遊戲"
Write-Host "2) [core]: 跑單元測試"
Write-Host "3) [web]: 執行 WebApi Server"
Write-Host "4) [service]: 執行 Service 測試"
Write-Host "5) [web]: 執行 跑單元測試"
Write-Host "6) [app]: 執行 tauri 前端 UI"
Write-Host "7) [web]: 執行 WebApi Server: HTTPS"
Write-Host "8) [wasm]: 建置 wasm"
Write-Host "9) [web]: 執行gRPC Server"
$opt = Read-Host "："

if ($opt -eq 1) {
     cargo watch -c -q -w ./core/src -x 'run -p core --bin play'
} elseif ($opt -eq 2) {
    cargo watch -q -c -w ./core -x 'test -p core'
} elseif ($opt -eq 3) {
    cargo watch -q -c -w ./web -w ./service -w ./core -x 'run -p web --bin web'
} elseif ($opt -eq 4) {
    cargo watch -q -c -w ./service -w ./core -x 'test -p service'
} elseif ($opt -eq 5) {
    cargo watch -q -c -w ./web -w ./service -w ./core -x 'test -p web'
} elseif ($opt -eq 6) {
    cargo tauri dev -- -p app
} elseif ($opt -eq 7) {
    cargo watch -q -c -w ./web -w ./service -w ./core -x 'run -p web --bin https'
} elseif ($opt -eq 8) {
    cargo watch -q -c -w ./wasm/ -w ./service/ -w ./core/ -x 'build -p wasm && wasm-pack build --target web ./wasm'
} elseif ($opt -eq 9) {
    cargo watch -q -c -w ./web/ -w ./service/ -w ./core/ -x 'run -p web --bin grpc'
}