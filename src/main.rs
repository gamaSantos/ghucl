mod base_file_selector;
mod file_tree;
mod message;
mod request_error;
mod request_message;

use std::fs;

use file_tree::FileTree;
use iced::widget::{
    button, column, horizontal_space, pick_list, row, scrollable, text, text_input,
};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use message::Message;
use request_error::RequestError;

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
    file_builder: Option<request_message::RequestMessageBuilder>,
    message: Option<request_message::RequestMessage>,
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
            file_builder: None,
            message: None,
        }
    }

    fn title(&self) -> String {
        String::from("Ghucl")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::BaseFileChanged(file_name) => {
                self.current_base = Some(file_name.clone());
                let rmb_read = Root::read_file(&file_name)
                    .and_then(|toml_text| request_message::RequestMessage::from_text(&toml_text));
                self.base_builder = match rmb_read {
                    Ok(rmb) => Some(rmb),
                    Err(_) => {
                        self.notify("could not read base file");
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
                        self.file_builder =
                            match request_message::RequestMessage::from_text(&content) {
                                Ok(rmb) => Some(rmb),
                                Err(_) => {
                                    self.notify("could not parse file");
                                    None
                                }
                            };
                        self.file_content = content
                    }
                    Err(_) => self.notify("Could not read file"),
                };
            }
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
            button("send")
        ]
        .padding(10)
        .align_items(Alignment::Center)
        .spacing(20);

        let tree_view = scrollable(match &self.file_tree {
            Some(fs) => fs.get_elements(),
            None => column![text("no item")].into(),
        })
        .width(Length::FillPortion(1));

        let request_view = scrollable(column![text(&self.file_content), text(&self.file_content),])
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
    fn read_file(file_path: &str) -> Result<String, RequestError> {
        match fs::read_to_string(file_path) {
            Ok(v) => Ok(v),
            Err(_) => Err(RequestError::CouldNotReadFile),
        }
    }
}
