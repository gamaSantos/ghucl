mod base_file_selector;
mod message;

use std::fs;

use iced::widget::{button, column, horizontal_space, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use message::Message;

pub fn main() -> iced::Result {
    Root::run(Settings::default())
}
#[derive(Default)]
struct Root {
    value: i32,
    files: Vec<String>,
    current_base: Option<String>,
    folder_path: String,
}


impl Sandbox for Root {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            files: vec![],
            current_base: None,
            folder_path: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Ghucl")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::FileSelected(file_name) => self.current_base = Some(file_name),
            Message::FolderChanged => {
                println!("folder changed {0}", self.folder_path);
                self.files = match fs::read_dir(&self.folder_path) {
                    Ok(entries) => entries
                        .map(|e| match e {
                            Ok(dir_entry) => dir_entry.path().to_str().unwrap().to_owned(),
                            Err(_) => {
                                String::from("try again later, there was some unexpected io error")
                            }
                        })
                        .collect(),
                    Err(_) => Vec::<String>::new(),
                };
            }
            Message::FolderInputValueChange(value) => self.folder_path = value,
        }
    }

    fn view(&self) -> Element<Message> {
        let folder_component = row![
            text("folder path:"),
            text_input("folder path", &self.folder_path)
                .on_input(Message::FolderInputValueChange)
                .on_paste(Message::FolderInputValueChange)
                .on_submit(Message::FolderChanged),
        ]
        .padding(10);

        let header = row![
            text("choose your destiny"),
            horizontal_space(Length::Fill),
            pick_list(
                &self.files,
                self.current_base.clone(),
                Message::FileSelected
            )
            .placeholder("choose a file")
        ]
        .padding(10);
        column![
            folder_component,
            header,
            // button("Increment").on_press(Message::IncrementPressed),
            // text(self.value).size(50),
            // button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
