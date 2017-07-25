extern crate redis;
extern crate ws;

/// Nazar alias for RedisResult
type NazarResult<T> = redis::RedisResult<T>;

/// Represents T38 types
pub enum Types {
    String(&'static str),
    Int(isize),
    Float(f32)
}

#[derive(Clone, Copy)]
pub struct Client {
    url: &'static str
}

impl Client {
    #[deprecated(since="1.0.1", note= "Please use `from` instead.")]
    pub fn new(url: &'static str) -> Self {
        Client { url }
    }
    /// Constructor with one argument - Rust convention!
    pub fn from(url: &'static str) -> Self {
        Client { url }
    }
    /// low level API
    pub fn execute(self, command: &str, args: Vec<Types>) -> NazarResult<String> {
        let mut command = redis::cmd(command);
        for e in args {
            match e {
                Types::Int(arg) => command.arg(arg),
                Types::String(arg) => command.arg(arg),
                Types::Float(arg) => command.arg(arg),
            };
        }
        command.query(&get_connection(self.url)?)
    }
    pub fn open_fence<F>(self, url: &str, fleet: &str, lat: &str, lng: &str, radius: &str, work: F) where F: Fn(String) {
        let cmd_url = format!("{}/NEARBY+{}+FENCE+POINT+{}+{}+{}", url, fleet, lat, lng, radius);
        ws::connect(cmd_url, |_out| {
            |msg: ws::Message| {
                match msg.into_text() {
                    Ok(m) => work(m),
                    Err(e) => println!("ERR: {:?}", e),
                }
                Ok(())
            }
        }).unwrap()
    }
}

/// Opens a T38 connection
fn get_connection(url: &str) -> redis::RedisResult<redis::Client> {
    redis::Client::open(url)
}
