use std::io;

#[path = "config/config.rs"]
mod config;
use crate::config::config::parse_config;


use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

#[tokio::main]
async fn main() -> io::Result<()> {

    let config = parse_config();
    println!("loaded config {}", config.video.source);

    let mut dev = Device::new(0).expect("Failed to open device");

    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = 1920;
    fmt.height = 1080;
    fmt.fourcc = FourCC::new(b"MJPG");
    dev.set_format(&fmt).expect("Failed to write format");

    println!("Format in use:\n{}", fmt);

    get_cam_details(&dev);

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

fn get_cam_details(dev:&Device) {

    let params = dev.params().expect("Couldn't get params");
    println!("Active parameters:\n{}", params); 

    println!("Available formats:");

    let format_description = dev.enum_formats().expect("Can't get device supported format");

    for format in format_description {
        println!("  {} ({})", format.fourcc, format.description);

        for framesize in dev.enum_framesizes(format.fourcc)
            .expect("Can't get framesizes") {
            for discrete in framesize.size.to_discrete() {
                println!("    Size: {}", discrete);
                for frameinterval in
                    dev.enum_frameintervals(framesize.fourcc, discrete.width, discrete.height)
                        .expect("Can't load frame intervals")
                {
                    println!("      Interval:  {}", frameinterval);
                }
            }
        }
        println!()
    }

}

async fn write_image(buf: &[u8]) {

    let img = image::load_from_memory(&buf).unwrap();
    let storage_path = format!("{}.jpg", "/tmp/image");
    img.save(storage_path).expect("Could not write frame");
}
