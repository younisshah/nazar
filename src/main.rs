extern crate nazar;

fn work(out: &nazar::t38::NazarSender, msg: String) {
    out.send("OK").unwrap();
    println!("{}", msg);
}

fn main() {
    let n = nazar::t38::Client::new();
    n.open_fence_within("ws://localhost:9851", "deep_fleet", "qwerty123", vec![vec![12.32, 23.4], vec![22.32, 33.4], vec![42.32, 23.5], vec![12.32, 23.4]], work)
}