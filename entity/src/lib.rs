pub mod prelude;

pub mod category;
pub mod image;
pub mod order;
pub mod product;
pub mod product_category;
pub mod product_image;
pub mod product_order;
pub mod product_tag;
pub mod tag;
pub mod user;

/// Anything to do with `ServiceTrait`
pub mod service;

#[cfg(test)]
mod test {
    use sea_orm::{Database, DbErr};

    #[tokio::test]
    async fn can_make_db() -> Result<(), DbErr> {
        let db = Database::connect("sqlite::memory:").await?;
        db.get_schema_registry("entity::*").sync(&db).await?;

        Ok(())
    }
}
