use std::sync::Arc;

use iced::{
    border::Radius,
    widget::{
        button, column,
        container::{self},
        text, text_input, Column, Container,
    },
    window::Action,
    Application, Background, Border, Color, Command, Element, Length, Settings, Subscription,
    Theme,
};
use lapin::ConnectionProperties;

fn main() -> iced::Result {
    NauticApp::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Default)]
struct NauticApp {
    state: AppState,
    amqp_url: String,
    connect_error: Option<String>,
}

#[derive(Clone, Default)]
enum AppState {
    Connected(Arc<lapin::Connection>),
    #[default]
    Disconnected,
    Connecting,
}

#[derive(Clone, Debug)]
enum Message {
    AmqpUrlEdited(String),
    Connect,
    Connected(Result<(Arc<lapin::Connection>, String), String>),
}

impl Application for NauticApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        if let Ok(url) = std::fs::read_to_string("connection.txt") {
            return (
                Self {
                    amqp_url: url,
                    ..Default::default()
                },
                Command::none(),
            );
        }
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Easy Robotics")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AmqpUrlEdited(value) => self.amqp_url = value,
            Message::Connect => {
                self.state = AppState::Connecting;
                self.connect_error = None;
                return Command::perform(
                    connect_rabbitmq(self.amqp_url.clone()),
                    Message::Connected,
                );
            }
            Message::Connected(result) => {
                match result {
                    Ok((connection, url)) => {
                        self.state = AppState::Connected(connection);
                        self.connect_error = None;

                        _ = std::fs::write("connection.txt", url);
                    }
                    Err(message) => {
                        self.connect_error = Some(message);
                        self.state = AppState::Disconnected;
                    }
                };
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let connecting = matches!(self.state, AppState::Connecting);
        if matches!(self.state, AppState::Disconnected)
            || matches!(self.state, AppState::Connecting)
        {
            let error = if let Some(error) = &self.connect_error {
                let container = Container::new(text(error).style(Color::WHITE))
                    .style(|_theme: &Theme| container::Appearance {
                        background: Some(Background::Color(Color::from_rgba8(255, 104, 96, 1.0))),
                        border: Border {
                            radius: Radius::from(8.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .padding(10)
                    .width(Length::Fill);
                Some(container)
            } else {
                None
            };

            let form = Column::new()
                .push(
                    text_input("amqp://127.0.0.1:5672//", &self.amqp_url)
                        .on_input(Message::AmqpUrlEdited)
                        .on_submit(Message::Connect),
                )
                .push(
                    button(if connecting {
                        "Connecting..."
                    } else {
                        "Connect"
                    })
                    .on_press_maybe(
                        if !self.amqp_url.is_empty() && !connecting {
                            Some(Message::Connect)
                        } else {
                            None
                        },
                    ),
                )
                .push_maybe(error)
                .max_width(400)
                .spacing(20);

            return Container::new(form)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .padding(20)
                .into();
        }

        column![].into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match self.state {
            AppState::Connected(_) => Subscription::none(),
            AppState::Disconnected => Subscription::none(),
            AppState::Connecting => Subscription::none(),
        }
    }
}

async fn connect_rabbitmq(url: String) -> Result<(Arc<lapin::Connection>, String), String> {
    tokio::select! {
        connection = lapin::Connection::connect(&url, ConnectionProperties::default()) => {
            return Ok((Arc::new(connection.map_err(|err| err.to_string())?), url));
        },
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            return Err(String::from("Timed out"));
        }
    };
}
