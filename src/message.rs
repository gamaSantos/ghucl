#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    FileSelected(String),
    FolderInputValueChange(String),
    FolderChanged,
}