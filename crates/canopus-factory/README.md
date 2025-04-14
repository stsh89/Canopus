# Canopus factory

## Start service

```pwsh
cargo run -- -p 3001 --database-url $env:CANOPUS_FACTORY_DATABASE_URL
```

## Sample requests

### Create brand

```pwsh
Invoke-RestMethod 127.0.0.1:3001/brands -Method Post -Body '{"name": "Mikrotik"}'
```

### List brands

```pwsh
Invoke-RestMethod 127.0.0.1:3001/brands
```

### Delete brand

```pwsh
Invoke-RestMethod 127.0.0.1:3001/brands/$id -Method Delete -Body
```

## Create database

```pwsh
sqlx database create --database-url $env:CANOPUS_FACTORY_DATABASE_URL
```

## Delete database

```pwsh
sqlx database drop --database-url $env:CANOPUS_FACTORY_DATABASE_URL
```

## Migrate database

```pwsh
sqlx migrate run --database-url $env:CANOPUS_FACTORY_DATABASE_URL
```

## Revert last migration

```pwsh
sqlx migrate revert --database-url $env:CANOPUS_FACTORY_DATABASE_URL
```

## Prepare SQLX artifacts

```pwsh
cargo sqlx prepare --database-url $env:CANOPUS_FACTORY_DATABASE_URL
```

## Connect to database

```pwsh
 docker exec -it canopus_db psql $env:CANOPUS_FACTORY_DATABASE_URL
```
