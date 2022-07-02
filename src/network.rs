use crate::interface::{ClientMessage, ServerMessage};
use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::{Builder, JoinHandle};
use std::time::Duration;

pub fn handle_client(
    mut socket: TcpStream,
    tx: Sender<ClientMessage>,
    rx: Receiver<ServerMessage>,
) {
    socket.set_nonblocking(false).unwrap(); // Windows crée des sockets non-bloquants par défaut
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    socket.write(b"D\xc3\x89BUT\n").unwrap();

    let mut msg = Vec::new();

    while match read_line(&mut socket, &mut msg) {
        Ok(size) => {
            if size == -1 {
                println!(
                    "le client @{} a mis fin à la connexion",
                    get_peer_address(&socket),
                );
                socket.shutdown(Shutdown::Both).unwrap();
                tx.send(ClientMessage::ConnectionEnded {}).unwrap();
                false
            } else {
                tx.send(ClientMessage::from_bytes(&msg)).unwrap();

                match rx.recv() {
                    Err(_) | Ok(ServerMessage::EndConnection {}) => {
                        println!("fermeture de la connexion @{}", get_peer_address(&socket));
                        socket.shutdown(Shutdown::Both).unwrap_or(());
                        false
                    }
                    Ok(msg) => match write_line(&mut socket, &msg.into_bytes()) {
                        Ok(_) => true,
                        Err(err) => {
                            println!(
                                "une erreur est survenue, fermeture de la connexion @{} ({})",
                                get_peer_address(&socket),
                                err,
                            );
                            socket.shutdown(Shutdown::Both).unwrap_or(());
                            tx.send(ClientMessage::ConnectionEnded {}).unwrap();
                            false
                        }
                    },
                }
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::WouldBlock | ErrorKind::TimedOut => match rx.try_recv() {
                Err(TryRecvError::Empty) => true,
                Err(TryRecvError::Disconnected) | Ok(ServerMessage::EndConnection {}) => {
                    println!("fermeture de la connexion @{}", get_peer_address(&socket));
                    socket.shutdown(Shutdown::Both).unwrap_or(());
                    false
                }
                Ok(msg) => match write_line(&mut socket, &msg.into_bytes()) {
                    Ok(_) => true,
                    Err(err) => {
                        println!(
                            "une erreur est survenue, fermeture de la connexion @{} ({})",
                            get_peer_address(&socket),
                            err,
                        );
                        socket.shutdown(Shutdown::Both).unwrap_or(());
                        tx.send(ClientMessage::ConnectionEnded {}).unwrap();
                        false
                    }
                },
            },
            _ => {
                println!(
                    "une erreur est survenue, fermeture de la connexion @{} ({})",
                    get_peer_address(&socket),
                    err,
                );
                socket.shutdown(Shutdown::Both).unwrap_or(());
                tx.send(ClientMessage::ConnectionEnded {}).unwrap();
                false
            }
        },
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

fn write_line<W: Write>(stream: &mut W, input: &[u8]) -> Result<usize, std::io::Error> {
    let msg_size = stream.write(input)?;
    let lf_size = stream.write(&[10])?;

    Ok(msg_size + lf_size)
}

pub struct NetworkThreadBuilder {
    last_discriminant: u32,
}

impl NetworkThreadBuilder {
    pub fn new() -> Self {
        Self {
            last_discriminant: 0,
        }
    }

    pub fn new_thread(&mut self) -> Builder {
        self.last_discriminant += 1;
        Builder::new().name(format!("network-{}", self.last_discriminant))
    }
}

pub struct ThreadHandles<S, R, T> {
    pub tx: Sender<S>,
    pub rx: Receiver<R>,
    pub join_handle: JoinHandle<T>,
}

fn get_peer_address(socket: &TcpStream) -> String {
    match socket.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "<inconnu>".to_string(),
    }
}
