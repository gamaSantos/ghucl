use std::{
    fs::ReadDir,
    io::Error,
};

#[derive(Default)]
pub struct FileTree {
    items: Vec<FileTreeItem>,
}

enum FileTreeItem {
    Directory {
        name: String,
        chield: Vec<FileTreeItem>,
        depth: i32,
        is_initilized: bool,
    },
    File {
        name: String,
        depth: i32,
    },
}

impl FileTree {
    pub fn from_read_dir(read_dir: ReadDir) -> Result<FileTree, Error> {
        let mut items: Vec<FileTreeItem> = vec![];

        for rde in read_dir {
            match rde {
                Ok(de) => {
                    let file_name = de.file_name().to_str().unwrap().to_string();
                    if let Ok(file_type) = de.file_type() {
                        if file_type.is_file() {
                            items.push(FileTreeItem::File {
                                name: file_name.to_string(),
                                depth: 0,
                            })
                        }
                        if file_type.is_dir() {
                            items.push(FileTreeItem::Directory {
                                name: file_name,
                                chield: vec![],
                                depth: 0,
                                is_initilized: false,
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
            .filter(|i| matches!(i, FileTreeItem::File { name, depth }))
            .map(|i| i.get_name())
            .collect()
    }
}


impl FileTreeItem {
    fn get_name(&self) -> String {
        match self {
            FileTreeItem::Directory { name, chield: _, depth: _, is_initilized: _ } => name.to_owned(),
            FileTreeItem::File { name, depth: _ } => name.to_owned(),
        }
    }

    fn get_element(&self) {
        
    }
}