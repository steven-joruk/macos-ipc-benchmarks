use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub fn write_and_read(stream: &mut UnixStream, buffer: &mut Vec<u8>) {
    stream
        .write_all(&(buffer.len() as u32).to_ne_bytes())
        .unwrap();
    stream.write_all(buffer.as_slice()).unwrap();

    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();
    stream.read_exact(buffer).unwrap();
}

fn main() {
    let mut stream = UnixStream::connect("/tmp/benchmark_uds").unwrap();
    let mut buffer: Vec<u8> = b"Hello world".to_vec();

    write_and_read(&mut stream, &mut buffer);
}
