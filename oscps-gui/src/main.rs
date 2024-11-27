#[derive(Default)]
struct Counter {
    value: i8,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}
use iced::widget::{button, row, text, Row};
impl Counter {
    pub fn view(&self) -> Row<Message> {
        row![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
