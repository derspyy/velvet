#![windows_subsystem = "windows"]

use anyhow::Result;
use iced::futures::TryFutureExt;
use serde::Deserialize;

mod get_minecraft_dir;
mod get_mods;
mod install_velvet;
pub mod write_json;

use iced::widget::{button, checkbox, column, pick_list, text, vertical_space};
use iced::{
    executor, theme::Palette, window, Alignment, Application, Color, Command, Element, Length,
    Settings, Theme,
};

#[derive(Deserialize)]
struct Versions {
    version: String,
    stable: bool,
}

pub fn main() -> iced::Result {
    Velvet::run(Settings {
        window: window::Settings {
            size: (500, 200),
            min_size: Some((500, 250)),
            resizable: true,
            icon: window::icon::from_file_data(
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
    snapshot: bool,
    version: Option<String>,
    vanilla: bool,
    beauty: bool,
    optifine: bool,
    message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Populate(Vec<String>),
    Update(String),
    Snapshot(bool),
    VButton(bool),
    BButton(bool),
    OButton(bool),
    Press,
    Done(Result<String, String>),
}

impl Application for Velvet {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Velvet {
                version_list: Vec::new(),
                snapshot: false,
                version: None,
                vanilla: true,
                beauty: false,
                optifine: false,
                message: String::from("Install"),
            },
            Command::perform(populate(false), Message::Populate),
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
            Message::Populate(value) => self.version_list = value,
            Message::Update(value) => self.version = Some(value),
            Message::Snapshot(value) => {
                self.snapshot = value;
                return Command::perform(populate(value), Message::Populate);
            }
            Message::VButton(value) => self.vanilla = value,
            Message::BButton(value) => self.beauty = value,
            Message::OButton(value) => self.optifine = value,
            Message::Press => match &self.version {
                Some(value) => {
                    self.message = String::from("Installing...");
                    let values = (self.vanilla, self.beauty, self.optifine);
                    return Command::perform(
                        run(value.clone(), values).map_err(|e| format!("{e}")),
                        Message::Done,
                    );
                }
                None => self.message = String::from("No version selected"),
            },
            Message::Done(value) => match value {
                Ok(x) => match x.is_empty() {
                    true => self.message = String::from("Finished!"),
                    false => {
                        self.message =
                            format!("Finished, although the following mods {x} were unavailable.")
                    }
                },
                Err(x) => self.message = x,
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            vertical_space(Length::Fixed(10.0)),
            text("Enter Minecraft version:").size(20),
            vertical_space(Length::Fixed(5.0)),
            pick_list(&self.version_list, self.version.clone(), Message::Update)
                .width(Length::Fixed(200.0)),
            vertical_space(Length::Fixed(5.0)),
            checkbox("Show snapshots", self.snapshot, Message::Snapshot),
            vertical_space(Length::Fill),
            checkbox(
                "Vanilla - Performance enhancing modlist.",
                self.vanilla,
                Message::VButton
            ),
            vertical_space(Length::Fixed(5.0)),
            checkbox(
                "Beauty - Immersive and beautiful modlist.",
                self.beauty,
                Message::BButton
            ),
            vertical_space(Length::Fixed(5.0)),
            checkbox(
                "Optifine - Optifine resource pack parity.",
                self.optifine,
                Message::OButton
            ),
            vertical_space(Length::Fill),
            button(self.message.as_str()).on_press(Message::Press),
            vertical_space(Length::Fixed(10.0)),
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

#[derive(Deserialize)]
struct Response {
    version: String,
}

async fn run(mc_version: String, modlists: (bool, bool, bool)) -> Result<String> {
    let response: Vec<Response> = reqwest::get("https://meta.quiltmc.org/v3/versions/loader")
        .await?
        .json()
        .await?;

    let mut quilt_version = String::new();
    for x in response {
        if !x.version.contains('-') {
            quilt_version = x.version;
            break;
        }
    }

    let path_mods = install_velvet::run(&mc_version, &quilt_version).await?;
    let errors = get_mods::run(mc_version, &modlists, path_mods).await?;
    Ok(errors)
}

async fn populate(x: bool) -> Vec<String> {
    let mut versions_list = Vec::new();
    let response: Vec<Versions> = reqwest::get("https://meta.quiltmc.org/v3/versions/game")
        .await
        .expect("Couldn't get versions.")
        .json()
        .await
        .unwrap();

    for value in response {
        if x || value.stable {
            versions_list.push(value.version)
        }
    }
    versions_list
}
