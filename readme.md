## Development remarks

### Prepare Docker environment

Change content of the .\docker\postgres.env.example file to match your environment and rename the file to .\docker\postgres.env

```pwsh
Copy-Item .\docker\postgres.env.example .\docker\postgres.env
```

### Start PostgreSQL container

```
docker compose --file .\docker\compose.yaml up -d
```
