use std::collections::VecDeque;
use std::env;
use tokio::time::Duration;

use std::sync::mpsc;
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

    let (tx, rx) = mpsc::channel();
    let (tx_server_messages, rx_server_messages) = mpsc::channel();

    thread::spawn(move || {
        let mut command_list: VecDeque<CommandType> = VecDeque::new();

        loop {
            println!("next loop in cmd execution thread");
            dbg!(&command_list);
            let commands: Vec<CommandType> = rx.recv().unwrap();
            for c in commands.iter() {
                command_list.push_back(*c);
            }

            let latest_cmd = command_list.pop_front();
            if let Some(cmd) = latest_cmd {
                match cmd {
                    CommandType::NoNew => println!("No new command - do nothing"),
                    CommandType::Stop => todo!(),
                    CommandType::Resume => todo!(),
                    CommandType::StopAndShutdown => todo!(),
                    CommandType::GetImage => {
                        println!("getting new image");
                        let image = opencv.get_single_image().unwrap();
                        let image_bytes = image.as_slice().to_owned();
                        let sm = ServerMessage {
                            command: cmd,
                            content: String::from("sdfdsf"),
                            binary_content: Some(image_bytes)
                        };
                        tx_server_messages.send(sm).unwrap();
                    }
                    CommandType::GetSourceInfo => {
                        let info = opencv.get_source_info();
                        let sm = ServerMessage {
                            command: cmd,
                            content: info.to_string(),
                            binary_content: None
                        };
                        tx_server_messages.send(sm).unwrap();
                    }
                }
            }

            thread::sleep(Duration::from_millis(300));
        }
    });

    let mut grpc_connector: GRPCConnector = GRPCConnector::new(config.clone());
    grpc_connector.setup_client().await;

    loop {
        let con = &mut grpc_connector;
        con.load_commands().await;
        let received_commands = con.active_commands.clone();
        tx.send(received_commands).unwrap();
        // check if we need to send stuff back to server
        match rx_server_messages.try_recv() {
            Ok(rec) => con.send_to_server(rec).await,
            Err(_) => println!("no messages for server"),
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
