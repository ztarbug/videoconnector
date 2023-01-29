use crate::config::ConfigData;
use crate::video_adapter::SourceInfo;

use opencv::{core::*, imgcodecs::*, prelude::*, videoio::*};

pub struct OpencvCapture {
    config: ConfigData,
    my_capture: VideoCapture,
}

impl OpencvCapture {
    pub fn new(conf: ConfigData) -> Self {
        let source = conf.video.source;
        let cam = VideoCapture::new(source, CAP_ANY).unwrap();

        OpencvCapture {
            config: conf,
            my_capture: cam,
        }
    }

    pub fn get_single_image(&mut self) -> Option<Vector<u8>> {
        let mut frame = Mat::default();

        self.my_capture.read(&mut frame).unwrap();
        self.save_image(&frame);

        //let bytes = frame.data_bytes().unwrap().to_vec();

        let mut jpg_buf: Vector<u8> = Vector::new();
        let params: Vector<i32> = Vector::new();

        match imencode(".jpg", &frame, &mut jpg_buf, &params) {
            Ok(b) => {
                println!("Encode result {} with size {}", b, jpg_buf.len());
                Some(jpg_buf)
            }
            Err(e) => {
                println!("Converting to image didn't work: {e}");
                None
            }
        }
    }

    fn save_image(&self, img_buf: &Mat) {
        let mut params: Vector<i32> = Vector::new();
        params.push(IMWRITE_PNG_COMPRESSION);
        params.push(9);

        imwrite("test.png", img_buf, &params).expect("couldn't write image");
    }

    pub fn get_source_info(&self) -> SourceInfo {
        let mut info: String = String::from("OpenCV Source ");
        info.push_str(&format!("Source type: {}", &self.config.video.src_type));
        info.push_str(&format!("Device id: {}", &self.config.video.source));
        SourceInfo { name: info }
    }
}
