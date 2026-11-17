use std::error::Error;

use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn database_is_reachable_and_migratable() -> Result<(), Box<dyn Error>> {
    let Ok(database_url) = std::env::var("TEST_DATABASE_URL") else {
        return Ok(());
    };

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let one: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;
    assert_eq!(one, 1);

    Ok(())
}
