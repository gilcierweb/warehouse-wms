/// Macro that generates IRepository implementations for `BaseRepo`.
///
/// Usage:
/// ```
/// impl_crud!(User, NewUser, users::table);
/// ```
///
/// This generates implementations for: `all()`, `find()`, `create()`, `update()`, `destroy()`.
#[macro_export]
macro_rules! impl_crud {
    ($model:ty, $new_model:ty, $table:expr) => {
        #[async_trait::async_trait]
        impl crate::repositories::traits::IRepository<$model, $new_model>
            for crate::repositories::base::BaseRepo
        {
            async fn all(&self) -> diesel::QueryResult<Vec<$model>> {
                use diesel::RunQueryDsl;
                self.run(|conn| $table.load::<$model>(conn)).await
            }

            async fn find(&self, id: &uuid::Uuid) -> diesel::QueryResult<$model> {
                use diesel::{QueryDsl, RunQueryDsl};
                let id = *id;
                self.run(move |conn| $table.find(id).first::<$model>(conn))
                    .await
            }

            async fn create(&self, item: &$new_model) -> diesel::QueryResult<$model> {
                use diesel::RunQueryDsl;
                let item = item.clone();
                self.run(move |conn| diesel::insert_into($table).values(&item).get_result(conn))
                    .await
            }

            async fn update(
                &self,
                id: &uuid::Uuid,
                item: &$new_model,
            ) -> diesel::QueryResult<$model> {
                use diesel::{QueryDsl, RunQueryDsl};
                let item = item.clone();
                let id = *id;
                self.run(move |conn| diesel::update($table.find(id)).set(&item).get_result(conn))
                    .await
            }

            async fn destroy(&self, id: &uuid::Uuid) -> diesel::QueryResult<usize> {
                use diesel::{QueryDsl, RunQueryDsl};
                let id = *id;
                self.run(move |conn| diesel::delete($table.find(id)).execute(conn))
                    .await
            }
        }
    };
}

/// Bridges a domain repository trait to generic IRepository methods.
///
/// Usage:
/// `impl_repository_for_trait!(IProfileRepository, Profile, NewProfile);`
#[macro_export]
macro_rules! impl_repository_for_trait {
    ($trait_name:ident, $model:ty, $new_model:ty) => {
        #[async_trait::async_trait]
        impl<T> crate::repositories::traits::IRepository<$model, $new_model> for T
        where
            T: $trait_name + ?Sized,
        {
            async fn all(&self) -> diesel::QueryResult<Vec<$model>> {
                <T as $trait_name>::all(self).await
            }

            async fn find(&self, id: &uuid::Uuid) -> diesel::QueryResult<$model> {
                <T as $trait_name>::find(self, id).await
            }

            async fn create(&self, item: &$new_model) -> diesel::QueryResult<$model> {
                <T as $trait_name>::create(self, item).await
            }

            async fn update(
                &self,
                id: &uuid::Uuid,
                item: &$new_model,
            ) -> diesel::QueryResult<$model> {
                <T as $trait_name>::update(self, id, item).await
            }

            async fn destroy(&self, id: &uuid::Uuid) -> diesel::QueryResult<usize> {
                <T as $trait_name>::destroy(self, id).await
            }
        }
    };
}
