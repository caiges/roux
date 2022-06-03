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

/// AfterState represents the various start, next and end states for the continuation of fetching paginated reddit resources.
pub enum AfterState {
    /// Start represents a starting point for the "after" value provided when fetching paginated reddit resources. It might be an existing resource name or None.
    Start(Option<String>),
    /// Next represents the "next" state for continuation. This would be in the middle of fetching paginated resources where we absolutely still have another "after" value to use.
    Next(String),
    /// End represents that we no longer have an after value and should not continue.
    End,
}