// Rust OSC library import
extern crate rosc;

// ROSC encoder import
use rosc::encoder;
// ROSC types (Message types / Packet types / ROSC arg types
use rosc::{OscMessage, OscPacket, OscType};
// IPv4 Address / Udp Socket object
use std::net::{SocketAddrV4, UdpSocket};
// From String Trait
use std::str::FromStr;
// Duration struct
use std::time::Duration;
// Proc Environment / 32 bit float / thread lib
use std::{env, f32, thread};

// Send OSC data
fn send_data(sock: &UdpSocket, param_name: &str, param_arg: OscType) {

    // Create OSC/1.0 Message buffer with parameter name and parameter value/arg
    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: param_name.to_string(),
        args: vec![param_arg],
    }))
    .unwrap();

    // Sends the encoded Message buffer to VRChat on port 9000
    sock.send_to(&msg_buf, "127.0.0.1:9000").unwrap();
}

fn recv_data(sock: &UdpSocket) {

    // Create/allocate buffer on the stack with a size of MTU
    let mut buf = [0u8; rosc::decoder::MTU];
    
    // Continuously read OSC data from port 9001.
    loop {
        /*
            Receive OSC data length in var "br". Address of origin data in "a".
            Writes the data received to the buffer on the stack "buf".
        */
        let (br, a) = sock.recv_from(&mut buf).unwrap();

        //println!("{:?}");
        /*
            Checks that the packet is greater than 0 bytes.
            If the packet length is <= 0 then the recv loop is restarted.
            The received buffer is then decoded and parsed.
            If the decoded packet "pkt" is of OSC type Message
            the OSC address and OSC args are printed to the CLI.
        */
        if br <= 0 {
            continue;
        } else {
            //println!("{:?}", buf);
            let pkt = match rosc::decoder::decode_udp(&buf) {
                Ok(pkt) => pkt,
                Err(_e) => {
                    println!("{}", "[-] Invalid OSC buffer.");
                    continue;
                },
            };
            match pkt.1 {
                OscPacket::Message(msg) => {
                    println!("OSC ADDRESS: {}", msg.addr);
                    println!("OSC ARGS: {:?}", msg.args);
                    //break;
                },
                _ => {}
            }
        }
    }
}

fn main() {

    /*
        Binds/creates a UDP socket to port 9001 to be used for communication with VRChat.
        VRChat binds to UDP port 9000.
    */
    let sock = UdpSocket::bind(format!("127.0.0.1:9001")).unwrap();

    /*
        Sends a parameter and value to VRChat on port 9000
        This example sends a Boolean type with value false
        to the avatar parameter DressT. This instantly sets
        the current loaded avatar's DressT parameter to false.
    */
    //send_data(&sock, "/avatar/parameters/DressT", OscType::Bool(false));
    
    /*
        Receives driven parameter data from port 9001 from VRChat.
    */
    recv_data(&sock);
}