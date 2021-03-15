# mediatrs

A simple `Mediator` for your Rust applications. This library is part
of a series of post about implementing OOP patterns in Rust, if this
interests you, feel free to check [my
blog](https://yorodm.is-a.dev/tags/rust/)

## Usage

```rust
pub struct Ping;
pub struct PingHandler;
impl Request for Ping {}
impl Handler<Ping, String> for PingHandler {
	fn handle(&self, _: &Ping) -> String {
		"Pong".to_owned()
	}
}
#[test]
fn test_ping_request() {
	let mut m = Mediator::new();
	m.add_handler::<Ping, _, _>(PingHandler {});
	let x: Option<String> = m.send::<Ping, _>(&Ping {});
	assert_eq!(x, Some("Pong".to_owned()))
}
```
