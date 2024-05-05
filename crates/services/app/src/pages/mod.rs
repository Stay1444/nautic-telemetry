use iced::{Command, Element};

use self::{dashboard::DashboardPage, login::LoginPage};

pub mod dashboard;
pub mod login;

#[derive(Debug, Clone)]
pub enum Page {
    Login(LoginPage),
    Dashboard(DashboardPage),
}

#[derive(Debug, Clone)]
pub enum Message {
    Login(login::Message),
    Dashboard(dashboard::Message),
}

pub enum PageAction {
    Command(Command<crate::Message>),
    ChangePage(Page),
}

impl Page {
    pub fn update(&mut self, message: Message) -> PageAction {
        match (message, self) {
            (Message::Login(message), Page::Login(p)) => p.update(message),
            (Message::Dashboard(message), Page::Dashboard(p)) => p.update(message),
            _ => unreachable!(),
        }
    }

    pub fn view(&self) -> Element<super::Message> {
        match self {
            Page::Login(p) => p.view(),
            Page::Dashboard(p) => p.view(),
        }
    }
}
