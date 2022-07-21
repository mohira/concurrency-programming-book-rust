use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

fn main() {
    // TCP の 10000番 ポートをリッスン
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // コネクション要求をアクセプト
    while let Ok((stream, _)) = listener.accept() {
        // 読み込み、書き込みオブジェクトを生成
        let stream0 = stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);
    
        // 1行を読み込んで、同じものを書き込み
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap();
    }

}