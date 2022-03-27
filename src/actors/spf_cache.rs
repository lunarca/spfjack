use actix::prelude::*;
use decon_spf::Spf;
use std::{collections::HashMap, sync::Arc};

type InsertCacheResponse = ();
type QueryCacheResponse = Option<Arc<Spf>>;

// Message to insert an item to the cache
pub struct InsertCacheMessage {
    pub domain: String,
    pub value: Arc<Spf>,
}
impl Message for InsertCacheMessage {
    type Result = InsertCacheResponse;
}

// Message to extract an item from the cache
pub struct QueryCacheMessage {
    pub domain: String,
}
impl Message for QueryCacheMessage {
    type Result = QueryCacheResponse;
}

pub struct SpfCacheActor {
    cache: HashMap<String, Arc<Spf>>
}

impl Actor for SpfCacheActor {
    type Context = Context<Self>;
}

impl Handler<InsertCacheMessage> for SpfCacheActor {
    type Result = InsertCacheResponse;

    fn handle(&mut self, msg: InsertCacheMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.cache.insert(msg.domain, msg.value);
    }
}

impl Handler<QueryCacheMessage> for SpfCacheActor {
    type Result = QueryCacheResponse;
    
    fn handle(&mut self, msg: QueryCacheMessage, _ctx: &mut Context<Self>) -> Self::Result {
        match self.cache.get(&msg.domain) {
            Some(spf_ref) => Some(Arc::clone(spf_ref)),
            None => None,
        }
    }
}

pub fn start_link() -> Addr<SpfCacheActor> {
    return SpfCacheActor { cache: HashMap::new()}.start();
}

