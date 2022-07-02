#[macro_use]
extern crate include_res;
extern crate oorandom;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rwops::RWops;
use std::io::ErrorKind;
use std::net::{Shutdown, TcpListener};
use std::sync::mpsc::{channel, TryRecvError};

use std::time::Duration;

mod clock;
mod game;
mod home;
mod interface;
mod network;
mod resource;

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
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let texture_creator = canvas.texture_creator();

    // configuration du système audio
    //let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3).unwrap();

    // configuration du système de texte
    let ttf_context = sdl2::ttf::init().unwrap();

    let text_renderer = resource::text::TextRenderer::new(
        ttf_context
            .load_font_from_rwops(
                RWops::from_bytes(include_bytes!("../res/Minecraftia-Regular.ttf")).unwrap(),
                20,
            )
            .unwrap(),
        &texture_creator,
    );

    // chargement des images
    let images = resource::image::Images::load(&texture_creator);

    // chargements de la musique
    //let sounds = audio::Sounds::load();

    // différents écrans
    let mut home = home::Home::new();
    let mut game = game::Game::new();
    // jeu en cours
    let mut ingame = false;

    let listener = TcpListener::bind("127.0.0.1:54321").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut handles: Option<
        network::ThreadHandles<interface::ServerMessage, interface::ClientMessage, ()>,
    > = None;
    let mut threads = network::NetworkThreadBuilder::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let frame_duration = Duration::from_millis(30);
    let mut clock = clock::Clock::new(Duration::from_millis(125)); // corresponds à 120 BPM : 60s / (0,125s * 4 temps) = 120 BPM

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
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
            while clock.tick() {
                if game.tick() {
                    match handles.as_ref().unwrap().rx.try_recv() {
                        Ok(interface::ClientMessage::ConnectionEnded) => {
                            handles = None;
                            ingame = false;
                            break;
                        }
                        Ok(other) => game.react_to_message(other),
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => {
                            panic!("le thread réseau a paniqué de manière inattendue")
                        }
                    }
                }
                match game.response() {
                    Some(message) => handles.as_ref().unwrap().tx.send(message).unwrap(),
                    None => {}
                }
            }
        }

        match listener.accept() {
            Ok((socket, addr)) => {
                if !ingame && home.ready() {
                    // communication dans les deux sens
                    let (tx2, rx1) = channel();
                    let (tx1, rx2) = channel();

                    let join_handle = threads
                        .new_thread()
                        // le nouveau thread prends une extrémité de chaque canal ...
                        .spawn(move || network::handle_client(socket, tx2, rx2))
                        .unwrap();

                    // ... et ce thread prends les autres
                    handles = Some(network::ThreadHandles {
                        rx: rx1,
                        tx: tx1,
                        join_handle,
                    });

                    println!("connecté au client @{}", addr);
                    ingame = true;
                    game.reset(home.seed());
                } else {
                    // refuser la connexion si une partie est déja en cours
                    socket.shutdown(Shutdown::Both).unwrap();
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => println!("impossible de se connecter au client: {}", e),
        }

        canvas.clear();
        if ingame {
            game.draw(&mut canvas, &images, &text_renderer);
        } else {
            home.draw(&mut canvas, &images, &text_renderer);
        }
        canvas.present();

        std::thread::sleep(frame_duration);
    }

    if ingame {
        handles
            .as_ref()
            .unwrap()
            .tx
            .send(interface::ServerMessage::EndConnection)
            .unwrap();
        handles.unwrap().join_handle.join().unwrap();
    }
}
