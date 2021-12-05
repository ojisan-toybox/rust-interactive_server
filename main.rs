use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    while let Ok((stream, _)) = listener.accept() {
        let stream0 = stream.try_clone().unwrap();
        
        // Q: ここって両方とも &stream じゃだめなの？
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        let mut buf = String::new();

        // buf に文字列を読み込む
        reader.read_line(&mut buf).unwrap();

        // stream へ buffer を書き出す
        writer.write(buf.as_bytes()).unwrap();

        // 目的のアドレスへ送信する
        writer.flush().unwrap();
    }
}
