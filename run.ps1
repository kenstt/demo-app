Write-Host "輸入以下選項："
Write-Host "1) [core]: 在命令列試玩井字遊戲"
Write-Host "2) [core]: 跑單元測試"
$opt = Read-Host "："

if ($opt -eq 1) {
     cargo watch -c -q -w ./core/src -x 'run -p core --bin play'
} elseif ($opt -eq 2) {
    cargo watch -q -c -w ./core -x 'test -p core'
}