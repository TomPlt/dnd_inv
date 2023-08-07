use iced::{Settings, Sandbox};
use iced::widget::Text;

struct HelloWorld;

impl Sandbox for HelloWorld {
    type Message = ();

    fn new() -> Self {
        HelloWorld
    }

    fn title(&self) -> String {
        String::from("A simple Iced app")
    }

    fn update(&mut self, _message: Self::Message) {
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Text::new("Hello, world!").into()
    }
}

fn main() -> iced::Result {
    HelloWorld::run(Settings::default())
}
