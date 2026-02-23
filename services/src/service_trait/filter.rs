use sea_orm::QueryFilter;

use super::ServiceTrait;

// 1 Service ideally should only be able to service for 1 type
pub trait ServiceFilter
where
    Self: QueryFilter,
{
    // if you're writing something beyond the scope of the Service,
    // but still want to have the default filters defined in the Service
    #[must_use]
    fn service_filter<S>(self) -> Self
    where
        S: ServiceTrait + ?Sized,
    {
        self.filter(S::default_filters())
    }
}

impl<Q: QueryFilter> ServiceFilter for Q {}
