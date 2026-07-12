#[cfg(feature = "server")]
pub mod server {
    use sqlx::PgPool;
    use tokio::sync::OnceCell;

    static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

    /// Retrieves a reference to the global database connection pool,
    /// initializing it and running migrations if it hasn't been initialized yet.
    pub async fn get_pool() -> &'static PgPool {
        DB_POOL
            .get_or_init(|| async {
                // Load the .env file if it exists
                dotenvy::dotenv().ok();

                let database_url = std::env::var("DATABASE_URL")
                    .expect("DATABASE_URL environment variable must be set");

                // Connect to PostgreSQL and build connection pool
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .expect("Failed to connect to PostgreSQL");

                // Run embedded migrations automatically. sqlx::migrate! records a
                // SHA-256 checksum of each migration file the first time it runs.
                // If a migration's content changes after it has been applied, the
                // next start will panic with `VersionMismatch`. The recovery is to
                // drop the database volume and rebuild so all migrations re-run
                // against a clean schema.
                if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
                    match &err {
                        sqlx::migrate::MigrateError::VersionMismatch(version) => {
                            panic!(
                                "Failed to run database migrations: VersionMismatch({version}).\n\
                                 A migration file was modified after it had already been\n\
                                 applied. To recover, drop the database volume and rebuild:\n\
                                 \n\
                                 \x20   docker compose -f docker-compose.dev.yaml down -v\n\
                                 \x20   docker compose -f docker-compose.dev.yaml up --watch --build\n\
                                 \n\
                                 (For the production compose, use `docker compose down -v`\n\
                                 \x20  then `docker compose up -d --build`.)"
                            );
                        }
                        other => panic!("Failed to run database migrations: {other}"),
                    }
                }

                println!("Successfully connected to PostgreSQL database and executed migrations.");
                pool
            })
            .await
    }
}
