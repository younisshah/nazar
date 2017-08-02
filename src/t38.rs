extern crate redis;
extern crate ws;

use std::cell::RefCell;
use std::borrow::Cow;
use std::io;

/// Nazar alias for RedisResult
type NazarResult<T> = redis::RedisResult<T>;

/// Represents T38 types
pub enum Types {
    String(&'static str),
    Int(isize),
    Float(f32)
}

pub struct Client<'a> {
    url: &'static str,
    cmd: String,
    args: RefCell<Vec<Cow<'a, str>>>,
}

impl<'a> Client<'a> {
    #[deprecated(since = "1.0.1", note = "Please use `from` instead.")]
    pub fn new(url: &'static str) -> Self {
        Client { url, cmd: String::new(), args: RefCell::new(Vec::new()) }
    }
    /// Constructor with one argument - Rust convention!
    pub fn from(url: &'static str) -> Self {
        Client { url, cmd: String::new(), args: RefCell::new(Vec::new()) }
    }

    /// cmd takes and sets a Tile38 command
    pub fn cmd(&mut self, s: &'a str) -> &Client<'a> {
        self.cmd = String::from(s);
        self
    }

    /// Use arg to construct a Tile38 command.
    /// Although this does not work for 'Field' yet.
    /// Supports only &str and String args only
    ///
    /// #Example
    ///
    /// n.arg("POINT").arg("23").arg("321");
    ///
    ///
    pub fn arg<A>(&self, a: A) -> &Client<'a>
        where A: Into<Cow<'a, str>>
    {
        let v: Cow<'a, str> = a.into();
        if !v.to_string().is_empty() {
            self.args.borrow_mut().push(v);
        } else {
            println!("* [WARNING] arg cannot be empty. Skipping...");
        }
        self
    }


    /// execute_with_args executes Tile38 query
    pub fn execute_with_args(&self) -> NazarResult<String> {
        if !self.cmd.is_empty() {
            let mut command = redis::cmd(&self.cmd[..]);
            for a in self.args.borrow().iter() {
                command.arg(a.to_string());
            }
            command.query(&get_connection(self.url)?)
        } else {
            println!(" [ERROR] Command cannot be empty!");
            Err(redis::RedisError::from(io::Error::new(io::ErrorKind::NotFound, "Command cannot be empty")))
        }

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

    // Open a static geofence!
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
