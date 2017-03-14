extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_serde_json;

#[macro_use]
extern crate serde_json;

use futures::{Future, Sink};

use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

// Use length delimited frames
use tokio_io::codec::length_delimited;

use tokio_serde_json::WriteJson;

pub fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind a server socket
    let socket = TcpStream::connect(
        &"127.0.0.1:17653".parse().unwrap(),
        &handle);

    core.run(socket.and_then(|socket| {

        // Delimit frames using a length header
        let length_delimited = length_delimited::FramedWrite::new(socket);

        // Serialize frames with JSON
        let serialized = WriteJson::new(length_delimited);

        // Send the value
        serialized.send(json!({
          "name": "John Doe",
          "age": 43,
          "phones": [
            "+44 1234567",
            "+44 2345678"
          ]
        }))
    })).unwrap();
}
