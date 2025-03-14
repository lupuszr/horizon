use futures::{Stream, StreamExt};
use hyper::body::Bytes;
use iroh_blobs::rpc::client::blobs::Reader;
use pin_project::pin_project;
use s3s::stream::ByteStream;
use s3s::stream::DynByteStream;
use s3s::stream::RemainingLength;
use s3s::{dto::StreamingBlob, StdError};
use std::error::Error;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

pub fn adapt_stream(blob: StreamingBlob) -> impl Stream<Item = io::Result<Bytes>> + Send + Unpin {
    blob.map(|res| res.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
}

struct ReaderByteStream {
    inner: Reader,
}

impl Stream for ReaderByteStream {
    type Item = Result<Bytes, Box<dyn Error + Send + Sync>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(Ok(bytes))) => Poll::Ready(Some(Ok(bytes))),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(Box::new(err)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl ByteStream for ReaderByteStream {}
impl ByteStream for Pin<Box<ReaderByteStream>> {}

/// Converts `Reader` to `StreamingBlob`
/// TODO: this is not a zero copy transformation
pub fn reader_to_streaming_blob(reader: Reader) -> StreamingBlob {
    let byte_stream = Box::pin(ReaderByteStream { inner: reader });
    StreamingBlob::new(byte_stream)
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range>
pub fn fmt_content_range(start: u64, end_inclusive: u64, size: u64) -> String {
    format!("bytes {start}-{end_inclusive}/{size}")
}
