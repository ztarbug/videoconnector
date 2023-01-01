use std::io;
use std::env;

#[path = "config/config.rs"]
mod config;
use crate::config::config::parse_config;

#[path = "v4l/v4l_capture.rs"]
mod v4l_capture;
use crate::v4l_capture::V4LDevice;

#[tokio::main]
async fn main() -> io::Result<()> {
    let params:Vec<String> = env::args().collect();
    dbg!(&params);

    let config = parse_config(params.get(1));
    println!("loaded config {}", config.video.source);
    
    let v4l_device = V4LDevice::new(config.clone());

    v4l_device.print_cam_details();
    v4l_device.save_image().await;

    Ok(())
}
