use super::prelude::*;

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
        .build()
        .unwrap();

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
        .build()
        .unwrap();

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

    let async_mediator = BasicAsyncMediator::<NotifyEvent>::builder()
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
        .build()
        .unwrap();

    async_std::task::block_on(async {
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
        let async_mediator = BasicAsyncMediator::<IncrementEvent>::builder()
            .add_listener(move |_| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + 1;
            })
            .build()
            .unwrap();

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

#[cfg(feature = "async")]
#[test]
fn cxaware_mediator_atomic_test_async() {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct IncrementRequest;
    #[derive(Debug, Clone)]
    struct IncrementEvent(usize);

    let base_num: usize = 3;

    #[async_trait]
    impl CxAwareAsyncRequestHandler<usize, IncrementRequest, IncrementEvent>
        for CxAwareAsyncMediator<usize, IncrementEvent>
    {
        async fn handle(&self, _req: IncrementRequest, dep: &usize) {
            self.publish(IncrementEvent(dep.clone())).await
        }
    }

    async_std::task::block_on(async {
        let u = Arc::new(Mutex::new(0usize));
        let cloned = u.clone();
        let async_mediator = CxAwareAsyncMediator::<usize, IncrementEvent>::builder()
            .add_listener(move |x: IncrementEvent| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + x.0;
            })
            .add_dependency(base_num)
            .build()
            .unwrap();

        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        assert_eq!(*(u.lock().unwrap()), 3usize);

        async_mediator.send(IncrementRequest).await;
        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();

        assert_eq!(*(u.lock().unwrap()), 9usize);
    })
}

#[cfg(feature = "async")]
#[test]
fn dependent_mediator_atomic_arc_test_async() {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct IncrementRequest;
    #[derive(Debug, Clone)]
    struct IncrementEvent(usize);

    let base_num: Arc<Mutex<usize>> = Arc::new(Mutex::new(5));

    #[async_trait]
    impl CxAwareAsyncRequestHandler<Arc<Mutex<usize>>, IncrementRequest, IncrementEvent>
        for CxAwareAsyncMediator<Arc<Mutex<usize>>, IncrementEvent>
    {
        async fn handle(&self, _req: IncrementRequest, dep: &Arc<Mutex<usize>>) {
            let c = {
                let mut m = dep.lock().unwrap();
                *m = *m - 1;
                m.clone() + 1
            };

            self.publish(IncrementEvent(c)).await
        }
    }

    async_std::task::block_on(async {
        let u = Arc::new(Mutex::new(0usize));
        let cloned = u.clone();
        let async_mediator = CxAwareAsyncMediator::<Arc<Mutex<usize>>, IncrementEvent>::builder()
            .add_listener(move |x: IncrementEvent| {
                let mut m = cloned.lock().unwrap();
                let c = *m;
                *m = c + x.0;
            })
            .add_dependency(base_num)
            .build()
            .unwrap();

        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        assert_eq!(*(u.lock().unwrap()), 5usize);

        async_mediator.send(IncrementRequest).await;
        async_mediator.send(IncrementRequest).await;

        async_mediator.next().await.ok();
        async_mediator.next().await.ok();

        assert_eq!(*(u.lock().unwrap()), 12usize);
    })
}
