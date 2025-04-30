
use std::fs::read_to_string;
use std::path::PathBuf;

use std::io;
use std::sync::Arc;
use iced::widget::text_editor::Edit;
use iced::{time, Alignment};

use iced::alignment::{Horizontal, Vertical};
use iced::{application, executor, Element, Length, Settings, Subscription, Task, Theme};
use iced::widget::{button, center, column, container, horizontal_space, row, text, text_editor, text_input, toggler, Container, Row, Space};
use chrono::{DateTime, Local, prelude};

#[derive(Default, Debug)]
struct Home {
    file: String,
    enabled: bool,
    num: i32,
    filters: Vec<String>,
    clock: String,
    time: DateTime<Local>,
    contents: Vec<text_editor::Content>,
    error: Option<io::ErrorKind>,
}


#[derive(Debug, PartialEq, Clone)]
enum Message {
    Enable(bool),
    Tick,
    Increment,
    Decrement,
    FileOpened(Result<Arc<String>, io::ErrorKind>),
    Save(Result<(), io::ErrorKind>),
    SaveButton,
    Add,
    Update(i32, text_editor::Action),
    None,
}

impl Home {
    

    pub fn theme() -> Theme {
        Theme::default()
    }

    pub fn new(num: i32, file: String ) -> (Self, Task<Message>) {
        
        (   
            Home {
                file: file.to_owned(),
                enabled: false,
                num: num,
                filters: Vec::new(),
                clock: prelude::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
                time: prelude::Local::now(),
                contents: Vec::new(),
                error: None,
            }, 
            Task::perform(
                read_lists(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), &file).into()),
                Message::FileOpened
            )
        )
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
            toggler(self.enabled).on_toggle(Message::Enable).size(20)
        ].spacing(10)
        .align_x(Horizontal::Center)
        .width(Length::Fill);

        let toggles = row![
            counter_col,
            enable_col,

        ].padding(10)
        .spacing(10)
        .align_y(Vertical::Center)
        .width(Length::Fill);

        let mut window = iced::widget::Column::with_children(vec![
            toggles.into(),
        ]).padding(5).align_x(Horizontal::Center);

        for (index, content) in self.contents.iter().clone().into_iter().enumerate() {
            window = window.push(
                Container::new(
                    Row::new()
                        .push(
                            text_editor(&content).size(16).padding(10)
                            .on_action(move |edit| Message::Update(index as i32, edit)))
                        .push(horizontal_space())
                        .push(button("Save").on_press(Message::SaveButton))
                        .align_y(Alignment::Center).spacing(10),
                ).padding(10)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill));
                
                // row![container(text_input(&content, &self.filters[index]).size(16)).padding(10).width(Length::Fill).style(container::rounded_box), container(button("Save").on_press(Message::SaveButton)).width(Length::Shrink),]);
        }

        window = window.push(row![horizontal_space(), button("+").on_press(Message::Add)]);

        
        window.into()
    
    }

    pub fn update(&mut self, message:Message) -> Task<Message>{
        match message {
            Message::Enable(status) => {
                        self.enabled = status;
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
                    },
            Message::FileOpened(Ok(content)) => {
                        self.filters.extend(content.as_str().split('\n').map(|x| x.to_owned()));
                        self.contents.extend(content.as_str().split('\n').map(|x| text_editor::Content::with_text(x)));
                        
                    }
            Message::FileOpened(Err(error)) => {
                        self.error = Some(error);
                    }
            Message::SaveButton => {
                        let _ = Task::perform(write(PathBuf::from(self.file.to_owned()), Vec::clone(&self.filters)), Message::Save);
                    }
            Message::Save(Ok(())) => {
                        self.filters.extend(self.filters.join("\n").as_str().split('\n').map(|x| x.to_owned()));
                    }
            Message::Save(Err(error)) => {
                        self.error = Some(error);
                    }
            Message::Add => {
                        self.filters.push("".to_owned());
                    }
            Message::Update(num, new) => {
                        self.contents[num as usize].perform(new);
                        self.filters[num as usize] = self.contents[num as usize].text();
                    },
            Message::None => {}
                };
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_secs(1))
            .map(|_| Message::Tick)
    }
    
    
}

async fn read_lists(file: PathBuf) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(file).await.map(Arc::new).map_err(|err| err.kind())
}

async fn write(file: PathBuf, lists: Vec<String>) -> Result<(), io::ErrorKind> {
    tokio::fs::write(&file, &lists.join("\n")).await.map_err(|err| err.kind())
}


fn main() -> iced::Result{
    iced::application("NetFilterRS", Home::update, Home::view).subscription(Home::subscription).run_with(|| {let initial_state = Home::new(5, "filter.txt".to_owned()); initial_state})
}
