use std::time::SystemTime;

use prost_types::Timestamp;
use tonic::transport::Channel;
use videoconnector::CommandList;
use videoconnector::CommandRequest;

use crate::config::ConfigData;
use crate::grpc_connector::videoconnector::video_connector_client::VideoConnectorClient;
use crate::grpc_connector::videoconnector::CommandType;
use crate::grpc_connector::videoconnector::RegisterRequest;
use crate::grpc_connector::videoconnector::UnRegisterRequest;

use self::videoconnector::SourceInfoRequest;
use self::videoconnector::TransferImageRequest;

pub mod videoconnector {
    tonic::include_proto!("videoconnector");
}

#[derive(Clone)]
struct ServerConnectionData {
    pub my_client_id: i32,
}

pub struct GRPCConnector {
    config: ConfigData,
    pub active_commands: Vec<videoconnector::CommandType>,
    client: Option<VideoConnectorClient<Channel>>,
    my_connection_data: Option<ServerConnectionData>,
}

pub struct ServerMessage<T> {
    pub command: videoconnector::CommandType,
    pub content: String,
    pub binary_content: Option<T>,
}

impl GRPCConnector {
    pub fn new(conf: ConfigData) -> Self {
        Self {
            config: conf,
            active_commands: Vec::new(),
            client: None,
            my_connection_data: None,
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

    pub async fn register_client(&mut self) -> Result<bool, &'static str> {
        println!("Registering Client");

        if let Some(ref mut client) = self.client {
            match client
                .register_client(RegisterRequest {
                    hostname: gethostname::gethostname().into_string().unwrap(),
                })
                .await
            {
                Ok(resp) => {
                    let id = resp.into_inner().id;
                    println!("registration successful got server id {}", id);

                    self.my_connection_data = Some(ServerConnectionData { my_client_id: id });

                    Ok(true)
                }
                Err(e) => {
                    println!("Registering Client didn't work, exiting {}", e);
                    Err("Registering Client didn't work")
                }
            }
        } else {
            Err("No working client. Registering Client didn't work")
        }
    }

    pub async fn unregister_client(&mut self) {
        println!("Unregistering Client");
        let id = self.my_connection_data.clone().unwrap().my_client_id;

        if let Some(ref mut client) = self.client {
            match client
                .un_register_client(UnRegisterRequest {
                    hostname: gethostname::gethostname().into_string().unwrap(),
                    client_id: id,
                })
                .await
            {
                Ok(_resp) => {
                    println!("unregistration successful");
                }
                Err(e) => {
                    println!("Registering Client didn't work, exiting {}", e);
                }
            }
        }
    }

    pub async fn load_commands(&mut self) {

        let hostname = gethostname::gethostname().into_string().unwrap();
        let ts = Timestamp::from(SystemTime::now());
        let id = self.my_connection_data.clone().unwrap().my_client_id;

        let req = tonic::Request::new(CommandRequest {
            connector_hostname: hostname,
            client_timestamp: Some(ts),
            client_id: id,
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
                    if command_list.commands.len() > 1 {
                        dbg!(&command_list);
                    }
                }
                Err(e) => println!("Getting commands failed {}", e),
            };
        }
    }

    pub async fn send_to_server(&mut self, server_message: ServerMessage<Vec<u8>>) {
        if server_message.command == CommandType::GetImage {
            self.send_image(server_message.binary_content).await;
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

    pub async fn send_image(&mut self, i: Option<Vec<u8>>) {
        let id = self.my_connection_data.clone().unwrap().my_client_id;
        let ts = Timestamp::from(SystemTime::now());
        let image_bytes = i.unwrap();
        let b64_message = base64::encode(&image_bytes);
        println!("Sending image with size {}", &image_bytes.len());

        let req = tonic::Request::new(TransferImageRequest {
            client_timestamp: Some(ts),
            client_id: id,
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
