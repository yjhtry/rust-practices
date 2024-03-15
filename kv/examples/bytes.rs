use bytes::{BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);

    buf.put_u32(2);

    println!("{:?}", buf.len());

    println!("{:?}", &256_u32.to_be_bytes());
}
