use wasm_bindgen::prelude::*;
use iced::{
    button, executor,
    Application, Settings, Command,
    Align, Button, Column, Element, Text
};

mod mstyle;
use mstyle::style;

#[derive(Default)]
struct App {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Null;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("iced_poc")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .style(style::Button::Blue)
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                .style(style::Button::Blue)
                .on_press(Message::DecrementPressed),
            )
            .into()
    }
}


#[wasm_bindgen]
pub fn run() {
    App::run(Settings::default())
}