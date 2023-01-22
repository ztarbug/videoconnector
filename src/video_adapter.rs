use crate::config::ConfigData;
use std::fmt;

pub struct SourceInfo {
    pub name: String,
}

impl fmt::Display for SourceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.name)
    }
}

pub trait VideoSource {
    fn new(conf: ConfigData) -> Self;
    fn get_single_image(&self);
    fn get_source_info(&self) -> SourceInfo;
}
