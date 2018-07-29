use std::thread;
use std::time::{SystemTime, Duration};
use std::net::UdpSocket;

struct Player {
    id: u8,
    name: String,
}

#[derive(Debug, Default)]
struct Packet {
    packet_type: u8,
    player_id: u8,
    input_x: i8,
    input_z: i8,
    rotation: f32,
}

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
            let mut buf: [u8; 1500] = [0; 1500];
            match socket.recv_from(&mut buf) {
                Ok((number_of_bytes, src_addr)) => {
                    println!("Datagram received!");
                    let packet: Packet = unpack_packet(buf, number_of_bytes);
                    println!("Packet length: {}", number_of_bytes);
                    println!("Player id: {}", packet.player_id);
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

fn unpack_packet(buf: [u8; 1500], number_of_bytes: usize) -> Packet {
    let mut packet = Packet {
        packet_type: buf[0],
        player_id: buf[1],
        ..Default::default()
    };

    match packet.packet_type {
        0 => {
            println!("Login packet!");
        },
        1 => {
            println!("Movement packet!");
            packet.input_x = buf[2];
            packet.input_z = buf[3];
        },
        2 => {
            println!("Rotation packet!")
        },
        3 => println!("Spell packet!"),
        _ => println!("Unknown packet!"),
    };

    packet
}
