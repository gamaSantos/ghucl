#[derive(Debug, Clone)]
pub enum Message {
    BaseFileChanged(String),
    FolderInputValueChange(String),
    FolderChanged,
    FileTreeItemToogled(String),
    FileSelected(String)
}
