use v4l::Device;
use v4l::video::Capture;
use v4l::FourCC;

pub fn print_cam_details(dev:&Device) {

    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = 1920;
    fmt.height = 1080;
    fmt.fourcc = FourCC::new(b"MJPG");
    dev.set_format(&fmt).expect("Failed to write format");

    println!("Format in use:\n{}", fmt);

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