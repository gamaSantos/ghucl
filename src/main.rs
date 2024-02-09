mod base_file_selector;
mod file_tree;
mod message;

use std::fs;

use file_tree::FileTree;
use iced::alignment::Horizontal;
use iced::widget::{button, column, horizontal_space, pick_list, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use message::Message;

pub fn main() -> iced::Result {
    Root::run(Settings::default())
}
#[derive(Default)]
struct Root {
    files: Vec<String>,
    current_base: Option<String>,
    folder_path: String,
    file_content : String,
    file_tree: Option<FileTree>,
    reponse: String
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
            reponse: String::from("empty for now")
        }
    }

    fn title(&self) -> String {
        String::from("Ghucl")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::BaseFileChanged(file_name) => self.current_base = Some(file_name),
            Message::FolderChanged => {
                println!("folder changed {0}", self.folder_path);
                let result = fs::read_dir(&self.folder_path)
                    .and_then(FileTree::from_read_dir)
                    .and_then(|tree| {
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
            Message::FileTreeItemToogled(_) => todo!(),
            Message::FileSelected(file_path) => {
                let content = std::fs::read_to_string(file_path).unwrap_or("could not read file".to_string());
                self.file_content = content;
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
            button("send")
        ]
        .padding(10)
        .align_items(Alignment::Center)
        .spacing(20);

        let tree_view = scrollable(match &self.file_tree {
            Some(fs) => fs.get_elements(),
            None => column![text("no item")].into(),
        });

        let content_row = row![
            tree_view.width(Length::FillPortion(1)),
            scrollable(text(&self.file_content))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .direction(scrollable::Direction::Both { vertical: scrollable::Properties::default(), horizontal: scrollable::Properties::default() }),
            scrollable(text(&self.reponse))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .direction(scrollable::Direction::Both { vertical: scrollable::Properties::default(), horizontal: scrollable::Properties::default() })
        ]
        .spacing(20);
        column![folder_component, header, content_row,]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}
