use iced::{Application, Command, Element, Settings};
use pages::{login::LoginPage, Page, PageAction};

pub mod pages;
pub mod style;

fn main() -> iced::Result {
    setup_logging();

    NauticApp::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

struct NauticApp {
    page: Page,
}

#[derive(Clone, Debug)]
pub enum Message {
    Page(pages::Message),
}

impl Application for NauticApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                page: Page::Login(LoginPage::default()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Easy Robotics")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Page(message) => match self.page.update(message) {
                PageAction::Command(command) => return command,
                PageAction::ChangePage(page) => self.page = page,
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        self.page.view()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Light
    }
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
