use v4l::Device;
use v4l::video::Capture;
use v4l::FourCC;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;

use crate::config::config::ConfigData;

pub struct V4LDevice {
    pub device: Device,
    config: ConfigData,
}

impl V4LDevice {
    pub fn new(conf:ConfigData) -> Self {
        let device_id = conf.video.source.clone();
        Self{
            device: Device::new(device_id).expect("Failed to open device"),
            config: conf,
        }
    }

    pub async fn save_image(mut self) {

        let mut stream = Stream::with_buffers(&mut self.device, Type::VideoCapture, 4)
            .expect("Failed to create buffer stream");
    
        let (buf, meta) = stream.next().unwrap();
        println!(
            "Buffer size: {}, seq: {}, timestamp: {}",
            buf.len(),
            meta.sequence,
            meta.timestamp
        );

        self.write_image(&buf);
    }
    
    fn write_image(&self, buf: &[u8]) {
    
        let img = image::load_from_memory(&buf).unwrap();
        let storage_path = format!("{}.jpg", "/tmp/image");
        img.save(storage_path).expect("Could not write frame");
    }
    
    pub fn print_cam_details(&self) {
    
        let mut fmt = self.device.format().expect("Failed to read format");
        fmt.width = 1920;
        fmt.height = 1080;
        fmt.fourcc = FourCC::new(b"MJPG");
        self.device.set_format(&fmt).expect("Failed to write format");
    
        println!("Format in use:\n{}", fmt);
    
        let params = self.device.params().expect("Couldn't get params");
        println!("Active parameters:\n{}", params); 
    
        println!("Available formats:");
    
        let format_description = self.device.enum_formats().expect("Can't get device supported format");
    
        for format in format_description {
            println!("  {} ({})", format.fourcc, format.description);
    
            for framesize in self.device.enum_framesizes(format.fourcc)
                .expect("Can't get framesizes") {
                for discrete in framesize.size.to_discrete() {
                    println!("    Size: {}", discrete);
                    for frameinterval in
                    self.device.enum_frameintervals(framesize.fourcc, discrete.width, discrete.height)
                            .expect("Can't load frame intervals")
                    {
                        println!("      Interval:  {}", frameinterval);
                    }
                }
            }
            println!()
        }
    }    
}



