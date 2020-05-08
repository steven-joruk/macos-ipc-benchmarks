use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub fn write_and_read(stream: &mut UnixStream, buffer: &mut [u8]) {
    stream
        .write_all(&(buffer.len() as u32).to_ne_bytes())
        .unwrap();
    stream.write_all(buffer).unwrap();

    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();
    let len = u32::from_ne_bytes(len_buf) as usize;
    stream.read_exact(&mut buffer[0..len]).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut connection = UnixStream::connect("/tmp/benchmark_uds").unwrap();
    let mut buffer = Vec::new();

    c.bench_function("round trip of 10b", |b| {
        buffer.resize(10, 1);
        b.iter(|| write_and_read(&mut connection, &mut buffer))
    });

    c.bench_function("round trip of 1KiB", |b| {
        buffer.resize(1024, 1);
        b.iter(|| write_and_read(&mut connection, &mut buffer))
    });

    c.bench_function("round trip of 10MiB", |b| {
        buffer.resize(10 * 1024 * 1024, 1);
        b.iter(|| write_and_read(&mut connection, &mut buffer))
    });

    drop(connection);

    c.bench_function("connect and round trip of 10b", |b| {
        b.iter(|| {
            buffer.resize(10, 1);
            let mut connection = UnixStream::connect("/tmp/benchmark_uds").unwrap();
            write_and_read(&mut connection, &mut buffer);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
