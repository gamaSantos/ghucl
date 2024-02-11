use std::{
    fs::{self, ReadDir},
    io::Error,
    path::Path,
    vec,
};

use iced::{
    widget::{button, column, text},
    Element,
};

use crate::message::Message;

#[derive(Default)]
pub struct FileTree {
    root: String,
    items: Vec<FileTreeItem>,
}

enum FileTreeItem {
    Control { name: String, path: String },
    Directory { name: String, path: String },
    File { name: String, path: String },
}

impl FileTree {
    pub fn from_path(path: &String) -> Result<FileTree, Error> {
        match fs::read_dir(path).and_then(|rd| FileTree::get_items(rd, path)) {
            Ok(items) => Ok(FileTree {
                root: path.clone(),
                items,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_file_names(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|i| matches!(i, FileTreeItem::File { name: _, path: _ }))
            .map(|i| i.get_name())
            .collect()
    }

    pub fn get_elements(&self) -> Element<Message> {
        let elem: Vec<Element<Message>> = self.items.iter().map(|fi| fi.get_element()).collect();
        column(elem).into()
    }

    pub fn navigate(&mut self, path: &String) {
        println!("trying to navigate to {path}");
        if path.starts_with(self.root.as_str()) == false {
            println!("could not navigate to {path}");
            return;
        }
        
        let parent = Path::new(path).parent().unwrap_or(Path::new(&self.root));
        let back_path = if parent.starts_with(&self.root) {
            String::from(parent.to_str().unwrap_or(&self.root))
        } else {
            self.root.clone()
        };
        let map_result = fs::read_dir(path).and_then(|rd| FileTree::get_items(rd, &back_path));

        if let Ok(items) = map_result {
            self.items = items;
        }
    }

    fn get_items(read_dir: ReadDir, back_path: &String) -> Result<Vec<FileTreeItem>, Error> {
        let mut dir_items: Vec<FileTreeItem> = vec![];
        let mut file_items: Vec<FileTreeItem> = vec![];

        for rde in read_dir {
            match rde {
                Ok(de) => {
                    let file_name = de.file_name().to_str().unwrap_or("whoops").to_string();
                    let path = de.path().to_str().unwrap_or("").to_string();
                    if let Ok(file_type) = de.file_type() {
                        if file_type.is_file() {
                            dir_items.push(FileTreeItem::File {
                                name: file_name.to_string(),
                                path: path.clone(),
                            })
                        }
                        if file_type.is_dir() {
                            file_items.push(FileTreeItem::Directory {
                                name: file_name,
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
        dir_items.sort_by(|a, b| a.get_name().cmp(&b.get_name()));
        
        file_items.sort_by(|a, b| a.get_name().cmp(&b.get_name()));
        file_items.append(&mut dir_items);
        file_items.insert(0,
            FileTreeItem::Control {
                name: "..".to_string(),
                path: back_path.clone(),
            },
        );
        Ok(file_items)
    }
}

impl FileTreeItem {
    fn get_name(&self) -> String {
        match self {
            FileTreeItem::Directory { name, path: _ } => name.to_owned(),
            FileTreeItem::File { name, path: _ } => name.to_owned(),
            FileTreeItem::Control { name, path: _ } => name.to_owned(),
        }
    }

    fn get_element(&self) -> Element<Message> {
        match self {
            FileTreeItem::Directory { name, path } => button(text(format!("> {0}", name)))
                .on_press(Message::FileTreeItemToogled(path.clone()))
                .style(iced::theme::Button::Text),
            FileTreeItem::File { name, path } => button(name.as_str())
                .on_press(Message::FileSelected(path.clone()))
                .style(iced::theme::Button::Text),
            FileTreeItem::Control { name, path } => button(text(name))
                .on_press(Message::FileTreeItemToogled(path.clone()))
                .style(iced::theme::Button::Text),
        }
        .into()
    }
}
