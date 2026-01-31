// TODO:
// user
// tag
// cetegory
// image
// product
// order(/transaction, not sure which name is appropriate yet)
//
// M-N
// product - tag
// product - category
// product - image
// product - order

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
