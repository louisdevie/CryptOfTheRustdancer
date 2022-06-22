#[macro_use]
extern crate include_res;
extern crate oorandom;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rwops::RWops;
use std::io::ErrorKind;
use std::net::{Shutdown, TcpListener};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

mod game;
mod home;
mod image;
mod interface;
mod map;
mod network;
mod pos;
mod text;

use interface::{ClientMessage, ServerMessage};

pub fn main() {
    // initialisation de SDL2
    let sdl_context = sdl2::init().unwrap();

    // configuration du système vidéo
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Crypt Of The RustDancer", 1440, 810)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // configuration du système de texte
    let ttf_context = sdl2::ttf::init().unwrap();

    let text_renderer = text::TextRenderer::new(
        ttf_context
            .load_font_from_rwops(
                RWops::from_bytes(include_bytes!("../res/Minecraftia-Regular.ttf")).unwrap(),
                20,
            )
            .unwrap(),
        &texture_creator,
    );

    // chargement des images
    let images = image::ImageBank::load(&texture_creator);

    // différents écrans
    let mut home = home::Home::new();
    let mut game = game::Game::new();
    // jeu en cours
    let mut ingame = false;

    let listener = TcpListener::bind("127.0.0.1:54321").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut tx: Option<Sender<ServerMessage>> = None;
    let mut rx: Option<Receiver<ClientMessage>> = None;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let frame_duration = Duration::new(0, 1_000_000_000u32 / 30);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                ev => {
                    if ingame {
                        game.handle_event(ev);
                    } else {
                        home.handle_event(ev);
                    }
                }
            }
        }

        if ingame {
            for msg in rx.as_ref().unwrap().try_iter() {
                match msg {
                    ClientMessage::ConnectionEnded {} => {
                        tx = None;
                        rx = None;
                        ingame = false;
                        break;
                    }
                    other => tx
                        .as_ref()
                        .unwrap()
                        .send(game.react_to_message(other))
                        .unwrap(),
                }
            }
        }

        match listener.accept() {
            Ok((socket, addr)) => {
                if !ingame {
                    println!("connected to client at {}", addr);

                    let (tx2, rx1) = channel();
                    let (tx1, rx2) = channel();

                    tx = Some(tx1);
                    rx = Some(rx1);

                    thread::spawn(move || network::handle_client(socket, tx2, rx2));
                    ingame = true;
                    game.reset(0);
                } else {
                    socket.shutdown(Shutdown::Both).unwrap();
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => println!("couldn't connect to client: {}", e),
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if ingame {
            game.draw();
        } else {
            home.draw(&mut canvas, &images, &text_renderer);
        }
        canvas.present();

        std::thread::sleep(frame_duration);
    }
}
