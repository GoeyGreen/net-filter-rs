
use iced::alignment::Horizontal;
use iced::{executor, application, Element, Length, Settings, Theme, Subscription};
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor};


#[derive(Default, Debug)]
struct Home {
    enabled: bool,
    num: i32,
}


#[derive(Debug, PartialEq)]
enum Message {
    Enable
}

impl Home {
    pub fn view(&self) -> Element<Message>{
        column![text("Hello Iced!").size(16)].padding(10).align_x(Horizontal::Center).width(Length::Fill).into()
    }
    pub fn update(&mut self, message:Message) {
        match message {
            Message::Enable => {
                self.enabled = true
            }
        };
    }

}

fn main() -> iced::Result{
    iced::application("NetFilterRS", Home::update, Home::view).run()
}
