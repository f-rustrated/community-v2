use eventstore::Client;

use crate::config::config;

pub struct EventStoreRepository {
    pub(crate) client: Client,
}

impl EventStoreRepository {
    pub fn new() -> Self {
        let settings = config().eventstoredb_url.parse().unwrap();
        Self {
            client: Client::new(settings).unwrap(),
        }
    }
}