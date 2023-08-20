use abi::AddLocalFolderResponse;
use http::header::{ACCEPT, CONTENT_TYPE};
use prost::bytes::{Buf, BufMut, Bytes, BytesMut};

#[tokio::main]
async fn main() {
  let body = abi::AddLocalFolderRequest {
    path: "D:\\media\\music".to_string(),
  };

  let request = http::Request::builder()
    .version(http::Version::HTTP_11)
    .method(http::Method::POST)
    .uri("http://127.0.0.1:8396/musync.MusyncService/AddLocalFolder")
    .header(CONTENT_TYPE, "application/grpc-web")
    .header(ACCEPT, "application/grpc-web")
    .body(hyper::Body::from(encode_body(body)))
    .unwrap();

  let client = hyper::Client::new();
  let response = client.request(request).await.unwrap();

  assert_eq!(
    response.headers().get(CONTENT_TYPE).unwrap(),
    "application/grpc-web+proto"
  );

  let body = response.into_body();
  let reply = decode_body::<AddLocalFolderResponse>(body).await;

  println!("REPLY={:?}", reply);
}

// one byte for the compression flag plus four bytes for the length
const GRPC_HEADER_SIZE: usize = 5;

fn encode_body<T>(msg: T) -> Bytes
where
  T: prost::Message,
{
  let msg_len = msg.encoded_len();
  let mut buf = BytesMut::with_capacity(GRPC_HEADER_SIZE + msg_len);

  // compression flag, 0 means "no compression"
  buf.put_u8(0);
  buf.put_u32(msg_len as u32);

  msg.encode(&mut buf).unwrap();
  buf.freeze()
}

async fn decode_body<T>(body: hyper::Body) -> T
where
  T: Default + prost::Message,
{
  let mut body = hyper::body::to_bytes(body).await.unwrap();

  // ignore the compression flag
  body.advance(1);

  let len = body.get_u32();
  #[allow(clippy::let_and_return)]
  let msg = T::decode(&mut body.split_to(len as usize)).unwrap();

  msg
}
