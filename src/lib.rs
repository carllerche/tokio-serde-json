extern crate futures;
extern crate bytes;
extern crate serde;
extern crate serde_json;
extern crate tokio_serde;

use futures::{Stream, Poll, Sink, StartSend};
use bytes::{Bytes, BytesMut, Buf, IntoBuf};
use serde::{Serialize, Deserialize};
use serde_json::error::Error;
use tokio_serde::{Serializer, Deserializer, FramedRead, FramedWrite};

use std::io;
use std::marker::PhantomData;

pub struct ReadJson<T, U> {
    inner: FramedRead<T, U, Json<U>>,
}

pub struct WriteJson<T: Sink, U> {
    inner: FramedWrite<T, U, Json<U>>,
}

struct Json<T> {
    ghost: PhantomData<T>,
}

impl<T, U> ReadJson<T, U>
    where T: Stream<Error = io::Error>,
          U: Deserialize,
          Bytes: From<T::Item>,
{
    pub fn new(inner: T) -> ReadJson<T, U> {
        let json = Json { ghost: PhantomData };
        ReadJson { inner: FramedRead::new(inner, json) }
    }
}

impl<T, U> ReadJson<T, U> {
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }
}

impl<T, U> Stream for ReadJson<T, U>
    where T: Stream<Error = io::Error>,
          U: Deserialize,
          Bytes: From<T::Item>,
{
    type Item = U;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<U>, Error> {
        self.inner.poll()
    }
}

impl<T, U> WriteJson<T, U>
    where T: Sink<SinkItem = BytesMut, SinkError = io::Error>,
          U: Serialize,
{
    pub fn new(inner: T) -> WriteJson<T, U> {
        let json = Json { ghost: PhantomData };
        WriteJson { inner: FramedWrite::new(inner, json) }
    }
}

impl<T, U> Sink for WriteJson<T, U>
    where T: Sink<SinkItem = BytesMut, SinkError = io::Error>,
          U: Serialize,
{
    type SinkItem = U;
    type SinkError = io::Error;

    fn start_send(&mut self, item: U) -> StartSend<U, io::Error> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), io::Error> {
        self.inner.poll_complete()
    }
}

impl<T: Deserialize> Deserializer<T> for Json<T> {
    type Error = Error;

    fn deserialize(&mut self, src: &Bytes) -> Result<T, Error> {
        serde_json::from_reader(src.into_buf().reader())
    }
}

impl<T: Serialize> Serializer<T> for Json<T> {
    type Error = io::Error;

    fn serialize(&mut self, item: &T) -> Result<BytesMut, io::Error> {
        serde_json::to_vec(item)
            .map(Into::into)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
