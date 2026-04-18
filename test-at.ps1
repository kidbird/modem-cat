$port = New-Object System.IO.Ports.SerialPort('COM18',115200,'None','8','One')
$port.Open()
Start-Sleep -Seconds 3
$port.WriteLine('AT')
Start-Sleep -Seconds 3
$response = $port.ReadExisting()
if ($response -match 'OK') {
    Write-Output 'SUCCESS'
    Write-Output "Response: $response"
} else {
    Write-Output 'FAILED'
    Write-Output "Response: $response"
}
$port.Close()