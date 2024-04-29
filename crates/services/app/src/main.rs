use std::sync::Arc;

use futures::{channel::oneshot, SinkExt};
use iced::{
    border::Radius,
    widget::{
        button, column,
        container::{self},
        text, text_input, Column, Container,
    },
    Application, Background, Border, Color, Command, Element, Length, Settings, Subscription,
    Theme,
};
use lapin::{options::ExchangeDeclareOptions, types::FieldTable, ConnectionProperties};
use views::ConnectionForm;

mod views;

fn main() -> iced::Result {
    NauticApp::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

struct NauticApp {
    state: AppState,
    on_connect: Option<oneshot::Sender<Arc<lapin::Connection>>>,
}

enum AppState {
    Connected,
    Disconnected(ConnectionForm),
    Connecting,
}

#[derive(Clone, Debug)]
enum Message {
    AmqpUrlEdited(String),
    Connect,
    Connected(Result<(Arc<lapin::Connection>, String), String>),
    LapinEvent(Arc<LapinEvent>),
}

impl Application for NauticApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let url = std::fs::read_to_string("connection.txt").unwrap_or_default();

        (
            Self {
                state: AppState::Disconnected(ConnectionForm::new(url)),
                on_connect: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Easy Robotics")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AmqpUrlEdited(value) => {
                if let AppState::Disconnected(form) = &mut self.state {
                    form.set_url(value);
                    form.set_error(None);
                }
            }
            Message::Connect => {
                let url = if let AppState::Disconnected(form) = &self.state {
                    form.url().to_owned()
                } else {
                    return Command::none();
                };

                self.state = AppState::Connecting;

                return Command::perform(connect_rabbitmq(url), Message::Connected);
            }
            Message::Connected(result) => {
                match result {
                    Ok((connection, url)) => {
                        if let Some(sender) = self.on_connect.take() {
                            _ = sender.send(connection);
                            self.state = AppState::Connected;
                            _ = std::fs::write("connection.txt", url);
                        }
                    }
                    Err(message) => {
                        let url = std::fs::read_to_string("connection.txt").unwrap_or_default();
                        let mut form = ConnectionForm::new(url);
                        form.set_error(Some(message));
                        self.state = AppState::Disconnected(form);
                    }
                };
            }
            Message::LapinEvent(_) => todo!(),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let error = if let AppState::Disconnected(form) = &self.state {
            if let Some(error) = form.error() {
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
            }
        } else {
            None
        };

        if let AppState::Disconnected(form) = &self.state {
            let col = Column::new()
                .push(
                    text_input("amqp://127.0.0.1:5672//", form.url())
                        .on_input(Message::AmqpUrlEdited)
                        .on_submit(Message::Connect),
                )
                .push(button("Connect").on_press(Message::Connect))
                .push_maybe(error)
                .max_width(400)
                .spacing(20);

            return Container::new(col)
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
        lapin_subscription().map(|x| Message::LapinEvent(Arc::new(x)))
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

#[derive(Debug)]
enum LapinEvent {
    WaitingForConnection(oneshot::Sender<lapin::Connection>),
}

enum LapinState {
    Failed,
    Disconnected(oneshot::Receiver<lapin::Connection>),
    Connected(),
}

fn lapin_subscription() -> Subscription<LapinEvent> {
    iced::subscription::channel(
        std::any::TypeId::of::<LapinState>(),
        10,
        |mut output| async move {
            let mut state = LapinState::Failed;

            loop {
                match &mut state {
                    LapinState::Disconnected(rx) => {
                        let Ok(connection) = rx.await else {
                            state = LapinState::Failed;
                            continue;
                        };

                        let Ok(channel) = connection.create_channel().await else {
                            state = LapinState::Failed;
                            continue;
                        };

                        if let Err(err) = lapin_declare_channels(&channel).await {
                            state = LapinState::Failed;
                        }
                    }
                    LapinState::Connected() => {
                        println!("lapin subscripber changed to connected state");
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                    LapinState::Failed => {
                        let (sender, receiver) = oneshot::channel();
                        state = LapinState::Disconnected(receiver);
                        _ = output.send(LapinEvent::WaitingForConnection(sender)).await;
                    }
                }
            }
        },
    )
}

async fn lapin_declare_channels(channel: &lapin::Channel) -> anyhow::Result<()> {
    channel
        .exchange_declare(
            queues::telemetry::exange::NAME,
            queues::telemetry::exange::KIND,
            queues::telemetry::exange::options(),
            queues::telemetry::exange::arguments(),
        )
        .await?;

    channel
        .queue_declare(
            queues::telemetry::NAME,
            queues::telemetry::options(),
            queues::telemetry::arguments(),
        )
        .await?;

    Ok(())
}
