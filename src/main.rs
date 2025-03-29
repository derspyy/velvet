#![windows_subsystem = "windows"]

use anyhow::Result;
use iced::alignment::Horizontal;
use iced::futures::TryFutureExt;
use serde::Deserialize;

mod get_minecraft_dir;
mod get_mods;
mod install_velvet;
pub mod write_json;

use iced::widget::{button, checkbox, column, pick_list, text, Column, Space};
use iced::{
    application, theme::Palette, window, Alignment, Color, Element, Length, Size, Task, Theme,
};

#[derive(Deserialize)]
struct Versions {
    version: String,
    stable: bool,
}

pub fn main() -> iced::Result {
    application(Velvet::title, Velvet::update, Velvet::view)
        .theme(Velvet::theme)
        .window(window::Settings {
            size: Size::new(500.0, 250.0),
            resizable: false,
            icon: window::icon::from_file_data(
                include_bytes!("assets/icon.png"),
                Some(image::ImageFormat::Png),
            )
            .ok(),
            ..window::Settings::default()
        })
        .antialiasing(true)
        .run_with(Velvet::new)
}

struct Velvet {
    version_list: Vec<String>,
    snapshot: bool,
    version: Option<String>,
    vanilla: bool,
    beauty: bool,
    optifine: bool,
    status: Status,
}

enum Status {
    Idle,
    Installing,
    NoVersion,
    Success(Option<Vec<String>>),
    Failure(String),
}

#[derive(Debug, Clone)]
enum Message {
    Populate(Vec<String>),
    Update(String),
    Snapshot(bool),
    VButton(bool),
    BButton(bool),
    OButton(bool),
    Pressed,
    Done(Result<Vec<String>, String>),
}

impl Velvet {
    fn new() -> (Self, Task<Message>) {
        (
            Velvet {
                version_list: Vec::new(),
                snapshot: false,
                version: None,
                vanilla: true,
                beauty: false,
                optifine: false,
                status: Status::Idle,
            },
            Task::perform(populate(false), Message::Populate),
        )
    }

    fn title(&self) -> String {
        match &self.version {
            None => String::from("Velvet Installer"),
            Some(value) => format!("Velvet Installer - {}", &value),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Populate(value) => {
                self.version = Some(value[0].clone());
                self.version_list = value;
            }
            Message::Update(value) => self.version = Some(value),
            Message::Snapshot(value) => {
                self.snapshot = value;
                return Task::perform(populate(value), Message::Populate);
            }
            Message::VButton(value) => self.vanilla = value,
            Message::BButton(value) => self.beauty = value,
            Message::OButton(value) => self.optifine = value,
            Message::Pressed => {
                match &self.version {
                    Some(value) => {
                        self.status = Status::Installing;
                        let values = (self.vanilla, self.beauty, self.optifine);
                        let mut commands = Vec::new();
                        commands.push(Task::perform(
                            run(value.clone(), values).map_err(|e| format!("{e}")),
                            Message::Done,
                        ));
                        commands.push(
                            window::get_oldest()
                                .and_then(move |id| window::resize(id, (500.0, 250.0).into())),
                        );
                        return Task::batch(commands);
                    }
                    None => self.status = Status::NoVersion,
                };
            }
            Message::Done(value) => match value {
                Ok(x) => match x.is_empty() {
                    true => self.status = Status::Success(None),
                    false => {
                        self.status = Status::Success(Some(x));
                        return window::get_latest()
                            .and_then(move |id| window::resize(id, (500.0, 350.0).into()));
                    }
                },
                Err(x) => {
                    self.status = Status::Failure(x);
                    return window::get_latest()
                        .and_then(move |id| window::resize(id, (500.0, 275.0).into()));
                }
            },
        }
        Task::none()
    }

    fn view(&self) -> Column<Message> {
        let red = Color::from_rgb8(235, 111, 146);
        let (button_message, extra_message): (&str, Element<Message>) = match &self.status {
            Status::Idle => ("Install", column!().into()),
            Status::Installing => ("Installing...", column!().into()),
            Status::NoVersion => ("No version selected!", column!().into()),
            Status::Success(mods) => (
                "Finished!",
                match mods {
                    Some(mods) => {
                        let mut mod_string = String::new();
                        mod_string.push_str(&mods[0]);
                        for name in mods.iter().skip(1) {
                            mod_string.push_str(", ");
                            mod_string.push_str(name);
                        }
                        column![
                            text("The mods"),
                            text(mod_string).color(red).align_x(Horizontal::Center),
                            text("were unavailable.")
                        ]
                        .align_x(Alignment::Center)
                        .into()
                    }
                    None => column!().into(),
                },
            ),
            Status::Failure(message) => ("Error!", text(message).color(red).into()),
        };
        column![
            Space::with_height(Length::Fixed(10.0)),
            text("Enter Minecraft version:").size(20),
            Space::with_height(Length::Fixed(5.0)),
            pick_list(
                self.version_list.clone(),
                self.version.clone(),
                Message::Update
            )
            .width(Length::Fixed(200.0)),
            Space::with_height(Length::Fixed(5.0)),
            checkbox("Show snapshots", self.snapshot).on_toggle(Message::Snapshot),
            Space::with_height(Length::Fill),
            column![
                checkbox("Vanilla - Performance enhancing modlist.", self.vanilla,)
                    .on_toggle(Message::VButton),
                Space::with_height(Length::Fixed(5.0)),
                checkbox("Beauty - Immersive and beautiful modlist.", self.beauty,)
                    .on_toggle(Message::BButton),
                Space::with_height(Length::Fixed(5.0)),
                checkbox("Optifine - Optifine resource pack parity.", self.optifine,)
                    .on_toggle(Message::OButton),
            ]
            .align_x(Alignment::Start),
            Space::with_height(Length::Fill),
            extra_message,
            Space::with_height(Length::Fill),
            button(button_message).on_press(Message::Pressed),
            Space::with_height(Length::Fixed(10.0)),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    }

    fn theme(&self) -> Theme {
        Theme::custom(
            "Rosé Pine".to_string(),
            Palette {
                background: Color::from_rgb8(25, 23, 36),
                text: Color::from_rgb8(224, 222, 244),
                primary: Color::from_rgb8(235, 111, 146),
                success: Color::from_rgb8(156, 207, 216),
                danger: Color::from_rgb8(235, 111, 146),
            },
        )
    }
}

#[derive(Deserialize)]
struct Response {
    version: String,
}

async fn run(mc_version: String, modlists: (bool, bool, bool)) -> Result<Vec<String>> {
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

async fn populate(snapshots: bool) -> Vec<String> {
    let mut versions_list = Vec::new();
    let response: Vec<Versions> = reqwest::get("https://meta.quiltmc.org/v3/versions/game")
        .await
        .expect("Couldn't get versions.")
        .json()
        .await
        .unwrap();

    for value in response {
        if snapshots || value.stable {
            versions_list.push(value.version)
        }
    }
    versions_list
}
