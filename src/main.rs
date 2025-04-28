
use iced::time;

use iced::alignment::{Horizontal, Vertical};
use iced::{application, executor, Element, Length, Settings, Subscription, Task, Theme};
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor, Container};
use chrono::{DateTime, Local, prelude};

#[derive(Default, Debug)]
struct Home {
    enabled: bool,
    num: i32,
    clock: String,
    time: DateTime<Local>,
}


#[derive(Debug, PartialEq, Clone)]
enum Message {
    Enable,
    Tick,
    Increment,
    Decrement,
}

impl Home {
    

    pub fn theme() -> Theme {
        Theme::default()
    }

    pub fn new(num: i32 ) -> Self {
        Home{
            enabled: false,
            num: num,
            clock: prelude::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
            time: prelude::Local::now(),
        }
    }

    pub fn view(&self) -> Element<Message>{
        let default_color = match Home::theme() {
            iced::Theme::Light => iced::Color::BLACK, 
            iced::Theme::Dark => iced::Color::WHITE,
            _ => iced::Color::BLACK,
        };
        let text_col = if self.num > 10 {
            iced::Color::from_rgba(255.0, 0.0, 0.0, 255.0)
        } else {
            default_color
        };

        let counter_col = column![
            text("Hello Iced!").size(16), 
            button(" + ").on_press(Message::Increment), 
            text(self.num.to_string()).color(text_col), 
            button(" - ").on_press(Message::Decrement), 
            text(&self.clock)].padding(10)
            .spacing(10)
            .align_x(Horizontal::Center)
            .width(Length::Fill);

        let enable_col = column![
            text("Enabled : ".to_owned()).size(16),
            text(if self.enabled {"Y"} else {"N"}).color(if self.enabled {iced::Color::from_rgb(0f32, 255f32, 0f32)} else {iced::Color::from_rgb(255f32, 0f32, 0f32)}),
            button("Enable").on_press(Message::Enable)
        ].spacing(10)
        .align_x(Horizontal::Center)
        .width(Length::Fill);

        row![
            counter_col,
            enable_col,
        ].padding(10)
        .spacing(10)
        .align_y(Vertical::Center)
        .width(Length::Fill).into()
    
    }

    pub fn update(&mut self, message:Message) -> Task<Message>{
        match message {
            Message::Enable => {
                self.enabled = !self.enabled;
            },
            Message::Tick => {
                if self.time != prelude::Local::now() {
                    self.time = prelude::Local::now();
                    self.clock = self.time.format("%d/%m/%Y %H:%M:%S").to_string();
                    if self.enabled {
                        self.num += 1;
                    }
                }
            },
            Message::Increment => {
                self.num += 1;
            },
            Message::Decrement => {
                self.num -= 1;
            }

        };
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_secs(1))
            .map(|_| Message::Tick)
    }
    
    
}


fn main() -> iced::Result{
    iced::application("NetFilterRS", Home::update, Home::view).subscription(Home::subscription).run_with(|| {let initial_state = Home::new(5); (initial_state, iced::Task::none())})
}
