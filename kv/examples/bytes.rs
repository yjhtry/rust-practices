use bytes::{Buf, BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);

    buf.get_u32();

    println!("{:?}", buf.len());
}
