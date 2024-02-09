use std::fs::ReadDir;

use iced::{widget::pick_list, Element};

use crate::message::Message;

#[derive()]
struct BaseFileSelector {
    dir: Option<ReadDir>,
    on_select: fn(String),
    selected: Option<String>,
    files: Vec<String>
}

impl BaseFileSelector {
    pub fn new(dir: Option<ReadDir>, callback: fn(String)) -> Self {
        Self {
            dir: dir,
            on_select: callback,
            selected: None,
            files: vec![]
        }
    }

    pub fn view(&self) -> Element<Message> {
        pick_list(
            &self.files,
            self.selected.clone(),
            Message::BaseFileChanged
        ).into()
    }
}