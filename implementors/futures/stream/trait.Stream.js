(function() {var implementors = {};
implementors["tokio_serde_json"] = [{text:"impl&lt;T, U&gt; <a class=\"trait\" href=\"https://docs.rs/futures/0.1/futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_serde_json/struct.ReadJson.html\" title=\"struct tokio_serde_json::ReadJson\">ReadJson</a>&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://docs.rs/futures/0.1/futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"type\" href=\"https://docs.rs/futures/0.1/futures/stream/trait.Stream.html#associatedtype.Error\" title=\"type futures::stream::Stream::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.33/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.80/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://docs.rs/bytes/0.4.10/bytes/bytes/struct.BytesMut.html\" title=\"struct bytes::bytes::BytesMut\">BytesMut</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T::<a class=\"type\" href=\"https://docs.rs/futures/0.1/futures/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures::stream::Stream::Item\">Item</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_serde_json::ReadJson"]},{text:"impl&lt;T, U&gt; <a class=\"trait\" href=\"https://docs.rs/futures/0.1/futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_serde_json/struct.WriteJson.html\" title=\"struct tokio_serde_json::WriteJson\">WriteJson</a>&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://docs.rs/futures/0.1/futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"https://docs.rs/futures/0.1/futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a>,&nbsp;</span>",synthetic:false,types:["tokio_serde_json::WriteJson"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
