bouyomi
========

Rust Client for BouyomiChan

usage
------

* Add following setting to your Cargo.toml

```toml
bouyomi = {version = "0.1.0", git = "https://github.com/0x75960/bouyomi"}
```

* use in code 

```rust
let client = bouyomi::BouyomiChanClient::default();
client.talk_with_default("こんにちは").expected("something happened..");
```

enjoy!
