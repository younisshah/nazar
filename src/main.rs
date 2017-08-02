extern crate nazar;

fn main() {

    let mut n = nazar::t38::Client::from("redis://127.0.0.1:9851");
    n.cmd("SET").arg("drivers").arg("qwerty").arg("POINT").arg("23").arg("324");
    match n.execute_with_args() {
        Ok(r) => println!("Result {}", r),
        Err(e) => panic!(e),
    };
}