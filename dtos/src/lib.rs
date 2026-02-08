// TODO: remove the allow(unused)

#[allow(unused)]
pub mod category;
#[allow(unused)]
pub mod image;
#[allow(unused)]
pub mod order;
#[allow(unused)]
pub mod product;
#[allow(unused)]
pub mod tag;
#[allow(unused)]
pub mod user;

// TODO: dtos
// file structure: <model>/<method>.rs

#[macro_export]
macro_rules! from_models {
    // `$entity` has to be an already imported module
    ($entity:ident, $type:ty, $vname:ident, $def:block) => {
        impl From<$entity::Model> for $type {
            fn from($vname: $entity::Model) -> Self {
                $def
            }
        }

        impl From<$entity::ModelEx> for $type {
            fn from($vname: $entity::ModelEx) -> Self {
                $def
            }
        }
    };
}

pub mod active_action {
    pub trait ActiveAction {
        #[must_use]
        fn creating(self) -> Self;
        #[must_use]
        fn modifying(self) -> Self;
    }

    #[macro_export]
    macro_rules! active_actions {
        ($am:path) => {
            impl $crate::active_action::ActiveAction for $am {
                fn creating(self) -> Self {
                    let now = sea_orm::sea_query::prelude::Utc::now().naive_local();
                    self.set_created_at(now).set_modified_at(now)
                }
                fn modifying(self) -> Self {
                    let now = sea_orm::sea_query::prelude::Utc::now().naive_local();
                    self.set_modified_at(now)
                }
            }
        };
    }
}
