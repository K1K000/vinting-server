pub mod filter;

use sea_orm::{
    Condition, DatabaseConnection, DbErr, EntityTrait, PrimaryKeyTrait, QueryFilter, SelectExt,
    prelude::async_trait::async_trait,
};

/// trait for getting tables via service
///
/// functions that should be provided:
///  - `default_filters` (returns the filters the queries should run with by default)
///  - `get_backing_db` (returns the db the queries should be run with)
#[async_trait]
pub trait ServiceTrait<T>
where
    T: EntityTrait,
{
    fn default_filters() -> Condition;
    fn get_backing_db(&self) -> &DatabaseConnection;

    async fn exists_by_id<U>(&self, id: U) -> Result<bool, DbErr>
    where
        U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        self.exists_by_id_raw(id, Some(Self::default_filters()))
            .await
    }

    async fn exists_by_id_raw<U>(&self, id: U, filter: Option<Condition>) -> Result<bool, DbErr>
    where
        U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        let mut q = T::find_by_id(id);

        if let Some(filter) = filter {
            q = q.filter(filter);
        }
        q.exists(self.get_backing_db()).await
    }

    async fn get_by_id<U>(&self, id: U) -> Result<Option<T::Model>, DbErr>
    where
        U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        self.get_by_id_raw(id, Some(Self::default_filters())).await
    }

    async fn get_by_id_raw<U>(
        &self,
        id: U,
        filter: Option<Condition>,
    ) -> Result<Option<T::Model>, DbErr>
    where
        U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        let mut q = T::find_by_id(id);

        if let Some(filter) = filter {
            q = q.filter(filter);
        }

        q.one(self.get_backing_db()).await
    }

    async fn get_all(&self) -> Result<Vec<T::Model>, DbErr> {
        self.get_all_raw(Some(Self::default_filters())).await
    }
    async fn get_all_raw(&self, filter: Option<Condition>) -> Result<Vec<T::Model>, DbErr> {
        let mut q = T::find();

        if let Some(filter) = filter {
            q = q.filter(filter);
        }

        q.all(self.get_backing_db()).await
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, Database, DatabaseConnection,
        DbErr, sea_query::prelude::Utc,
    };
    use std::ops::Not;

    use crate::service::ServiceTrait;
    use crate::tag;

    /// Uses tag as the test entity because it's small
    /// In real services use `&DatabaseConnection` instead of `DatabaseConnection` directly
    struct TestService(DatabaseConnection);

    impl ServiceTrait<tag::Entity> for TestService {
        fn default_filters() -> Condition {
            Condition::all().add(tag::Column::DeletedAt.is_null())
        }
        fn get_backing_db(&self) -> &DatabaseConnection {
            &self.0
        }
    }

    fn mock_data() -> Vec<tag::ActiveModel> {
        let now = Utc::now().naive_local();
        vec![
            tag::ActiveModel {
                id: Set(1),
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(None),
                name: Set("Test1".to_string()),
            },
            tag::ActiveModel {
                id: Set(2),
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(None),
                name: Set("Test2".to_string()),
            },
            tag::ActiveModel {
                id: Set(3),
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(Some(now)),
                name: Set("Test2".to_string()),
            },
        ]
    }

    async fn setup_db() -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect("sqlite::memory:").await?;
        db.get_schema_registry("entity::*").sync(&db).await?;
        Ok(db)
    }

    async fn setup_mock(db: &DatabaseConnection) -> Result<(), DbErr> {
        for i in mock_data() {
            i.insert(db).await?;
        }

        Ok(())
    }

    async fn setup_service() -> Result<TestService, DbErr> {
        let db = setup_db().await?;

        setup_mock(&db).await?;

        Ok(TestService(db))
    }

    // base for service based tests, do take out the lint allow
    #[allow(unused_variables)]
    #[tokio::test]
    async fn service_test_base() -> Result<(), DbErr> {
        let service = setup_service().await?;

        Ok(())
    }

    #[tokio::test]
    async fn exists() -> Result<(), DbErr> {
        let service = setup_service().await?;

        assert!(service.exists_by_id(1).await?);
        assert!(service.exists_by_id(3).await?.not());

        Ok(())
    }

    #[tokio::test]
    async fn exists_raw() -> Result<(), DbErr> {
        let service = setup_service().await?;

        assert!(service.exists_by_id_raw(1, None).await?);
        assert!(service.exists_by_id_raw(3, None).await?);
        assert!(service.exists_by_id_raw(5, None).await?.not());

        Ok(())
    }

    #[tokio::test]
    async fn get_by_id() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let tag1 = service.get_by_id(1).await?;
        assert!(tag1.is_some());
        let tag1 = tag1.expect("already asserted that it exists");

        assert!(tag1.id == 1);
        assert!(tag1.name == "Test1");

        Ok(())
    }

    #[tokio::test]
    async fn get_by_id_raw() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let tag1 = service.get_by_id_raw(3, None).await?;
        assert!(tag1.is_some());
        let tag1 = tag1.expect("already asserted that it exists");

        assert!(tag1.id == 3);
        assert!(tag1.deleted_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn get_all() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let all = service.get_all().await?;
        assert!(all.len() == 2);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_raw() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let raw_all = service.get_all_raw(None).await?;
        assert!(raw_all.len() == 3);

        Ok(())
    }
}
