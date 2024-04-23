use futures::{Future, FutureExt};
use hyper::Response;
use hyper::{
    client::connect::Connect, Body, Error, Request
};
use sealed::sealed;

#[sealed]
pub trait HttpClient: Send + Sync + 'static {
    fn _request(&self, req: Request<Body>) -> impl Future<Output = Result<Response<Body>, Error>>;
}

#[sealed]
impl<C> HttpClient for hyper::Client<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    fn _request(&self, req: Request<Body>) -> impl Future<Output = Result<Response<Body>, Error>> {
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
    fn _request(&self, mut req: Request<Body>) -> impl Future<Output = Result<Response<Body>, Error>> {
        (self.modify)(&mut req);
        dbg!(&req);
        self.client.request(req).inspect(|res| {dbg!(&res);})
    }
}
