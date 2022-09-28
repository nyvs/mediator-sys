# mediator-sys
[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]

Event mediator for synchronous and asynchronous environments.

## Usage
### Sync
```rust
use mediator_sys::prelude::*;

struct UserMessageRequest {
    msg: String,
    priority: u8,
}

#[derive(Clone)]
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

// Prints: Ignored some Message
mediator.next();
// Prints: Send Email with Message: Is Rust Memory Safe?
mediator.next();
// Prints: Send SMS with Message: New Rust Version
mediator.next();

```

### Async
<details>
<summary>Click to open the asynchronous version</summary>

```rust
use mediator_sys::prelude::*;
use async_trait::async_trait;

struct UserMessageRequest {
    msg: String,
    priority: u8,
}

#[derive(Clone)]
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

    async_mediator.send(UserMessageRequest {
        msg: String::from("Hello World"),
        priority: 0,
    }).await;

    async_mediator.send(UserMessageRequest {
        msg: String::from("Is Rust Memory Safe?"),
        priority: 2,
    }).await;

    async_mediator.send(UserMessageRequest {
        msg: String::from("New Rust Version"),
        priority: 8,
    }).await;

    async_mediator.next().await;
    async_mediator.next().await;
    async_mediator.next().await;
});
```

</details>

## Features
- sync and async mediator (use `async` feature)
- compiler-baked typing
- extensible

## Contributions
Feel free to open an issue/PR explaining possible improvements or changes.

## Help
Also, please do not hesitate and open an issue when needed. I am happy to help!

[crates-io-badge]: https://img.shields.io/crates/v/mediator-sys.svg
[crates-io-url]: https://crates.io/crates/mediator-sys
[docs-rs-img]: https://docs.rs/mediator-sys/badge.svg
[docs-rs-url]: https://docs.rs/mediator-sys
