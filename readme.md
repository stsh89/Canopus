## Development remarks

### Create and store PostgreSQL database password

```pwsh
$pwd =  [guid]::NewGuid().ToString().Replace("-", "").Substring(0, 24)

Add-Content `
	-Path .\secrets\postgres-passwd `
	-Value $pwd
```
