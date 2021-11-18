//     std::env::set_var("DATABASE_URL", "sqlite:data.db");
//     let sqlite = SqlitePool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
//     log::info!(target:  "app::backend::db",  "[INFO] Connected to SQLite DB");
//     log::info!(target:  "app::backend::db",  "[INFO] URL: {}", &env::var("DATABASE_URL").unwrap());

    
//     let mut db_connection = sqlite.acquire().await.unwrap();
//     sqlx::query!(
//         r#"
// CREATE TABLE IF NOT EXISTS users
// (
//     id          INTEGER PRIMARY KEY NOT NULL,
//     name        TEXT                NOT NULL,
//     pwd         BOOLEAN             NOT NULL DEFAULT 0
// );
// "#
//     ).execute(&mut db_connection)
//     .await.unwrap();