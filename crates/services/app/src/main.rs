use std::{collections::HashMap, sync::Arc};

use futures::{channel::oneshot, SinkExt, StreamExt};
use iced::{
    border::Radius,
    widget::{
        button, column,
        container::{self},
        row,
        scrollable::{Direction, Properties},
        text, text_input, Column, Container, Scrollable,
    },
    Application, Background, Border, Color, Command, Element, Length, Settings, Subscription,
    Theme,
};
use lapin::{
    options::{BasicConsumeOptions, QueueBindOptions},
    types::FieldTable,
    ConnectionProperties,
};
use telemetry::{EnvironmentalTelemetry, Telemetry};
use views::ConnectionForm;

mod views;

fn main() -> iced::Result {
    setup_logging();
    NauticApp::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

struct NauticApp {
    state: AppState,
    on_connect: Option<oneshot::Sender<lapin::Connection>>,

    thermometers: HashMap<String, f32>,
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
                thermometers: Default::default(),
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
                            let Ok(connection) = Arc::<lapin::Connection>::try_unwrap(connection)
                            else {
                                return Command::none();
                            };

                            _ = sender.send(connection);
                            self.thermometers = Default::default();
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
            Message::LapinEvent(event) => {
                let Ok(event) = Arc::<LapinEvent>::try_unwrap(event) else {
                    return Command::none();
                };

                match event {
                    LapinEvent::WaitingForConnection(tx) => {
                        self.on_connect = Some(tx);
                        let url = std::fs::read_to_string("connection.txt").unwrap_or_default();
                        let form = ConnectionForm::new(url);
                        self.state = AppState::Disconnected(form);
                    }
                    LapinEvent::Telemetry(telemetry) => {
                        if let Telemetry::Environmental(env) = telemetry {
                            match env {
                                EnvironmentalTelemetry::Temperature { tag, value } => {
                                    if let Some(temp) = self.thermometers.get_mut(&tag) {
                                        *temp = value;
                                    } else {
                                        self.thermometers.insert(tag, value);
                                    }
                                }
                                EnvironmentalTelemetry::Humidity { tag, value } => (),
                            }
                        }
                    }
                }
            }
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

        let thermometers = Container::new(
            column![
                text("Thermometers").size(18),
                Scrollable::new(Column::with_children(
                    self.thermometers
                        .iter()
                        .map(|x| thermometer(x.0.as_str(), *x.1)),
                ))
                .width(Length::Shrink)
                .height(Length::Shrink)
                .direction(Direction::Vertical(Properties::default()))
            ]
            .spacing(10)
            .padding(8),
        )
        .height(Length::FillPortion(1))
        .width(Length::FillPortion(1))
        .style(|theme: &Theme| container::Appearance {
            border: Border {
                width: 1.5,
                color: theme.palette().primary,
                ..Default::default()
            },
            ..Default::default()
        });

        Container::new(row![column![thermometers]])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        lapin_subscription().map(|x| Message::LapinEvent(Arc::new(x)))
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::GruvboxDark
    }
}

fn thermometer(name: &str, value: f32) -> Element<Message> {
    iced::widget::row![text(name), text(format!("{:.1} ºC", value))]
        .spacing(8)
        .into()
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
    Telemetry(Telemetry),
}

enum LapinState {
    Failed,
    Disconnected(oneshot::Receiver<lapin::Connection>),
    Connected(lapin::Consumer),
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
                            println!("Failed to connect");
                            continue;
                        };

                        let Ok(channel) = connection.create_channel().await else {
                            state = LapinState::Failed;
                            println!("Failed to create channel");
                            continue;
                        };

                        if let Err(err) = queues::telemetry(&channel).await {
                            state = LapinState::Failed;
                            println!("Failed to declare channels {err}");
                            continue;
                        }

                        let consumer = match channel
                            .basic_consume(
                                "telemetry",
                                "telemetry-app",
                                BasicConsumeOptions::default(),
                                FieldTable::default(),
                            )
                            .await
                        {
                            Ok(x) => x,
                            Err(err) => {
                                println!("Could not create consumer: {err}");
                                state = LapinState::Failed;
                                continue;
                            }
                        };

                        state = LapinState::Connected(consumer);
                    }
                    LapinState::Connected(consumer) => {
                        let Some(Ok(delivery)) = consumer.next().await else {
                            state = LapinState::Failed;
                            println!("Delivery failed");
                            continue;
                        };

                        if let Err(_) = delivery
                            .ack(lapin::options::BasicAckOptions::default())
                            .await
                        {
                            state = LapinState::Failed;
                            continue;
                        }

                        let telemetry: Telemetry = match bincode::deserialize(&delivery.data) {
                            Ok(x) => x,
                            Err(err) => {
                                state = LapinState::Failed;
                                println!("{err}");
                                continue;
                            }
                        };

                        _ = output.send(LapinEvent::Telemetry(telemetry)).await;
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

fn setup_logging() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
