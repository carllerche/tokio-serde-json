#![doc(html_root_url = "https://docs.rs/tokio-serde-json/0.1.0")]

//! `Stream` and `Sink` adaptors for serializing and deserializing values using
//! JSON.
//!
//! This crate provides adaptors for going from a stream or sink of buffers
//! ([`Bytes`]) to a stream or sink of values by performing JSON encoding or
//! decoding. It is expected that each yielded buffer contains a single
//! serialized JSON value. The specific strategy by which this is done is left
//! up to the user. One option is to use using [`length_delimited`] from
//! [tokio-io].
//!
//! # Examples
//!
//! ```ignore
//! use futures::{Future, Sink};
//!
//! use tokio_core::reactor::Core;
//! use tokio_core::net::TcpStream;
//!
//! // Use length delimited frames
//! use tokio_io::codec::length_delimited;
//! use tokio_serde_json::WriteJson;
//!
//! // Bind a server socket
//! let socket = TcpStream::connect(
//!     &"127.0.0.1:17653".parse().unwrap(),
//!     &handle);
//!
//! socket.and_then(|socket| {
//!     // Delimit frames using a length header
//!     let length_delimited = length_delimited::FramedWrite::new(socket);
//!
//!     // Serialize frames with JSON
//!     let serialized = WriteJson::new(length_delimited);
//!
//!     // Send the value
//!     serialized.send(json!({
//!       "name": "John Doe",
//!       "age": 43,
//!       "phones": [
//!         "+44 1234567",
//!         "+44 2345678"
//!       ]
//!     }))
//! })
//! ```
//!
//! For a full working server and client example, see the [examples] directory.
//!
//! [`Bytes`]: https://docs.rs/bytes/0.4/bytes/struct.Bytes.html
//! [`length_delimited`]: https://docs.rs/tokio-io/0.1/tokio_io/codec/length_delimited/index.html
//! [tokio-io]: https://github.com/tokio-rs/tokio-io
//! [examples]: https://github.com/carllerche/tokio-serde-json/tree/master/examples

extern crate futures;
extern crate bytes;
extern crate serde;
extern crate serde_json;
extern crate tokio_serde;

use futures::{Stream, Poll, Sink, StartSend};
use bytes::{Bytes, BytesMut, Buf, IntoBuf};
use serde::{Serialize, Deserialize};
use tokio_serde::{Serializer, Deserializer, FramedRead, FramedWrite};

use std::marker::PhantomData;

/// Adapts a stream of JSON encoded buffers to a stream of values by
/// deserializing them.
///
/// `ReadJson` implements `Sink` by polling the inner buffer stream and
/// deserializing the buffer as JSON. It expects that each yielded buffer
/// represents a single JSON value and does not contain any extra trailing
/// bytes.
///
/// If a `ReadJson` is used concurrently from two or more threads, it is
/// guaranteed that only one object will be read at a time. In other words, once
/// an object begins being read, that object will continue being read until it
/// finishes or errors, and another object will only be read once the first
/// object completes.
pub struct ReadJson<T, U> {
    inner: FramedRead<T, U, Json<U>>,
}

/// Adapts a buffer sink to a value sink by serializing the values as JSON.
///
/// `WriteJson` implements `Sink` by serializing the submitted values to a
/// buffer. The buffer is then sent to the inner stream, which is responsible
/// for handling framing on the wire.
///
/// If a `WriteJson` is used concurrently from two or more threads, it is
/// guaranteed that the bytes of different objects written will not be
/// interleaved. In other words, if two calls to `WriteJson::send` overlap, then
/// the bytes of one object will be written entirely before the bytes of the
/// other.
pub struct WriteJson<T: Sink, U> {
    inner: FramedWrite<T, U, Json<U>>,
}

struct Json<T> {
    ghost: PhantomData<T>,
}

impl<T, U> ReadJson<T, U>
    where T: Stream,
          T::Error: From<serde_json::Error>,
          for<'a> U: Deserialize<'a>,
          Bytes: From<T::Item>,
{
    /// Creates a new `ReadJson` with the given buffer stream.
    pub fn new(inner: T) -> ReadJson<T, U> {
        let json = Json { ghost: PhantomData };
        ReadJson { inner: FramedRead::new(inner, json) }
    }
}

impl<T, U> ReadJson<T, U> {
    /// Returns a reference to the underlying stream wrapped by `ReadJson`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying stream wrapped by
    /// `ReadJson`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consumes the `ReadJson`, returning its underlying stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T, U> Stream for ReadJson<T, U>
    where T: Stream,
          T::Error: From<serde_json::Error>,
          for<'a> U: Deserialize<'a>,
          Bytes: From<T::Item>,
{
    type Item = U;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

impl<T, U> Sink for ReadJson<T, U>
    where T: Sink,
{
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: T::SinkItem)
                  -> StartSend<T::SinkItem, T::SinkError> {
        self.get_mut().start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.get_mut().poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.get_mut().close()
    }
}

impl<T, U> WriteJson<T, U>
    where T: Sink<SinkItem = BytesMut>,
          T::SinkError: From<serde_json::Error>,
          U: Serialize,
{
    /// Creates a new `WriteJson` with the given buffer sink.
    pub fn new(inner: T) -> WriteJson<T, U> {
        let json = Json { ghost: PhantomData };
        WriteJson { inner: FramedWrite::new(inner, json) }
    }
}

impl<T: Sink, U> WriteJson<T, U> {
    /// Returns a reference to the underlying sink wrapped by `WriteJson`.
    ///
    /// Note that care should be taken to not tamper with the underlying sink as
    /// it may corrupt the sequence of frames otherwise being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying sink wrapped by
    /// `WriteJson`.
    ///
    /// Note that care should be taken to not tamper with the underlying sink as
    /// it may corrupt the sequence of frames otherwise being worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consumes the `WriteJson`, returning its underlying sink.
    ///
    /// Note that care should be taken to not tamper with the underlying sink as
    /// it may corrupt the sequence of frames otherwise being worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T, U> Sink for WriteJson<T, U>
    where T: Sink<SinkItem = BytesMut>,
          T::SinkError: From<serde_json::Error>,
          U: Serialize,
{
    type SinkItem = U;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: U) -> StartSend<U, T::SinkError> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.inner.close()
    }
}

impl<T, U> Stream for WriteJson<T, U>
    where T: Stream + Sink,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<T::Item>, T::Error> {
        self.get_mut().poll()
    }
}

impl<T> Deserializer<T> for Json<T>
    where for <'a> T: Deserialize<'a>,
{
    type Error = serde_json::Error;

    fn deserialize(&mut self, src: &Bytes) -> Result<T, Self::Error> {
        serde_json::from_reader(src.into_buf().reader())
    }
}

impl<T: Serialize> Serializer<T> for Json<T> {
    type Error = serde_json::Error;

    fn serialize(&mut self, item: &T) -> Result<BytesMut, Self::Error> {
        serde_json::to_vec(item).map(Into::into)
    }
}
