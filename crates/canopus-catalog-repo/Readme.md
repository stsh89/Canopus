# Catalog repository

## Create database

```PowerShell
sqlx database create --database-url $env:CANOPUS_CATALOG_DATABASE_URL
```

## Migrate database

```PowerShell
sqlx migrate run --database-url $env:CANOPUS_CATALOG_DATABASE_URL
```

## Prepare SQLX artifacts

```PowerShell
cargo sqlx prepare --database-url $env:CANOPUS_CATALOG_DATABASE_URL
```
