use futures::prelude::*;
use serde_json::Value;
use tokio::{
    codec::{FramedRead, LengthDelimitedCodec},
    net::TcpListener,
};
use tokio_serde_json::ReadJson;

#[tokio::main]
pub async fn main() {
    // Bind a server socket
    let listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();

    println!("listening on {:?}", listener.local_addr());

    let mut s = listener.incoming();

    while let Some(socket) = s.try_next().await.unwrap() {
        // Delimit frames using a length header
        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());

        // Deserialize frames
        let mut deserialized = ReadJson::<_, Value>::new(length_delimited);

        // Spawn a task that prints all received messages to STDOUT
        tokio::spawn(async move {
            while let Some(msg) = deserialized.try_next().await.unwrap() {
                println!("GOT: {:?}", msg);
            }
        });
    }
}
