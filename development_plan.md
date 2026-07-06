# Krynon: Database & Feature Evolution Roadmap

## Phase 3: First Release (PostgreSQL / Dedicated DB)
- **Library**: `sqlx` with the `postgres` and `runtime-tokio-native-tls` features. `sqlx` provides compile-time query checking which keeps Rust type-safety intact.
- **Database URL**: Use the `DATABASE_URL` environment variable for connection pooling.
- **Migrations**: Use embedded migrations located in the `./migrations` directory.

### 1. Transition Path from SQLite to PostgreSQL
- **Query Compatibility**: Since `sqlx` query syntax is extremely similar between SQLite and Postgres, the transition will primarily involve swapping the crate features from `sqlite` to `postgres` in `Cargo.toml`.
- **Database URL**: Migrate database connection pool instantiation to read from the `DATABASE_URL` environment variable.
- **Type Conversions**: Update Rust mappings where necessary (e.g., SQLite `TEXT` columns holding JSON will map to Postgres `JSONB` native types, allowing index-optimized nested queries).

### 2. High-Priority Features for First Release
