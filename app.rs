use std::thread;
use std::time::{SystemTime, Duration};
use std::net::UdpSocket;

fn main() {
    println!("Server started...");

    // Bind UDP socket
    let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();
    socket.set_nonblocking(true).unwrap();

    // Start game time
    let game_time = SystemTime::now();

    // Start game loop
    'game: loop {

        // Start loop timer
        let loop_time = SystemTime::now();

        // Handle all received datagrams
        'udp: loop {
            let mut buf = [0; 1500];
            match socket.recv_from(&mut buf) {
                Ok((number_of_bytes, src_addr)) => {
                    println!("Player joined!");
                },
                Err(_) => {
                    break 'udp;
                }
            }
        }

        // Sleep before next round
        thread::sleep(Duration::from_millis(500 - loop_time.elapsed().unwrap().subsec_nanos() as u64 / 1000000));
    }
}
