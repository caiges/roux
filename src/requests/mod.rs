//! # Requests
//! Base requests

use crate::responses::BasicListing;
use serde::Deserialize;

/// Many endpoints use a common set of parameters to control things like pagination.
/// Note, only `after` or `before` should be provided but not both since they represent an anchor point of the request.
#[derive(Deserialize, Debug)]
pub struct PaginationOptions {
    /// Maximum number of items to return.
    pub limit: Option<i32>,
    /// The fullname of an item in the listing to use as the anchor point of the request.
    pub after: Option<String>,
    /// The fullname of an item in the listing to use as the anchor point of the request.
    pub before: Option<String>,
}

/// Pagination trait used when making requests.
pub trait Paginate<T> {
    /// take
    fn take(&self, options: PaginationOptions) -> BasicListing<T>;
}
