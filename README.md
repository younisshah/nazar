## Nazar

[![nazar](https://img.shields.io/crates/v/nazar.svg)](https://crates.io/crates/nazar)

[Tile38](http://tile38.com) is an open source (MIT licensed), in-memory geolocation data store, spatial index, 
and realtime geofence. It supports a variety of object types including lat/lon points, bounding boxes, XYZ tiles, 
Geohashes, and GeoJSON.

**nazar** is a Tile38 client in rust!

The API is a bit sane now albeit still weird and unstable. 

**API will change a lot**


### Install

In your `Cargo.toml` file add under `[dependencies]` section


```ini
[dependencies]
nazar = "1.0.4"
```

### Usage 


1) `SET` command

```rust
use self::nazar::t38::Types::{String, Float};
let n = nazar::t38::Client::from("redis://127.0.0.1:9851");

match n.execute("SET", vec![String("my"), String("home"), Float(23.12), Float(45.343)]) {
    Ok(s) => println!("{}", s),
    Err(e) => panic!(e)
}

```

2) `GET` command

```rust
use self::nazar::t38::Types::{String};
let n = nazar::t38::Client::from("redis://127.0.0.1:9851");

match n.execute("GET", vec![String("my"), String("home")]) {
    Ok(s) => println!("{}", s),
    Err(e) => panic!(e)
}
```

3) New API to execute T38 command - `cmd`, `arg` and `execute_with_args`. 
This is a high-level API to execute Tile38 commands!

```rust
let mut n = nazar::t38::Client::from("redis://127.0.0.1:9851");
n.cmd("SET").arg("drivers").arg("qwerty").arg("POINT").arg("23.54").arg("32.74");
match n.execute_with_args() {
    Ok(r) => println!("Result {}", r),
    Err(e) => panic!(e),
};
```


## Geofence features

To open a fence **only**, it is advisable to use `new` associated method like this:
 
```rust
let n = nazar::t38::Client::new();
```

Then use `n` to open a geofence like this:

1) Open a static `FENCE` using `open_fence`:

```rust
let work = |msg| {
    println!("FENCE updates {:?}", msg);
};
n.open_fence("ws://127.0.0.1:9851", "my_fleet", "12.12", "33.22", "6000", work);
```

2) Open a static geofence with GeoJSON object type. `open_fence_within`
 
 ```rust
let work = |msg| {
    println!("FENCE updates {:?}", msg);
};
n.open_fence_within("ws://localhost:9851", "my_fleet", "qwerty123", vec![vec![12.32, 23.4], vec![22.32, 33.4], vec![42.32, 23.5], vec![12.32, 23.4]], work)
```


3) Open a static `FENCE` using `open_fence` (use this when to want to communicate with the server as well):

```rust
fn action (out: &nazar::t38::NazarSender, msg: String) {
    out.send("OK").unwrap();
    println!("{}", msg);
    // do stuff with msg
}

//.....

n.open_fence2("ws://127.0.0.1:9851", "my_fleet", "12.12", "33.22", "6000", action);
```

4) New API to open a static geofence with GeoJSON object type. `open_fence_within` (use this when to want to communicate with the server as well):
 
 ```rust
fn action (out: &nazar::t38::NazarSender, msg: String) {
    out.send("OK").unwrap();
    println!("{}", msg);
    // do stuff with msg
}

//.....

n.open_fence_within2("ws://localhost:9851", "my_fleet", "qwerty123", vec![vec![12.32, 23.4], vec![22.32, 33.4], vec![42.32, 23.5], vec![12.32, 23.4]], action);
```


####  A work in progress

TODO

1) Make sane API.
1) Documentation
2) Roaming `FENCE` 
