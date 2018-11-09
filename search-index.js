var N = null;var searchIndex = {};
searchIndex["tokio_serde_json"]={"doc":"`Stream` and `Sink` adaptors for serializing and deserializing values using JSON.","items":[[3,"ReadJson","tokio_serde_json","Adapts a stream of JSON encoded buffers to a stream of values by deserializing them.",N,N],[3,"WriteJson","","Adapts a buffer sink to a value sink by serializing the values as JSON.",N,N],[11,"new","","Creates a new `ReadJson` with the given buffer stream.",0,[[["t"]],["readjson"]]],[11,"get_ref","","Returns a reference to the underlying stream wrapped by `ReadJson`.",0,[[["self"]],["t"]]],[11,"get_mut","","Returns a mutable reference to the underlying stream wrapped by `ReadJson`.",0,[[["self"]],["t"]]],[11,"into_inner","","Consumes the `ReadJson`, returning its underlying stream.",0,[[["self"]],["t"]]],[11,"poll","","",0,[[["self"]],["poll",["option"]]]],[11,"start_send","","",0,N],[11,"poll_complete","","",0,[[["self"]],["poll"]]],[11,"close","","",0,[[["self"]],["poll"]]],[11,"new","","Creates a new `WriteJson` with the given buffer sink.",1,[[["t"]],["writejson"]]],[11,"get_ref","","Returns a reference to the underlying sink wrapped by `WriteJson`.",1,[[["self"]],["t"]]],[11,"get_mut","","Returns a mutable reference to the underlying sink wrapped by `WriteJson`.",1,[[["self"]],["t"]]],[11,"into_inner","","Consumes the `WriteJson`, returning its underlying sink.",1,[[["self"]],["t"]]],[11,"start_send","","",1,[[["self"],["u"]],["startsend"]]],[11,"poll_complete","","",1,[[["self"]],["poll"]]],[11,"close","","",1,[[["self"]],["poll"]]],[11,"poll","","",1,[[["self"]],["poll",["option"]]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]]],"paths":[[3,"ReadJson"],[3,"WriteJson"]]};
initSearch(searchIndex);
