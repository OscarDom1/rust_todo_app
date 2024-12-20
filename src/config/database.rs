use mysql::Pool;
use dotenv::dotenv;
use std::env;

pub async fn get_db_pool() -> Result<Pool, mysql::Error> {
    dotenv().ok();  // Load environment variables

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the database connection pool
    let pool = Pool::new(database_url)?;
    
    Ok(pool)
}
