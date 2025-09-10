Add-Type -AssemblyName System.Windows.Forms

$rootpath = $PSScriptRoot
$rust = '.rs'
$dst_folder = Join-Path -Path (Split-Path -Path $rootpath -Parent) -ChildPath 'rust'

## 如果資料夾不存在跳出訊息, 避免亂創建檔案
if (-not (Test-Path -LiteralPath $dst_folder)){
    [void][System.Windows.Forms.MessageBox]::show(
        "沒有 $dst_folder 這個資料夾",
        "訊息",
        [System.Windows.Forms.MessageBoxButtons]::OK,
        [System.Windows.Forms.MessageBoxIcon]::Information
    ) 
    exit 1
}

##list::new()
$created = New-Object System.Collections.Generic.List[string]

Get-ChildItem -Path $rootpath -File -Filter *.py | ForEach-Object {
    $name = [IO.Path]::GetFileNameWithoutExtension($_.Name)
    $target = Join-Path -Path $dst_folder -ChildPath ($name + '.rs')
    if (-not (Test-Path -LiteralPath $target)) {
        ni -path $target -ItemType File | Out-Null
        'fn main(){}' | Set-Content -NoNewline -Path $target
        [void]$created.add($target)
    }
}

if ($created.Count -gt 0){
    $names = $created | ForEach-Object {[IO.Path]::GetFileName($_)}

    $msg = "已創建$($created.Count)個檔案:`r`n$names"
    [void][System.Windows.Forms.MessageBox]::show(
        $msg,
        "建立完成",
        [System.Windows.Forms.MessageBoxButtons]::OK,
        [System.Windows.Forms.MessageBoxIcon]::Information
    )
}