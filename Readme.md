## Development remarks

### Prepare Docker environment

Rename the file .\docker\postgres.env.example to .\docker\postgres.env and update the content if needed.

```pwsh
Copy-Item .\docker\postgres.env.example .\docker\postgres.env
```

### Prepare development environment

Rename the file .\.env.example to .\.env and update the content if needed.

```pwsh
Copy-Item .\.env.example .\.env
```

### Start and prepare PostgreSQL container

```pwsh
docker compose --file .\docker\compose.yaml up -d
```

```pwsh
sqlx database create
```

```pwsh
sqlx migrate run
```
