use super::request::RequestHandler;

pub trait SyncMediatorInternal<Ev> {
    fn publish(&self, event: Ev);
    fn send<Req>(&self, req: Req) where Self: RequestHandler<Req, Ev>;
}