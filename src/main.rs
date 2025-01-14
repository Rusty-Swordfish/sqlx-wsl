use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
mod models;
use models::User;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Load .env file
    
    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await?;

    // Insert a user
    // sqlx::query_as!(User,
    //     "INSERT INTO users (name, email) VALUES ($1, $2)",
    //     "Fred Flintstone",
    //     "fred@example.com"
    // )
    // .execute(&pool)
    // .await?;

    // Query users
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("User {}: {} ({})", user.id, user.name, user.email);
    }

    Ok(())
}
