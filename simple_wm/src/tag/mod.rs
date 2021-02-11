mod tag;

use crate::client::Client;
use std::sync::Arc;

pub struct Tag {
    pub windows: Vec<Arc<Client>>,
}
