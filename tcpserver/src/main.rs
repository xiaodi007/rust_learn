use std::net::TcpListener;
use std::io::{Read, Write};
use std::str;

fn main() {
    // 创建了一个TcpListener实例，并监听本机3000端口
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    // 打印服务开启
    println!("Running on port 3000...");

    // 监听tcp数据流
    for stream in listener.incoming() {
        // 可变的数据流
        let mut  stream = stream.unwrap();
        // 打印客户端连接成功
        println!("Connection established!");
        // 只取前5个字节
        let mut buf = [0;5];
        //读取数据流
        stream.read(&mut buf).unwrap();
        //将读取到的数据流，写回响应中
        stream.write(&mut buf).unwrap();
        // 将数据流保存到req_str这个变量里面
        let req_str = str::from_utf8(&buf).unwrap();
        // 打印数据流
        println!("Request from client:{:?}", req_str);

        //匹配到Hello，说明正常，没有则系统崩溃
        match req_str {
            "Hello" => println!("Yse is Hello"),
            _ => panic!("No hello in it")
        }
    }
}


