use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// stream处理函数
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // 打印接收到的信息
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    // 生成基于7878端口的tcp服务监听器实例
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 遍历从客户端接收到的数据，再通过handle_connection处理
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 将数据提供给处理函数
                handle_connection(stream);
            }

            // 捕捉异常
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }
}
