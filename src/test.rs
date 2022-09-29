use super::prelude::*;

#[derive(Debug, Clone)]
enum AuthEventType {
    Login,
}

#[derive(Debug, Clone)]
struct Login {
    token: String,
}

#[derive(Debug, Clone)]
struct AuthEvent(Login, AuthEventType);

struct LoginRequest(Login);

#[cfg(not(feature = "async"))]
#[test]
fn email_example_sync() {
    struct UserMessageRequest {
        msg: String,
        priority: u8,
    }

    #[derive(Debug, Clone)]
    enum NotifyEvent {
        Ignore,
        SendEmail(String),
        SendTextMessage(String),
    }

    impl RequestHandler<UserMessageRequest, NotifyEvent> for BasicMediator<NotifyEvent> {
        fn handle(&self, req: UserMessageRequest) {
            match req.priority {
                0 => self.publish(NotifyEvent::Ignore),
                1..=5 => self.publish(NotifyEvent::SendEmail(req.msg)),
                _ => self.publish(NotifyEvent::SendTextMessage(req.msg)),
            };
        }
    }

    let mediator = BasicMediator::<NotifyEvent>::builder()
        .add_listener(move |ev| {
            if let NotifyEvent::Ignore = ev {
                println!("Ignored some Message")
            }
        })
        .add_listener(move |ev| {
            if let NotifyEvent::SendEmail(msg) = ev {
                println!("Send Email with Message: {}", msg)
            }
        })
        .add_listener(move |ev| {
            if let NotifyEvent::SendTextMessage(msg) = ev {
                println!("Send SMS with Message: {}", msg)
            }
        })
        .build();

    mediator.send(UserMessageRequest {
        msg: String::from("Hello World"),
        priority: 0,
    });

    mediator.send(UserMessageRequest {
        msg: String::from("Is Rust Memory Safe?"),
        priority: 2,
    });

    mediator.send(UserMessageRequest {
        msg: String::from("New Rust Version"),
        priority: 8,
    });

    mediator.next().ok();
    mediator.next().ok();
    mediator.next().ok();
}

#[cfg(not(feature = "async"))]
#[test]
fn it_works_sync() {
    impl RequestHandler<LoginRequest, AuthEvent> for BasicMediator<AuthEvent> {
        fn handle(&self, req: LoginRequest) {
            if req.0.token == String::from("xyz") {
                self.publish(AuthEvent(req.0, AuthEventType::Login))
            }
        }
    }

    let mediator = BasicMediator::<AuthEvent>::builder()
        .add_listener(move |ev| println!("my listened event: {:?}", ev))
        .build();

    mediator.send::<LoginRequest>(LoginRequest(Login {
        token: String::from("xyz"),
    }));

    mediator.next().ok();
}

#[cfg(not(feature = "async"))]
#[test]
fn atomic_test_sync() {
    use std::sync::{Arc, Mutex};

    struct IncrementRequest;
    #[derive(Debug, Clone)]
    struct IncrementEvent;

    impl RequestHandler<IncrementRequest, IncrementEvent> for BasicMediator<IncrementEvent> {
        fn handle(&self, _req: IncrementRequest) {
            self.publish(IncrementEvent)
        }
    }

    let u = Arc::new(Mutex::new(0usize));
    let cloned = u.clone();
    let mediator = BasicMediator::<IncrementEvent>::builder()
        .add_listener(move |_| {
            let mut m = cloned.lock().unwrap();
            let c = *m;
            *m = c + 1;
        })
        .build();

    mediator.send(IncrementRequest);

    mediator.next().ok();
    assert_eq!(*(u.lock().unwrap()), 1usize);

    mediator.send(IncrementRequest);
    mediator.send(IncrementRequest);

    mediator.next().ok();
    mediator.next().ok();

    assert_eq!(*(u.lock().unwrap()), 3usize);
}

#[cfg(feature = "async")]
#[test]
fn email_example_async() {
    use async_trait::async_trait;

    struct UserMessageRequest {
        msg: String,
        priority: u8,
    }

    #[derive(Debug, Clone)]
    enum NotifyEvent {
        Ignore,
        SendEmail(String),
        SendTextMessage(String),
    }

    #[async_trait]
    impl AsyncRequestHandler<UserMessageRequest, NotifyEvent> for BasicAsyncMediator<NotifyEvent> {
        async fn handle(&self, req: UserMessageRequest) {
            match req.priority {
                0 => self.publish(NotifyEvent::Ignore).await,
                1..=5 => self.publish(NotifyEvent::SendEmail(req.msg)).await,
                _ => self.publish(NotifyEvent::SendTextMessage(req.msg)).await,
            };
        }
    }

    let mediator = BasicMediator::<NotifyEvent>::builder()
        .add_listener(move |ev| {
            if let NotifyEvent::Ignore = ev {
                println!("Ignored some Message")
            }
        })
        .add_listener(move |ev| {
            if let NotifyEvent::SendEmail(msg) = ev {
                println!("Send Email with Message: {}", msg)
            }
        })
        .add_listener(move |ev| {
            if let NotifyEvent::SendTextMessage(msg) = ev {
                println!("Send SMS with Message: {}", msg)
            }
        })
        .build();

    async_std::task::block_on(async {
        let async_mediator = BasicAsyncMediator::<NotifyEvent>::from(mediator);

        async_mediator
            .send(UserMessageRequest {
                msg: String::from("Hello World"),
                priority: 0,
            })
            .await;

        async_mediator
            .send(UserMessageRequest {
                msg: String::from("Is Rust Memory Safe?"),
                priority: 2,
            })
            .await;

        async_mediator
            .send(UserMessageRequest {
                msg: String::from("New Rust Version"),
                priority: 8,
            })
            .await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();
        async_mediator.next().await.ok();
    });
}

#[cfg(feature = "async")]
#[test]
fn it_works_async() {
    use async_trait::async_trait;

    #[async_trait]
    impl AsyncRequestHandler<LoginRequest, AuthEvent> for BasicAsyncMediator<AuthEvent> {
        async fn handle(&self, req: LoginRequest) {
            if req.0.token == String::from("xyz") {
                self.publish(AuthEvent(req.0, AuthEventType::Login)).await
            }
        }
    }

    async_std::task::block_on(async {
        let mediator = BasicMediator::<AuthEvent>::builder()
            .add_listener(move |ev| println!("my listened event: {:?}", ev))
            .build();

        let async_mediator = BasicAsyncMediator::<AuthEvent>::from(mediator);

        async_mediator
            .send::<LoginRequest>(LoginRequest(Login {
                token: String::from("xyz"),
            }))
            .await;

        async_mediator.next().await.ok();
    })
}

#[cfg(feature = "async")]
#[test]
fn atomic_test_async() {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct IncrementRequest;
    #[derive(Debug, Clone)]
    struct IncrementEvent;

    #[async_trait]
    impl AsyncRequestHandler<IncrementRequest, IncrementEvent> for BasicAsyncMediator<IncrementEvent> {
        async fn handle(&self, _req: IncrementRequest) {
            self.publish(IncrementEvent).await
        }
    }

    async_std::task::block_on(async {
        let u = Arc::new(Mutex::new(0usize));
        let cloned = u.clone();
        let mediator = BasicMediator::<IncrementEvent>::builder()
            .add_listener(move |_| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + 1;
            })
            .build();

        let async_mediator = BasicAsyncMediator::<IncrementEvent>::from(mediator);

        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        assert_eq!(*(u.lock().unwrap()), 1usize);

        async_mediator.send(IncrementRequest).await;
        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();

        assert_eq!(*(u.lock().unwrap()), 3usize);
    })
}
