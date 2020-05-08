[Some people claim that XPC is faster than unix domain sockets](https://developer.apple.com/forums/thread/74498).
I haven't seen any benchmarks to back it up, so I made this.

## Results

Tests were performed on a 2017 MacBook Pro, 2.9GHz i7, Monterey 12.2.

| Benchmark | UDS | XPC |
| --- | --- | -- |
| round trip of 10b | **11us** | 12us |
| round trip of 1KiB | **11us** | 15us |
| round trip of 10MiB | 17ms | **12ms** |
| connect and round trip of 10b | **32us** | 95us |
