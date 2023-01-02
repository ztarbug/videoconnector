
use std::time::SystemTime;
use std::time::Duration;

use tonic::transport::Channel;
use videoconnector::CommandRequest;
use videoconnector::CommandList;

use crate::config::config::ConfigData;
use crate::grpc_connector::videoconnector::video_connector_client::VideoConnectorClient;

pub mod videoconnector {
    tonic::include_proto!("videoconnector");
}

pub struct Command {
    server_command: videoconnector::CommandType,
}

pub struct GRPCConnector {
    config: ConfigData,
    active_commands: Vec<Command>,
    client: Option<VideoConnectorClient<Channel>>,
}

impl GRPCConnector {

    pub fn new(conf: ConfigData) -> Self {
        Self{
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
        println!("getting commands from server");
    
        let hostname = String::from("localhost");
        
        let now = SystemTime::now();
        let ts: Duration = now.duration_since(SystemTime::UNIX_EPOCH).expect("");
    
        let req = tonic::Request::new(CommandRequest{connector_hostname:hostname, client_timestamp: ts.as_secs().to_string()});
        let command_list: CommandList;

        if let Some(ref mut client) = self.client {
            match client.get_command(req).await {
                Ok(resp) => {
                        command_list =  resp.into_inner();
                        dbg!(command_list);
                    },
                Err(e) => println!("Getting commands failed {}", e),
            };
        }
    }

}

