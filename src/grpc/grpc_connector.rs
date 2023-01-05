use std::time::SystemTime;

use prost_types::Timestamp;
use tonic::transport::Channel;
use videoconnector::CommandList;
use videoconnector::CommandRequest;

use crate::config::ConfigData;
use crate::grpc_connector::videoconnector::video_connector_client::VideoConnectorClient;
use crate::grpc_connector::videoconnector::CommandType;

use self::videoconnector::SourceInfoRequest;
use self::videoconnector::TransferImageRequest;

pub mod videoconnector {
    tonic::include_proto!("videoconnector");
}

pub struct GRPCConnector {
    config: ConfigData,
    pub active_commands: Vec<videoconnector::CommandType>,
    client: Option<VideoConnectorClient<Channel>>,
}

pub struct ServerMessage {
    pub command: videoconnector::CommandType,
    pub content: String,
}

impl GRPCConnector {
    pub fn new(conf: ConfigData) -> Self {
        Self {
            config: conf,
            active_commands: Vec::new(),
            client: None,
        }
    }

    pub async fn setup_client(&mut self) {
        let config = &self.config;
        let url: String = config.backend.url.clone();

        let tmp_client = VideoConnectorClient::connect(url)
            .await
            .expect("couldn't build client stub");

        self.client = Some(tmp_client);
    }

    pub async fn load_commands(&mut self) {
        println!("loading commands from server");

        let hostname = String::from("localhost");

        let ts = Timestamp::from(SystemTime::now());

        let req = tonic::Request::new(CommandRequest {
            connector_hostname: hostname,
            client_timestamp: Some(ts),
        });
        let command_list: CommandList;

        if let Some(ref mut client) = self.client {
            match client.get_command(req).await {
                Ok(resp) => {
                    command_list = resp.into_inner();
                    self.active_commands = Vec::new();
                    let commands = &command_list.commands;
                    for c in commands.iter() {
                        let ct = CommandType::from_i32(*c);
                        self.active_commands.push(ct.unwrap());
                    }

                    dbg!(command_list);
                }
                Err(e) => println!("Getting commands failed {}", e),
            };
        }
    }

    pub async fn send_to_server(&mut self, server_message: &ServerMessage) {
        if server_message.command == CommandType::GetImage {
            self.send_image(&server_message.content).await;
        }
        if server_message.command == CommandType::GetSourceInfo {
            self.send_source_info(&server_message.content).await;
        }
    }

    pub async fn send_source_info(&mut self, info: &String) {
        let ts = Timestamp::from(SystemTime::now());
        let message = String::from(info);
        let req = tonic::Request::new(SourceInfoRequest {
            client_timestamp: Some(ts),
            source_info: message,
        });

        if let Some(ref mut client) = self.client {
            match client.deliver_source_info(req).await {
                Ok(resp) => {
                    dbg!(resp);
                }
                Err(e) => println!("Sending source info failed {}", e),
            };
        }
    }

    pub async fn send_image(&mut self, image: &String) {
        let ts = Timestamp::from(SystemTime::now());
        let message = String::from(image);
        let b64_message = base64::encode(&message);
        println!("Sending image with size {}", &message.len());
        let req = tonic::Request::new(TransferImageRequest {
            client_timestamp: Some(ts),
            camera_id: 1,
            image: b64_message.into(),
        });

        if let Some(ref mut client) = self.client {
            match client.transfer_image(req).await {
                Ok(resp) => {
                    dbg!(resp);
                }
                Err(e) => println!("Transfer image failed {}", e),
            };
        }
    }
}
