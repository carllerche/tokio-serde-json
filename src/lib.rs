#![doc(html_root_url = "https://docs.rs/tokio-serde-json/0.3.0")]

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
//! ```no_run
//! use futures::prelude::*;
//!
//! use serde_json::json;
//!
//! use tokio::{codec::{FramedWrite, LengthDelimitedCodec}, net::TcpStream};
//!
//! use tokio_serde_json::WriteJson;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Bind a server socket
//!     let socket = TcpStream::connect("127.0.0.1:17653")
//!         .await
//!         .unwrap();
//!
//!     // Delimit frames using a length header
//!     let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
//!
//!     // Serialize frames with JSON
//!     let mut serialized = WriteJson::new(length_delimited);
//!
//!     // Send the value
//!     serialized.send(json!({
//!       "name": "John Doe",
//!       "age": 43,
//!       "phones": [
//!         "+44 1234567",
//!         "+44 2345678"
//!       ]
//!     })).await.unwrap()
//! }
//! ```
//!
//! For a full working server and client example, see the [examples] directory.
//!
//! [`Bytes`]: https://docs.rs/bytes/0.4/bytes/struct.Bytes.html
//! [`length_delimited`]: https://docs.rs/tokio-io/0.1/tokio_io/codec/length_delimited/index.html
//! [tokio-io]: https://github.com/tokio-rs/tokio-io
//! [examples]: https://github.com/carllerche/tokio-serde-json/tree/master/examples

use bytes::{Buf, Bytes, BytesMut, IntoBuf};
use futures::prelude::*;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use tokio_serde::{Deserializer, FramedRead, FramedWrite, Serializer};

use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

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
#[pin_project]
pub struct ReadJson<T, U> {
    #[pin]
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
#[pin_project]
pub struct WriteJson<T, U> {
    #[pin]
    inner: FramedWrite<T, U, Json<U>>,
}

struct Json<T> {
    ghost: PhantomData<T>,
}

impl<T, U> ReadJson<T, U> {
    /// Creates a new `ReadJson` with the given buffer stream.
    pub fn new(inner: T) -> Self {
        let json = Json { ghost: PhantomData };
        Self {
            inner: FramedRead::new(inner, json),
        }
    }

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
where
    T: TryStream<Ok = BytesMut>,
    T::Error: From<serde_json::Error>,
    for<'a> U: Deserialize<'a>,
{
    type Item = Result<U, T::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

impl<T, U, SinkItem> Sink<SinkItem> for ReadJson<T, U>
where
    T: Sink<SinkItem>,
{
    type Error = T::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: SinkItem) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

impl<T, U> WriteJson<T, U> {
    /// Creates a new `ReadJson` with the given buffer stream.
    pub fn new(inner: T) -> Self {
        let json = Json { ghost: PhantomData };
        Self {
            inner: FramedWrite::new(inner, json),
        }
    }

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

impl<T, U> Stream for WriteJson<T, U>
where
    T: Stream,
{
    type Item = T::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

impl<T, U> Sink<U> for WriteJson<T, U>
where
    T: Sink<Bytes>,
    T::Error: From<serde_json::Error>,
    U: Serialize,
{
    type Error = T::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: U) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

impl<T> Deserializer<T> for Json<T>
where
    for<'a> T: Deserialize<'a>,
{
    type Error = serde_json::Error;

    fn deserialize(self: Pin<&mut Self>, src: &BytesMut) -> Result<T, Self::Error> {
        serde_json::from_reader(src.into_buf().reader())
    }
}

impl<T: Serialize> Serializer<T> for Json<T> {
    type Error = serde_json::Error;

    fn serialize(self: Pin<&mut Self>, item: &T) -> Result<Bytes, Self::Error> {
        serde_json::to_vec(item).map(Into::into)
    }
}
