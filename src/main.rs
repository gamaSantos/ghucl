use iced::widget::{button, column, horizontal_space, pick_list, row, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Root::run(Settings::default())
}

struct Root {
    value: i32,
    files: Vec<String>,
    current_base: String
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    FileSelected(String)
}

impl Sandbox for Root {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            files: vec!["file_one.toml".to_owned(), "environment.toml".to_owned()],
            current_base: String::from("none")
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
            Message::FileSelected(file_name) => self.current_base = file_name,
        }
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text("choose your destiny"),
            horizontal_space(Length::Fill),
            text(&self.current_base),
            horizontal_space(Length::Fill),
            pick_list(&self.files, Some(self.files[0].clone()), Message::FileSelected)
        ]
        .padding(100);
        column![
            header,
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
