bouyomi
========

Rust Client for BouyomiChan

usage
------

* Add following setting to your Cargo.toml

```toml
bouyomi = {version = "0.1.2", git = "https://github.com/0x75960/bouyomi"}
```

* use in code 

```rust
let client = bouyomi::Client::default();
client.talk("こんにちは").expected("something happened..");
```

|command|status|
|:--|:--|
|talk|:white_check_mark:|
|talk_manual|:white_check_mark:|
|pause|:white_check_mark:|
|resume|:white_check_mark:|
|skip|:white_check_mark:|
|clear|:white_check_mark:|
|get_pause|:white_check_mark:|
|get_now_playing|:white_check_mark:|
|get_remain_task|:white_check_mark:|

I wrote this with referencing https://github.com/xztaityozx/VSBouyomi.

thanks!

