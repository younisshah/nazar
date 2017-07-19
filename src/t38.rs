extern crate redis;

/// Represents T38 types
#[allow(dead_code)]
pub enum Types {
    String(String),
    Int(isize),
    Float(f32)
}

/// Nazar alias for RedisResult
type NazarResult<T> = redis::RedisResult<T>;

/// Opens a T38 connection
fn get_connection(url: &str) -> redis::RedisResult<redis::Client> {
    redis::Client::open(url)
}

/// Executes a T38 command
pub fn execute(command: String, args: Vec<Types>) -> NazarResult<String> {
    let mut command = redis::cmd(&command[..]);
    for e in args {
        match e {
            Types::Int(arg) => command.arg(arg),
            Types::String(arg) => command.arg(arg),
            Types::Float(arg) => command.arg(arg),
        };
    }
    let connection = get_connection("redis://127.0.0.1:9851")?;
    command.query(&connection)
}