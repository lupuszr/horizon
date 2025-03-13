use futures::{Stream, StreamExt};
use hyper::body::Bytes;
use s3s::dto::StreamingBlob;
use std::io;

pub fn adapt_stream(blob: StreamingBlob) -> impl Stream<Item = io::Result<Bytes>> + Send + Unpin {
    blob.map(|res| res.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
}
