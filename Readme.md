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

### Upgrade PostgreSQL version

Create database dump:

```pwsh
docker compose --file .\docker\compose.yaml exec db `
sh -c 'pg_dump -U $POSTGRES_USER --quote-all-identifiers --format=custom canopus_dev' `
> ".\pg_dumps\canopus_dev_$(Get-Date -Format 'yyyyMMddHHmmss').bak"
```

Remove db_data volume (ALERT, following command will remove all volumes):

```pwsh
docker compose --file .\docker\compose.yaml down --volumes
```

Update compose.yaml with newer PostgreSQL version.

Start PostgreSQL container and create database:

```pwsh
docker compose --file .\docker\compose.yaml up -d
```

```pwsh
sqlx database create
```

Restore database dump (NOTICE, set your file timestamp):

```pwsh
Get-Content .\pg_dumps\canopus_dev_{TIMESTAMP}.bak -AsByteStream | `
docker compose --file .\docker\compose.yaml exec -T db `
sh -c 'pg_restore -U $POSTGRES_USER -d canopus_dev'
```
