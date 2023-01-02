use std::env;
use tokio::time::Duration;

#[path = "config/config.rs"]
mod config;
use crate::config::config::parse_config;

#[path = "v4l/v4l_capture.rs"]
mod v4l_capture;
use crate::v4l_capture::V4LDevice;

#[path = "grpc/grpc_connector.rs"]
mod grpc_connector;
use crate::grpc_connector::GRPCConnector;

#[tokio::main]
async fn main() {
    let params:Vec<String> = env::args().collect();
    dbg!(&params);

    let config = parse_config(params.get(1));
    println!("loaded config {}", config.video.source);
    
    let v4l_device = V4LDevice::new(config.clone());

    v4l_device.print_cam_details();
    v4l_device.save_image().await;


    let mut grpc_connector: GRPCConnector = GRPCConnector::new(config.clone());
    grpc_connector.setup_client().await;

    loop {
        let con = &mut grpc_connector;
        con.load_commands().await;
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
}
