$src = "dist\v0.2.4\portable\*"
$dst = "dist\v0.2.4\ModemCat_v0.2.4_portable.zip"
Compress-Archive -Path $src -DestinationPath $dst -Force
Write-Host "ZIP created:" $dst
