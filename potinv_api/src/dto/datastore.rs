use super::types::*;
use std::collections::HashMap;
use minimal_id::MinimalId;

#[derive(Debug, Default)]
pub(crate) struct Datastore {
    pub(crate) clays: HashMap<MinimalId, Clay>
}
