## Nazar

[Tile38](http://tile38.com) is an open source (MIT licensed), in-memory geolocation data store, spatial index, 
and realtime geofence. It supports a variety of object types including lat/lon points, bounding boxes, XYZ tiles, 
Geohashes, and GeoJSON.

nazar is a Tile38 client in rust!

The API is a bit sane now albeit still weird.

### Install

In your `Cargo.toml` file add under `[dependencies]` section


```ini
[dependencies]
nazar = "1.0.1"
```

### Usage 


1) `SET` command

```rust
use self::nazar::t38::Types::{String, Float};
let n = nazar::t38::Client::from("redis://127.0.0.1:9851"); // new is now deprecated!

match n.execute("SET", vec![String("my"), String("home"), Float(23.12), Float(45.343)]) {
    Ok(s) => println!("{}", s),
    Err(e) => panic!(e)
}

```

2) `GET` command

```rust
use self::nazar::t38::Types::{String};
let n = nazar::t38::Client::from("redis://127.0.0.1:9851"); // new is now deprecated!

match n.execute("GET", vec![String("my"), String("home")]) {
    Ok(s) => println!("{}", s),
    Err(e) => panic!(e)
}
```

3) Open a static `FENCE`

```rust
use self::nazar::t38::Types::{String};
let work = |msg| {
    println!("FENCE updates {:?}", msg);
};
n.open_fence("ws://127.0.0.1:9851", "my_fleet", "12.12", "33.22", "6000", work);
```

####  A work in progress

TODO

1) Make sane API.
1) Documentation
2) Roaming `FENCE` 