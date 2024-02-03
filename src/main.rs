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
                
                let mut temp_files: Vec<String> = vec![];
                match fs::read_dir(&self.folder_path) {
                    Ok(entries) => {
                        for rde in entries {
                            match rde {
                                Ok(de) => {
                                    if let Ok(file_type) = de.file_type() {
                                        if file_type.is_file() { 
                                            if let Some(file_name) = de.file_name().to_str() {
                                                temp_files.push(file_name.to_string())
                                            }
                                        }
                                    }
                                },
                                Err(de_error) => {
                                    println!("{de_error}")
                                },
                            }
                        }
                        self.files = temp_files;
                    },
                    Err(_) => {
                        println!("could not read dir");
                        self.files = vec![];
                    }
                }
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
// let mut temp_files: Vec<String> = vec![];
//                 match fs::read_dir(&self.folder_path) {
//                     Ok(entries) => {
//                         for rde in entries {
//                             match rde {
//                                 Ok(de) => {
//                                     if let Ok(file_type) = de.file_type() {
//                                         if file_type.is_file() { 
//                                             if let Some(file_name) = de.file_name().to_str() {
//                                                 temp_files.push(file_name.to_string())
//                                             }
//                                         }
//                                     }
//                                 },
//                                 Err(_) => continue,
//                             }
//                         }
//                         self.files = temp_files;
//                     },
//                     Err(_) => {
//                         self.files = vec![];
//                     }
//                 }