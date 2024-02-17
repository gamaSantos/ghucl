mod file_tree;
mod http_client;
mod message;
mod request_error;
mod request_message;
mod response_message;

use std::fs;
use std::path::Path;

use async_std::task;
use file_tree::FileTree;
use iced::widget::{
    button, column, horizontal_space, pick_list, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use message::Message;
use request_error::RequestError;
use request_message::RequestMessageBuilder;

pub fn main() -> iced::Result {
    Root::run(Settings::default())
}

#[derive(Default)]
struct Root {
    files: Vec<String>,
    current_base: Option<String>,
    folder_path: String,
    file_content: String,
    file_tree: Option<FileTree>,
    reponse: String,
    base_builder: Option<request_message::RequestMessageBuilder>,
    req_builder: Option<request_message::RequestMessageBuilder>,
    req_content: String,
}

impl Sandbox for Root {
    type Message = Message;

    fn new() -> Self {
        Self {
            files: vec![],
            current_base: None,
            folder_path: String::from(""),
            file_tree: None,
            file_content: String::from("no file selected"),
            reponse: String::from("empty for now"),
            base_builder: None,
            req_builder: None,
            req_content: String::from("[none]"),
        }
    }

    fn title(&self) -> String {
        String::from("Ghucl")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::BaseFileChanged(file_name) => {
                self.current_base = Some(file_name.clone());
                let base_path = Path::new(&self.folder_path).join(&file_name);
                let full_path = base_path.to_str().unwrap_or("");

                self.base_builder = match Root::get_builder_from_file(full_path) {
                    Ok(rmb) => Some(rmb),
                    Err(_) => {
                        self.notify(format!("could not read base file in {0}", full_path).as_str());
                        None
                    }
                };
            }
            Message::FolderChanged => {
                println!("folder changed {0}", self.folder_path);
                let result = FileTree::from_path(&self.folder_path).and_then(|tree| {
                    self.files = tree.get_file_names();
                    self.file_tree = Some(tree);
                    Ok(())
                });
                match result {
                    Ok(_) => {}
                    Err(_) => {
                        println!("could not read dir"); //change alert
                        self.files = vec![];
                        self.file_tree = None;
                    }
                }
            }
            Message::FolderInputValueChange(value) => self.folder_path = value,
            Message::FileTreeItemToogled(path) => {
                if let Some(tree) = self.file_tree.as_mut() {
                    tree.navigate(&path);
                };
            }
            Message::FileSelected(file_path) => {
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        self.req_builder =
                            match request_message::RequestMessage::from_text(&content) {
                                Ok(rmb) => {
                                    if let Some(base_buiilder) = &self.base_builder {
                                        let req = base_buiilder.merge_with(&rmb);
                                        self.req_content = format!("{req}");    
                                        Some(req)
                                    }
                                    else {
                                        self.req_content = format!("{rmb}");
                                        Some(rmb)
                                    }
                                }
                                Err(_) => {
                                    self.notify("could not parse file");
                                    None
                                }
                            };
                        self.file_content = content;
                    }
                    Err(_) => self.notify("Could not read file"),
                };
            }
            Message::Send => match &self.req_builder {
                Some(req_builder) => match req_builder.to_message() {
                    Ok(message) => {
                        match task::block_on(http_client::send(message)) {
                            Ok(respone) => self.reponse = format!("{0}", respone),
                            Err(_) => self.notify("error while sending request"),
                        };
                    }
                    Err(_) => self.notify("error while sending request"),
                },
                None => self.notify("Could not send the message"),
            },
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
        .padding(10)
        .align_items(Alignment::Center)
        .spacing(10);

        let header = row![
            text("choose base file"),
            pick_list(
                &self.files,
                self.current_base.clone(),
                Message::BaseFileChanged
            )
            .placeholder("choose a file"),
            horizontal_space(Length::Fill),
            button("send").on_press(Message::Send)
        ]
        .padding(10)
        .align_items(Alignment::Center)
        .spacing(20);

        let tree_view = scrollable(match &self.file_tree {
            Some(fs) => fs.get_elements(),
            None => column![text("no item")].into(),
        })
        .width(Length::FillPortion(1));

        let request_view =
            scrollable(column![text(&self.file_content), text(&self.req_content),].spacing(10))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .direction(scrollable::Direction::Both {
                    vertical: scrollable::Properties::default(),
                    horizontal: scrollable::Properties::default(),
                });

        let result_view = scrollable(text(&self.reponse))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Properties::default(),
                horizontal: scrollable::Properties::default(),
            });

        let content_row = row![tree_view, request_view, result_view].spacing(20);
        column![folder_component, header, content_row,]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}

impl Root {
    fn notify(&mut self, message: &str) {
        // TODO implement actual notification
        self.reponse = message.to_string();
    }

    // replace with actual implementation
    fn get_builder_from_file(file_path: &str) -> Result<RequestMessageBuilder, RequestError> {
        match fs::read_to_string(file_path) {
            Ok(v) => Ok(v),
            Err(_) => Err(RequestError::CouldNotReadFile),
        }
        .and_then(|toml_text| request_message::RequestMessage::from_text(&toml_text))
    }
}
