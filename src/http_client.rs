use hyper::{
    client::{connect::Connect, ResponseFuture},
    Body, Request,
};
use sealed::sealed;

#[sealed]
pub trait HttpClient: Send + Sync + 'static {
    fn _request(&self, req: Request<Body>) -> ResponseFuture;
}

#[sealed]
impl<C> HttpClient for hyper::Client<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    fn _request(&self, req: Request<Body>) -> ResponseFuture {
        self.request(req)
    }
}

pub struct HyperClientModifyRequest<C, F> {
    client: hyper::Client<C>,
    modify: F,
}

impl<C, F> HyperClientModifyRequest<C, F>
where
    C: Connect + Clone + Send + Sync + 'static,
    F: Fn(&mut Request<Body>) + Send + Sync + 'static,
{
    pub fn new(client: hyper::Client<C>, modify: F) -> Self {
        Self { client, modify }
    }
}

#[sealed]
impl<C, F> HttpClient for HyperClientModifyRequest<C, F>
where
    C: Connect + Clone + Send + Sync + 'static,
    F: Fn(&mut Request<Body>) + Send + Sync + 'static,
{
    fn _request(&self, mut req: Request<Body>) -> ResponseFuture {
        (self.modify)(&mut req);
        self.client.request(req)
    }
}
