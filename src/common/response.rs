use serde::{Deserialize, Serialize};
use crate::utils::Pagination;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}