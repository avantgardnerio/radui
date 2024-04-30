use crate::generated::models::Label;

pub mod models;

impl Label {
    pub fn hi(&self) {
        println!("Hello from {}", self.text);
    }
}