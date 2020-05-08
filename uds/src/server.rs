use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

fn main() {
    let listener = UnixListener::bind("/tmp/benchmark_uds").unwrap();

    let mut len_buf = [0u8; 4];
    let mut buf = Vec::with_capacity(1024 * 1024 * 10);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        loop {
            if stream.read_exact(&mut len_buf).is_err() {
                break;
            }

            let len = u32::from_ne_bytes(len_buf);
            buf.resize(len as usize, 0);
            stream.read_exact(&mut buf).unwrap();

            stream.write_all(len.to_ne_bytes().as_ref()).unwrap();
            stream.write_all(&buf).unwrap();
        }
    }
}
