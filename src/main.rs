use nix::sys::epoll::{
    epoll_create1, epoll_ctl, epoll_wait,EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
};

use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::StepBy;
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd, RawFd};

fn main (){
    // epollのフラグの短縮形
    let epoll_in = EpollFlags::EPOLLIN;
    let epoll_add =  EpollFlags::EpollCtlAdd;
    let epoll_del = EpollOp::EpollCtDel;

    // TCPの10000番ポートをリッスン
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // epoll用のオブジェクトを作成
    // https://manpages.ubuntu.com/manpages/bionic/ja/man2/epoll_create.2.html
    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap();

    // リッスン用のソケットを監視対象に追加
    let listen_fd = listener.as_raw_fd();
    let mut ev = EpollEvent::new(epoll_in, listen_fd as u64);
    epoll_ctl(epfd, epoll_add, listen_fd, &mut ev).unwrap();

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty; 1024];

    // epoll で イベント発生を監視
    while let Ok(nfds) = epoll_wait(epfd, &mut events, -1){
        for n in 0..nfds {
            // リッスンソケットにイベント ⑤
            if let Ok((stream, _)) = listener.accept(){
                // 読み込み、書き込みオブジェクトを生成
                let fd = stream.as_raw_fd();
                let stream0 =stream.try_clone().unwrap();

                let reader = BufReader::new(stream0);
                let writer = BufWriter::new(stream);

                // fd と reader および writer を関連付け
                fd2buf.insert(fd, (reader, writer));

                println!("accceptだよ: fd = {}", fd);

                // fdを監視対象に登録する
                let mut ev = EpollEvent::new(epoll_in, fd as u64);
                epoll_ctl(epfd, epoll_add, fd, &mut ev).unwrap();

            } else {
                // クライアントからデータ到着⑥
                let fd = events[n].data() as RawFd;
                let (reader, writer) = fd2buf.get_mut(&fd).unwrap();

                // 1行読み込み
                let mut buf = String::new();
                let n = reader.read_line(&mut buf).unwrap();

                // コネクションクローズした場合、epollの監視対象から外す
                if n == 0 {
                    let mut ev = EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_del, fd, &mut ev).unwrap();
                    fd2buf.remove(&fd);
                    println!("closed: fd = {}", fd);
                    continue;
                }

                print!("read: fd = {}, buf = {}", fd, buf);

                // 読み込んだデータをそのまま書き込み
                writer.write(buf.as_bytes()).unwrap();
                writer.flush().unwrap()
            }
        }
        
    }


}
