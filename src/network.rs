use crate::interface::{ClientMessage, ServerMessage};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{Receiver, Sender};

pub fn handle_client(
    mut socket: TcpStream,
    tx: Sender<ClientMessage>,
    rx: Receiver<ServerMessage>,
) {
    let mut msg = Vec::new();

    while match read_line(&mut socket, &mut msg) {
        Ok(size) => {
            if size == -1 {
                println!(
                    "client at {} ended the connection",
                    socket.peer_addr().unwrap()
                );
                socket.shutdown(Shutdown::Both).unwrap();
                tx.send(ClientMessage::ConnectionEnded {}).unwrap();
                false
            } else {
                tx.send(ClientMessage::from_bytes(&msg)).unwrap();

                socket.write(&rx.recv().unwrap().into_bytes()).unwrap();
                socket.write(&[10]).unwrap();
                true
            }
        }
        Err(_) => {
            println!(
                "an error occurred, terminating connection with {}",
                match socket.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => "<unknown>".to_string(),
                }
            );
            socket.shutdown(Shutdown::Both).unwrap_or(());
            tx.send(ClientMessage::ConnectionEnded {}).unwrap();
            false
        }
    } {}
}

fn read_line<R: Read>(stream: &mut R, output: &mut Vec<u8>) -> Result<isize, std::io::Error> {
    let mut buffer = [0 as u8; 50];
    output.clear();

    'l: loop {
        let size = stream.read(&mut buffer)?;

        if size == 0 {
            return Ok(-1);
        } else {
            output.extend_from_slice(&buffer[0..size]);
            if buffer[size - 1] == 10 {
                output.pop().unwrap();
                if let Some(c) = output.last() {
                    if *c == 13 {
                        output.pop().unwrap();
                    }
                }
                break 'l;
            }
        }
    }

    Ok(output.len().try_into().unwrap())
}
