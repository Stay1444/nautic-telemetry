use std::{ops::Deref, sync::Arc};

use strum::IntoEnumIterator;

use iced::{
    theme,
    widget::{column, row, Button, Column, Container, Text},
    Alignment, Color, Command, Element, Length, Shadow, Vector,
};

use super::PageAction;

#[derive(Clone, Debug)]
pub struct DashboardPage {
    connection: Arc<lapin::Connection>,
    layer: Layer,
}

impl DashboardPage {
    pub fn new(connection: lapin::Connection) -> Self {
        Self {
            connection: Arc::new(connection),
            layer: Layer::Inicio,
        }
    }

    pub fn update(&mut self, message: Message) -> PageAction {
        match message {
            Message::ChangeLayer(layer) => self.layer = layer,
        }

        PageAction::Command(Command::none())
    }

    pub fn view(&self) -> Element<crate::Message> {
        row![
            Container::new(
                Column::with_children(Layer::iter().map(|x| { tab(x, self.layer.clone()) }))
                    .padding(4.0)
                    .spacing(4.0)
                    .align_items(Alignment::Center)
                    .max_width(100.0)
            )
            .style(iced::widget::container::Appearance {
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 5.0
                },
                ..Default::default()
            })
            .width(Length::Shrink)
            .height(Length::Fill),
            column![]
        ]
        .into()
    }
}

fn tab(layer: Layer, active: Layer) -> Element<'static, crate::Message> {
    let active = layer == active;
    Button::new(Text::new(format!("{}", layer)))
        .width(Length::Fill)
        .style(if active {
            theme::Button::Primary
        } else {
            theme::Button::Secondary
        })
        .on_press_maybe(if active {
            None
        } else {
            Some(Message::ChangeLayer(layer).wrap())
        })
        .into()
}

#[derive(Clone, Debug, strum::EnumIter, strum::Display, PartialEq, Eq)]
pub enum Layer {
    Inicio,
    Radio,
    Reles,
    Tareas,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeLayer(Layer),
}

impl Message {
    fn wrap(self) -> crate::Message {
        crate::Message::Page(super::Message::Dashboard(self))
    }
}
