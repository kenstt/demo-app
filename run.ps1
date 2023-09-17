Write-Host "輸入以下選項："
Write-Host "1) [core]: 在命令列試玩井字遊戲"
$opt = Read-Host "："

if ($opt -eq 1) {
     cargo watch -c -q -w ./core/src -x 'run -p core --bin play'
} elseif ($opt -eq 2) {
    # 語法先寫著保留未來擴充使用
}