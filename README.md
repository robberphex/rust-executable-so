# rust executable so

1. `libexample.so` could be load by `entry`.
2. `libexample.so` could be execute directly.

# How to run

```bash
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.18s
$ ./target/debug/entry
This is a message from libexample.so.
This is a message from entry main.
$ ./target/debug/libexample.so
This is a message from libexample.so.
This is a message from libexample.so's _start
```
