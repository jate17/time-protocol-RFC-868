use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown;
use chrono::{Utc, TimeZone};

fn handle(mut stream: TcpStream)  -> std::io::Result<()> {
    let mut buffer = [0u8; 8];
    let now = Utc::now();
    let time_epoch = Utc.ymd(1900, 1, 1).and_hms(0, 0, 0);
    let time_secs = (now - time_epoch).num_seconds();

    let resp = time_secs.to_be_bytes();

    stream.write_all(&resp[4..]);

    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:37")?;

    for stream in listener.incoming() {

        let mut stream = stream?;
        let peer = stream.peer_addr()?;
        println!("[NEWCON] {:?}", peer);

        handle(stream);
    }

    Ok(())
}


