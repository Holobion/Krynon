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

                // Run embedded migrations automatically
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run database migrations");

                println!("Successfully connected to PostgreSQL database and executed migrations.");
                pool
            })
            .await
    }
}
