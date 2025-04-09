Get-Content .env | ForEach-Object {
    $name, $value = $_ -split '='
    Set-Item -Path "Env:$name" -Value $value.Trim('"')
}
