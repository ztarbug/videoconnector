use std::collections::VecDeque;
use std::env;
use tokio::time::Duration;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[path = "config/config.rs"]
mod config;
use crate::config::parse_config;

#[path = "video_adapter.rs"]
mod video_adapter;

#[path = "opencv/opencv_capture.rs"]
mod opencv_capture;
use crate::opencv_capture::OpencvCapture;

#[path = "grpc/grpc_connector.rs"]
mod grpc_connector;
use crate::grpc_connector::videoconnector::CommandType;
use crate::grpc_connector::GRPCConnector;
use crate::grpc_connector::ServerMessage;

#[tokio::main]
async fn main() {
    let params: Vec<String> = env::args().collect();
    dbg!(&params);

    let config = parse_config(params.get(1));
    println!("loaded config {}", config.video.source);

    let mut opencv = OpencvCapture::new(config.clone());

    let (tx, rx): (Sender<Vec<CommandType>>, Receiver<Vec<CommandType>>) = mpsc::channel();
    let (tx_server_messages, rx_server_messages) = mpsc::channel();
    let (tx_shutdown_sender, rx_shutdown_receiver) = mpsc::channel();
    let (tx_ctrl_send, rx_ctrl_rec) = mpsc::channel();

    ctrlc::set_handler(move || {
        tx_ctrl_send.send(()).expect("sending ctrlc failed");
    })
    .expect("Listening for ctrl+c failed");

    thread::spawn(move || {
        let mut command_list: VecDeque<CommandType> = VecDeque::new();

        loop {
            if command_list.len() > 1 {
                dbg!(&command_list);
            }

            if let Ok(commands) = rx.try_recv() {
                for c in commands.iter() {
                    command_list.push_back(*c);
                }
            }

            let latest_cmd = command_list.pop_front();
            if let Some(cmd) = latest_cmd {
                match cmd {
                    CommandType::NoNew => println!("No new command - do nothing"),
                    CommandType::Stop => {
                        tx_shutdown_sender.send(()).unwrap();
                        break;
                    }
                    CommandType::Resume => todo!(),
                    CommandType::StopAndShutdown => {
                        tx_shutdown_sender.send(()).unwrap();
                        break;
                    }
                    CommandType::GetImage => {
                        println!("getting new image");
                        let image = opencv.get_single_image().unwrap();
                        let image_bytes = image.as_slice().to_owned();
                        let sm = ServerMessage {
                            command: cmd,
                            content: String::from("sdfdsf"),
                            binary_content: Some(image_bytes),
                        };
                        tx_server_messages.send(sm).unwrap();
                    }
                    CommandType::GetSourceInfo => {
                        let info = opencv.get_source_info();
                        let sm = ServerMessage {
                            command: cmd,
                            content: info.to_string(),
                            binary_content: None,
                        };
                        tx_server_messages.send(sm).unwrap();
                    }
                }
            }

            thread::sleep(Duration::from_millis(200));
        }
    });

    let mut grpc_connector: GRPCConnector = GRPCConnector::new(config.clone());
    grpc_connector.setup_client().await;
    match grpc_connector.register_client().await {
        Ok(r) => {
            println!("Client is registered, start receiving {r}");
        }
        Err(e) => {
            println!("registering client failed {e}");
            todo!();
        }
    };

    loop {
        let con = &mut grpc_connector;
        con.load_commands().await;
        let received_commands = con.active_commands.clone();
        if received_commands.len() > 1 {
            dbg!(&received_commands);
        }
        if let Err(e) = tx.send(received_commands) {
            println!("execution thread no longer listening. Stopping: {e}");
        }

        // check if we need to send stuff back to server
        if let Ok(rec) = rx_server_messages.try_recv() {
            con.send_to_server(rec).await;
        };

        if let Ok(()) = rx_shutdown_receiver.try_recv() {
            con.unregister_client().await;
            println!("Shutting down client orderly by command ");
            break;
        }

        if let Ok(()) = rx_ctrl_rec.try_recv() {
            con.unregister_client().await;
            println!("Shutting down client orderly by ctrlc");
            break;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}
