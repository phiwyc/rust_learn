// 引入库
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// 入口
fn main() {
  // 地址与端口
  let local_address = String::from("127.0.0.1");
  let port = String::from("7777");
  // 地址
  let host = local_address + ":" + &port;
  // 监听器
  let listener = TcpListener::bind(&host).unwrap();
  // 打印日志
  println!("TCP Server is listening at port:{}", port);
  // 遍历监听器获取的信息
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

// 遍历
fn handle_connection (mut stm: TcpStream) {
  // 获取传入的内容
  let mut buffer = [0, 255];
  stm.read(&mut buffer).unwrap();
  // 打印
  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
