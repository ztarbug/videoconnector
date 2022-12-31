use std::io;
use std::env;

#[path = "config/config.rs"]
mod config;
use crate::config::config::parse_config;

#[path = "v4l/v4l_capture.rs"]
mod v4l_capture;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::Device;

#[tokio::main]
async fn main() -> io::Result<()> {
    let params:Vec<String> = env::args().collect();
    dbg!(&params);

    let config = parse_config(params.get(1));
    println!("loaded config {}", config.video.source);

    let mut dev = Device::new(0).expect("Failed to open device");

    v4l_capture::print_cam_details(&dev);

    let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 4)
        .expect("Failed to create buffer stream");

    let (buf, meta) = stream.next().unwrap();
    println!(
        "Buffer size: {}, seq: {}, timestamp: {}",
        buf.len(),
        meta.sequence,
        meta.timestamp
    );
    write_image(&buf).await;

    Ok(())
}

async fn write_image(buf: &[u8]) {

    let img = image::load_from_memory(&buf).unwrap();
    let storage_path = format!("{}.jpg", "/tmp/image");
    img.save(storage_path).expect("Could not write frame");
}
