## Nazar

A Tile38 client in rust!

The API is a bit weird although it's set to change in future versions.

### Install

In your `Cargo.toml` file add under `[dependencies]` section


```ini
[dependencies]
nazar = "0.1.0"
```

### Usage 


1) `SET` command

```rust
extern crate nazar;

use nazar::t38::*;

let cmd = String::from("SET");
    let args = vec!(nazar::t38::Types::String(String::from("my")),
                   nazar::t38::Types::String(String::from("home")),
                   nazar::t38::Types::String(String::from("POINT")),
                   nazar::t38::Types::Float(33.12),
                   nazar::t38::Types::Float(33.112));
    println!("rust_key: {}", nazar::t38::execute(cmd, args).unwrap());
```

2) `GET` command

```rust
let cmd = String::from("GET");
    let args = vec!(nazar::t38::Types::String(String::from("my")),
                   nazar::t38::Types::String(String::from("home")));
    println!("rust_key: {}", nazar::t38::execute(cmd, args).unwrap());
```

####  A work in progress

TODO

1) Documentation
2) `Fence` command 
3) Websocket support