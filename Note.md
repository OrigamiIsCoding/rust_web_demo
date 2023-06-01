# 笔记

## TcpServer

在 Rust 中 `std::net` 提供了一些网络相关的基础工具。

要建立 Tcp 连接的服务端可以使用 `std::net::TcpListener`。

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    // 如果只接受一个请求，就可以调用 accept 方法
    let result = listener.accept();
}
```

如果需要持续监听可以使用 `incoming` 方法

```rust
for stream in listener.incoming() {
    let _stream: TcpStream = stream.unwrap();
    println!("Connection established!")
}
```

`std::net::TcpStream` 实现了 `std::io::{Read, Write}` 的 trait，所以可以进行读和写，以下是读取客户端的数据然后再原封不动返回。

```rust
for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    println!("Connection established!");
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(&mut buffer).unwrap();
}
```

## TcpClient

使用 Rust 进行 Tcp 连接，可以使用 `std::net::TcpStream`。

```rust
use std::net::TcpStream;

fn main() {
    let _stream: TcpStream = TcpStream::connect("127.0.0.1:3000").unwrap();
}
```

拿到 TcpStream 就可以对它进行读和写了。

```rust
let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
stream.write("Hello".as_bytes()).unwrap();

let mut buffer = [0; 5];
stream.read(&mut buffer).unwrap();

println!(
    "Response from server: {:?}",
    str::from_utf8(&buffer)
);
```