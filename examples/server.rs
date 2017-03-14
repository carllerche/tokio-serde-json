extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_serde_json;
extern crate serde_json;

use futures::Stream;

use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

// Use length delimited frames
use tokio_io::codec::length_delimited;

use serde_json::Value;
use tokio_serde_json::ReadJson;

pub fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind a server socket
    let listener = TcpListener::bind(
        &"127.0.0.1:17653".parse().unwrap(),
        &handle).unwrap();

    println!("listening on {:?}", listener.local_addr());

    core.run(listener.incoming().for_each(|(socket, _)| {
        // Delimit frames using a length header
        let length_delimited = length_delimited::FramedRead::new(socket);

        // Deserialize frames
        let deserialized = ReadJson::<_, Value>::new(length_delimited)
            .map_err(|e| println!("ERR: {:?}", e));

        // Spawn a task that prints all received messages to STDOUT
        handle.spawn(deserialized.for_each(|msg| {
            println!("GOT: {:?}", msg);
            Ok(())
        }));

        Ok(())
    })).unwrap();
}
