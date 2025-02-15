
param (
  [string]$projectName,
  [string]$file
)

# 设定源文件路径
$sourceFile = "D:\a\$projectName\$projectName\$file"

# 定义目标目录数组
$targetDirArr = @("lib\x64", "lib\x86", "atlmfc\lib\x64", "atlmfc\lib\x86")

# 定义目标根目录
$targetRootDir = "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\"

# 检查目标根目录是否存在
if (-not (Test-Path -Path $targetRootDir -PathType Container)) {
  Write-Error "target root $targetRootDir does not exist, please check the path."
  return
}

# 获取目标根目录下的所有子目录
$subDirectories = Get-ChildItem -Path $targetRootDir -Directory

foreach ($subDir in $subDirectories) {
  # 构建可能的 lib 目录路径
  foreach ($targetDir in $targetDirArr) {
    $libDir = Join-Path -Path $subDir.FullName -ChildPath $targetDir
    if (Test-Path -Path $libDir -PathType Container) {
      try {
        # 复制文件到 lib 目录
        Copy-Item -Path $sourceFile -Destination $libDir -Force -ErrorAction Stop
        Write-Host "Copy-Item $sourceFile to $libDir success"
      }
      catch {
        Write-Warning "Copy-Item to $libDir error: $($_.Exception.Message)"
      } 
    }
  }
    
}