use std::sync::mpsc::{channel, Receiver, RecvError, Sender, TryRecvError};

pub struct DefaultMediator<Ev> {
    channel: (Sender<Ev>, Receiver<Ev>),
    listener: Vec<Box<dyn Fn(&Ev) -> ()>>,
}

pub struct Builder<Ev> {
    mediator: DefaultMediator<Ev>,
}

impl<Ev> Builder<Ev> {
    pub fn add_listener<F>(mut self, f: F) -> Self
    where
        F: Fn(&Ev) -> () + 'static,
    {
        self.mediator.add_listener(f);
        self
    }

    pub fn build(self) -> DefaultMediator<Ev> {
        self.mediator
    }
}

impl<Ev> Default for DefaultMediator<Ev> {
    fn default() -> Self {
        Self {
            channel: channel(),
            listener: Default::default(),
        }
    }
}

impl<Ev> DefaultMediator<Ev> {
    pub fn builder() -> Builder<Ev> {
        Builder::<Ev> {
            mediator: Default::default(),
        }
    }

    pub fn publish(&self, event: Ev) {
        self.channel.0.send(event).ok();
    }

    pub fn send<Req>(&self, req: Req)
    where
        Self: RequestHandler<Req, Ev>,
    {
        <Self as RequestHandler<Req, Ev>>::handle(self, req)
    }

    pub fn listen<F>(&self, f: F) -> Result<(), RecvError>
    where
        F: Fn(Ev) -> (),
    {
        let res = self.channel.1.recv();
        match res {
            Ok(event) => Ok(f(event)),
            Err(e) => Err(e),
        }
    }

    pub fn try_listen<F>(&self, f: F) -> Result<(), TryRecvError>
    where
        F: Fn(Ev) -> (),
    {
        let res = self.channel.1.try_recv();
        match res {
            Ok(event) => Ok(f(event)),
            Err(e) => Err(e),
        }
    }

    pub fn run_block(&self) {
        for event in self.channel.1.iter() {
            for f in self.listener.iter() {
                f(&event)
            }
        }
    }

    fn add_listener<F>(&mut self, f: F)
    where
        F: Fn(&Ev) -> () + 'static,
    {
        self.listener.push(Box::new(f));
    }
}

pub trait RequestHandler<Req, Res> {
    fn handle(&self, req: Req);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum AuthEventType {
        Login,
        Logout,
    }

    #[derive(Debug)]
    struct Login {
        username: String,
        password: String,
    }

    #[derive(Debug)]
    struct AuthEvent(Login, AuthEventType);

    struct LoginRequest(Login);

    impl RequestHandler<LoginRequest, AuthEvent> for DefaultMediator<AuthEvent> {
        fn handle(&self, req: LoginRequest) {
            if req.0.password == String::from("crocodile13") {
                self.publish(AuthEvent(req.0, AuthEventType::Login))
            }
        }
    }

    #[test]
    fn it_works() {
        let mediator = DefaultMediator::<AuthEvent>::builder()
            .add_listener(move |ev| println!("my listened event: {:?}", ev))
            .build();
        mediator.send::<LoginRequest>(LoginRequest(Login {
            username: String::from("nyvs"),
            password: String::from("crocodile13"),
        }));
        mediator.run_block();
        mediator
            .try_listen(move |ev| println!("my event: {:?}", ev))
            .ok();
    }
}
