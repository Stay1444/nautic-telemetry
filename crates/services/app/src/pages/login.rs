use std::{path::PathBuf, sync::Arc, time::Duration};

use iced::{
    border::Radius,
    widget::{column, row, Button, Container, Text, TextInput, Toggler},
    Alignment, Background, Color, Command, Element, Length, Theme, Vector,
};
use iced_aw::NumberInput;
use lapin::ConnectionProperties;
use serde::{Deserialize, Serialize};

use super::{dashboard::DashboardPage, Page, PageAction};

#[derive(Debug, Clone)]
pub struct LoginPage {
    ip: String,
    port: u16,
    username: String,
    password: String,
    vhost: String,
    remember: bool,

    correct: bool,
    connecting: bool,

    error: Option<String>,
}

impl Default for LoginPage {
    fn default() -> Self {
        let path: PathBuf = "login.json".into();
        let mut settings = Settings::default();
        let mut error = None;

        if path.exists() {
            match std::fs::read_to_string(path) {
                Ok(json) => {
                    match serde_json::from_str(&json) {
                        Ok(x) => settings = x,
                        Err(err) => error = Some(err.to_string()),
                    };
                }
                Err(x) => {
                    error = Some(x.to_string());
                }
            };
        }

        Self {
            ip: settings.ip,
            port: settings.port,
            username: settings.username,
            password: settings.password,
            vhost: settings.vhost,
            remember: true,
            correct: true,
            connecting: false,
            error,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Settings {
    ip: String,
    port: u16,
    username: String,
    password: String,
    vhost: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ip: String::from("0.0.0.0"),
            port: 5672,
            username: String::from("root"),
            password: String::from("root"),
            vhost: String::from("/"),
        }
    }
}

impl LoginPage {
    pub fn update(&mut self, message: Message) -> PageAction {
        match message {
            Message::IpChanged(value) => self.ip = value,
            Message::PortChanged(value) => self.port = value,
            Message::UsernameChanged(value) => self.username = value,
            Message::PasswordChanged(value) => self.password = value,
            Message::VHostChanged(value) => self.vhost = value,
            Message::RememberChanged(value) => self.remember = value,
            Message::Connect => {
                let url = format!(
                    "amqp://{}:{}@{}:{}/{}",
                    self.username, self.password, self.ip, self.port, self.vhost
                );

                self.connecting = true;

                return PageAction::Command(Command::perform(connect(url), |v| {
                    Message::Connected(Arc::new(v)).wrap()
                }));
            }
            Message::Connected(result) => {
                self.error = None;

                let result =
                    Arc::try_unwrap(result).expect("Value should be unwrapped successfully.");

                if let Err(err) = result {
                    self.error = Some(err.to_string());
                    self.connecting = false;
                    return PageAction::Command(Command::none());
                }

                if let Ok(connection) = result {
                    if self.remember {
                        let settings = Settings {
                            ip: self.ip.clone(),
                            port: self.port,
                            username: self.username.clone(),
                            password: self.password.clone(),
                            vhost: self.vhost.clone(),
                        };

                        let json = serde_json::to_string(&settings).expect("Serialization");

                        if let Err(err) = std::fs::write("login.json", json) {
                            self.error = Some(err.to_string());
                        }
                    }

                    return PageAction::ChangePage(Page::Dashboard(DashboardPage::new(connection)));
                }

                self.connecting = false;
            }
        }

        self.correct = true;

        if self.ip.trim().is_empty() {
            self.correct = false;
        }

        if self.username.trim().is_empty() || self.password.trim().is_empty() {
            self.correct = false;
        }

        if self.vhost.trim().is_empty() {
            self.correct = false;
        }

        PageAction::Command(Command::none())
    }

    pub fn view(&self) -> Element<crate::Message> {
        let error = if let Some(err) = &self.error {
            Container::new(Text::new(err))
                .width(Length::Fill)
                .style(iced::widget::container::Appearance {
                    background: Some(Background::Color(Color::from_rgba8(255, 51, 51, 1.0))),
                    border: iced::Border {
                        radius: Radius::from(8.0),
                        ..Default::default()
                    },
                    text_color: Some(Color::WHITE),
                    ..Default::default()
                })
                .padding(10)
                .into()
        } else {
            Container::new(row![])
        };
        let modal = Container::new(
            column![
                Text::new("Login").size(20),
                row![
                    TextInput::new("AMQP IP", &self.ip).on_input(|v| Message::IpChanged(v).wrap()),
                    NumberInput::new(self.port, u16::MAX, |v| Message::PortChanged(v).wrap())
                ]
                .spacing(8),
                row![
                    TextInput::new("Username", &self.username)
                        .on_input(|v| Message::UsernameChanged(v).wrap()),
                    TextInput::new("Password", &self.password)
                        .on_input(|v| Message::PasswordChanged(v).wrap())
                ]
                .spacing(8),
                TextInput::new("Virtual Host", &self.vhost)
                    .on_input(|v| Message::VHostChanged(v).wrap()),
                row![
                    Toggler::new(
                        Some(String::from("Remember Configuration")),
                        self.remember,
                        |v| { Message::RememberChanged(v).wrap() }
                    ),
                    Button::new(Text::new(if !self.connecting {
                        "Connect"
                    } else {
                        "Connecting"
                    }))
                    .on_press_maybe(if self.correct && !self.connecting {
                        Some(Message::Connect.wrap())
                    } else {
                        None
                    })
                ],
                error
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .padding(10.0)
            .width(Length::Fill),
        )
        .width(400)
        .style(|_theme: &Theme| iced::widget::container::Appearance {
            shadow: iced::Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 25.0,
            },
            border: iced::Border {
                radius: Radius::from(10.0),
                ..Default::default()
            },
            background: Some(Background::Color(Color::from_rgba8(220, 220, 220, 1.0))),
            ..Default::default()
        });

        Container::new(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_: &_| iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgba8(45, 49, 53, 1.0))),
                ..Default::default()
            })
            .center_x()
            .center_y()
            .into()
    }
}

async fn connect(url: String) -> anyhow::Result<lapin::Connection> {
    let conn = lapin::Connection::connect(&url, ConnectionProperties::default());
    tokio::select! {
        result = conn => {
            Ok(result?)
        },
        _ = tokio::time::sleep(Duration::from_secs(10)) =>  {
            Err(anyhow::anyhow!("Timed out after 10 seconds."))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    IpChanged(String),
    PortChanged(u16),
    UsernameChanged(String),
    PasswordChanged(String),
    VHostChanged(String),
    RememberChanged(bool),
    Connect,
    Connected(Arc<Result<lapin::Connection, anyhow::Error>>),
}

impl Message {
    fn wrap(self) -> crate::Message {
        crate::Message::Page(super::Message::Login(self))
    }
}
