#![windows_subsystem = "windows"]



use serde::Deserialize;

mod get_minecraft_dir;
mod get_mods;
mod install_velvet;
pub mod write_json;

use iced::widget::{button, checkbox, column, pick_list, text, vertical_space};
use iced::{executor, theme::Palette, window, Alignment, Application, Color, Command, Element, Length, Settings, Theme};

#[derive(Deserialize)]
struct Versions {
    version: String,
}

pub fn main() -> iced::Result {
    let mut vec: Vec<String> = vec![];
    let version_response: Vec<Versions> =
        reqwest::blocking::get("https://meta.quiltmc.org/v3/versions/game")
            .expect("Failed to fetch latest Quilt version.")
            .json()
            .unwrap();
    for x in version_response {
        vec.push(String::from(&x.version))
    }

    Velvet::run(Settings {
        flags: vec,
        window: window::Settings {
            size: (500, 200),
            resizable: false,
            icon: window::icon::Icon::from_file_data(
                include_bytes!("assets/icon.png"),
                Some(image::ImageFormat::Png),
            )
            .ok(),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Velvet {
    version_list: Vec<String>,
    version: Option<String>,
    vanilla: bool,
    beauty: bool,
    optifine: bool,
    message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
    VButton(bool),
    BButton(bool),
    OButton(bool),
    Press,
    Done,
}

impl Application for Velvet {

    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Vec<String>;

    fn new(flags: Vec<String>) -> (Self, Command<Message>) {
        (
            Velvet {
                version_list: flags,
                version: None,
                vanilla: true,
                beauty: false,
                optifine: false,
                message: String::from("Install"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        match &self.version {
            None => String::from("Velvet Installer"),
            Some(value) => format!("Velvet Installer - {}", &value),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Update(value) => self.version = Some(value),
            Message::VButton(value) => self.vanilla = value,
            Message::BButton(value) => self.beauty = value,
            Message::OButton(value) => self.optifine = value,
            Message::Press => match &self.version {
                Some(value) => {
                    self.message = String::from("Installing...");
                    let values = (self.vanilla, self.beauty, self.optifine);
                    return Command::perform(run(value.clone(), values), |_| Message::Done) 
                }
                None => self.message = String::from("No version selected"),
            },
            Message::Done => self.message = String::from("Finished!"),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            vertical_space(Length::Units(10)),
            text("Enter Minecraft version:").size(20),
            pick_list(&self.version_list, self.version.to_owned(), Message::Update),
            vertical_space(Length::Fill),
            checkbox("Vanilla - Performance enhancing modlist.", self.vanilla, Message::VButton),
            checkbox("Beauty - Immersive and beautiful modlist.", self.beauty, Message::BButton),
            checkbox("Optifine - Optifine resource pack parity.", self.optifine, Message::OButton),
            vertical_space(Length::Fill),
            button(self.message.as_str()).on_press(Message::Press),
            vertical_space(Length::Units(10)),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(Palette {
            background: Color::from_rgb8(25, 23, 36),
            text: Color::from_rgb8(224, 222, 244),
            primary: Color::from_rgb8(235, 111, 146),
            success: Color::from_rgb8(156, 207, 216),
            danger: Color::from_rgb8(235, 111, 146),
        })
    }
}


async fn run(mc_version: String, modlists: (bool, bool, bool)) {
    let version_response: Vec<Versions> =
        reqwest::blocking::get("https://meta.quiltmc.org/v3/versions/loader")
            .expect("Failed to fetch latest Quilt version.")
            .json()
            .unwrap();
    let mut quilt_version = String::new();
    for x in version_response {
        if !x.version.contains('-') {
            quilt_version = x.version;
            break;
        }
    }

    let path_mods = install_velvet::run(&mc_version, &quilt_version);
    get_mods::run(&mc_version, &modlists, path_mods);
}

