use sea_orm::*;

pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
    let db: DatabaseConnection = Database::connect("sqlite:test.db?mode=rwc").await?;
    Ok(db)
}
