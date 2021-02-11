use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Display {
    Wayland,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Capturer {
    Wlroots,
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Processor {
    Vulkan,
}

#[derive(Deserialize, Debug)]
pub struct ScreenContents {
    pub capturer: Capturer,
    pub processor: Processor,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Als {
    Iio { path: String },
    None,
    Time { hour_to_lux: HashMap<String, u32> },
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct OutputByType {
    pub backlight: HashMap<String, BacklightOutput>,
    pub ddcutil: HashMap<String, DdcUtilOutput>,
}

#[derive(Deserialize, Debug)]
pub struct BacklightOutput {
    pub path: String,
    pub use_contents: bool,
}

#[derive(Deserialize, Debug)]
pub struct DdcUtilOutput {
    pub display: u8,
    pub use_contents: bool,
}

#[derive(Deserialize, Debug)]
pub enum Output {
    Backlight(BacklightOutput),
    DdcUtil(DdcUtilOutput),
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Keyboards {
    pub backlight: HashMap<String, Keyboard>,
}

#[derive(Deserialize, Debug)]
pub struct Keyboard {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub display: Display,
    pub screen_contents: ScreenContents,
    pub als: Als,
    #[serde(rename = "output")]
    output_by_type: OutputByType,
    pub keyboard: Keyboards,
    #[serde(skip)]
    pub output: HashMap<String, Output>,
}

impl Config {
    pub fn load() -> Result<Self, toml::de::Error> {
        let config = dirs::config_dir()
            .and_then(|config_dir| fs::read_to_string(&config_dir.join("wluma/config.toml")).ok())
            .unwrap_or(include_str!("../config.toml").to_string());

        toml::from_str(&config).map(|mut cfg: Self| {
            cfg.output = cfg
                .output_by_type
                .backlight
                .into_iter()
                .map(|(name, output)| (name, Output::Backlight(output)))
                .chain(
                    cfg.output_by_type
                        .ddcutil
                        .into_iter()
                        .map(|(name, output)| (name, Output::DdcUtil(output))),
                )
                .collect();
            cfg.output_by_type = OutputByType::default();
            cfg
        })
    }

    pub fn use_output_contents(&self, name: &str) -> bool {
        self.output
            .get(name)
            .and_then(|o| match o {
                Output::Backlight(o) => Some(o.use_contents),
                Output::DdcUtil(o) => Some(o.use_contents),
            })
            .unwrap_or(false)
    }
}
