pub trait RequestHandler<Req, Res> {
    fn handle(&self, req: Req);
}
