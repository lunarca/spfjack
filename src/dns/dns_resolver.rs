use trust_dns_resolver::{
    TokioAsyncResolver,
    TokioHandle,
};



pub fn new_resolver() -> TokioAsyncResolver {
    TokioAsyncResolver::from_system_conf(TokioHandle).unwrap()
}