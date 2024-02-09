use std::{fmt::format, fs::ReadDir, io::Error};

use iced::{
    application::StyleSheet,
    widget::{button, column, text},
    Element, Theme,
};

use crate::message::Message;

#[derive(Default)]
pub struct FileTree {
    items: Vec<FileTreeItem>,
}

enum FileTreeItem {
    Directory {
        name: String,
        path: String,
        chield: Vec<FileTreeItem>,
        depth: i32,
        is_initilized: bool,
        is_open: bool,
    },
    File {
        name: String,
        path: String,
        depth: i32,
    },
}

impl FileTree {
    pub fn from_read_dir(read_dir: ReadDir) -> Result<FileTree, Error> {
        let mut items: Vec<FileTreeItem> = vec![];

        for rde in read_dir {
            match rde {
                Ok(de) => {
                    let file_name = de.file_name().to_str().unwrap_or("whoops").to_string();
                    let path = de.path().to_str().unwrap_or("").to_string();
                    if let Ok(file_type) = de.file_type() {
                        if file_type.is_file() {
                            items.push(FileTreeItem::File {
                                name: file_name.to_string(),
                                depth: 0,
                                path: path.clone(),
                            })
                        }
                        if file_type.is_dir() {
                            items.push(FileTreeItem::Directory {
                                name: file_name,
                                chield: vec![],
                                depth: 0,
                                is_initilized: false,
                                is_open: false,
                                path: path.clone(),
                            })
                        }
                    }
                }
                Err(de_error) => {
                    return Err(de_error);
                }
            }
        }

        return Ok(FileTree { items });
    }

    pub fn get_file_names(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|i| matches!(i, FileTreeItem::File { name, depth, path }))
            .map(|i| i.get_name())
            .collect()
    }

    pub fn get_elements(&self) -> Element<Message> {
        let elem: Vec<Element<Message>> = self.items.iter().map(|fi| fi.get_element()).collect();
        column(elem).into()
    }
}

impl FileTreeItem {
    fn get_name(&self) -> String {
        match self {
            FileTreeItem::Directory {
                name,
                chield: _,
                depth: _,
                is_initilized: _,
                is_open: _,
                path,
            } => name.to_owned(),
            FileTreeItem::File {
                name,
                depth: _,
                path: _,
            } => name.to_owned(),
        }
    }

    fn get_element(&self) -> Element<Message> {
        match self {
            FileTreeItem::Directory {
                name,
                chield: _,
                depth: _,
                is_initilized: _,
                is_open,
                path,
            } => {
                let prefix = if is_open.to_owned() { "◿" } else { ">" };

                button(text(format!("{0} {1}", prefix, name)))
                    .on_press(Message::FileTreeItemToogled(path.clone()))
                    .style(iced::theme::Button::Text)
            }
            FileTreeItem::File {
                name,
                depth: _,
                path,
            } => button(name.as_str())
                .on_press(Message::FileSelected(path.clone()))
                .style(iced::theme::Button::Text),
        }
        .into()
    }
}

fn get_button_theme() -> iced::theme::Button {
    let default = iced::theme::Button::default();
    default
}
