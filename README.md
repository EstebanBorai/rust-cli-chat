<div align="center">
  <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  <h1>rust-cli-chat</h1>
  <small>Rust Client & Server CLI Chat Application</small>
</div>

## Motivation

Explore Rust threads, TCP streams and channels.

## Getting Started

- Run the `server` crate

```bash
cd server && cargo run
```

A TCP listener will bind to `127.0.0.1:6000`, awating for connections.

```log
➜  server git:(main) cargo run
   Compiling server v0.1.0 (/projects/rust-cli-chat/server)
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/server`
Client 127.0.0.1:56795 connected
```

Then you must run the `client` crate in another terminal window or tab.

```bash
cd client && cargo run
```

A connection to the TCP listener on `127.0.0.1:6000` will be established
and you will be prompted to write a message:

```log
➜  client git:(main) cargo run
   Compiling client v0.1.0 (/projects/rust-cli-chat/client)
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/client`
Write a message:
```
