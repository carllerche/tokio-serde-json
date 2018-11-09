extern crate futures;
extern crate tokio;
extern crate tokio_serde_json;

#[macro_use]
extern crate serde_json;

use futures::{Future, Sink};

use tokio::{
    codec::{FramedWrite, LengthDelimitedCodec},
    net::TcpStream,
};

use tokio_serde_json::WriteJson;

pub fn main() {
    // Bind a server socket
    let socket = TcpStream::connect(&"127.0.0.1:17653".parse().unwrap());

    tokio::run(
        socket
            .and_then(|socket| {
                // Delimit frames using a length header
                let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());

                // Serialize frames with JSON
                let serialized = WriteJson::new(length_delimited);

                // Send the value
                serialized
                    .send(json!({
                        "name": "John Doe",
                        "age": 43,
                        "phones": [
                            "+44 1234567",
                            "+44 2345678"
                        ]
                    })).map(|_| ())
            }).map_err(|_| ()),
    );
}
