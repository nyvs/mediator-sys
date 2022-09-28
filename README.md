# mediator-sys
[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]

Event Mediator for synchronous and asynchronous environments.

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
The async mediator uses the `BasicAsyncMediator` and the `AsyncRequestHandler`,
Take a look at the test module for an example. The API is identical.

## Features
- sync and async mediator (use `async` feature)
- compiler-baked typing (no `std::any::Any`)
- extensible

## Contributions
Feel free to open an issue/PR explaining possible improvements or changes.

## Help
Also, please do not hesitate and open an issue when needed. I am happy to help!

[crates-io-badge]: https://img.shields.io/crates/v/mediator-sys.svg
[crates-io-url]: https://crates.io/crates/mediator-sys
[docs-rs-img]: https://docs.rs/mediator-sys/badge.svg
[docs-rs-url]: https://docs.rs/mediator-sys
