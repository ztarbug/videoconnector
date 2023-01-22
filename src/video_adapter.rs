use crate::config::ConfigData;

pub struct SourceInfo {
    pub name: String,
}

impl SourceInfo {
    pub fn to_string(&self) -> String {
        let mut result = String::from("");
        result.push_str(&self.name);
        return result;
    }
}

pub trait VideoSource {
    fn new(conf: ConfigData) -> Self;
    fn get_single_image(&self);
    fn get_source_info(&self) -> SourceInfo;
}