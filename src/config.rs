use std::{error::Error, fs, path::PathBuf};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub plugins: Vec<PluginConfig>,
    pub search_engines: Vec<SearchEngine>,
    pub theme: Theme,
    pub height: u16,
    pub width: u16,
    pub default_search_engine: usize,
    pub bookmarks: Vec<BookmarkConfig>,
    pub emojis_keyword: String,
    pub enable_emojis: bool,
    pub bookmarks_keyword: String,
    pub enable_bookmarks: bool,
    pub session_manager_keyword: String,
    pub enable_session_manager: bool,
    pub show_bookmarks_favicon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub id: String,
    pub keyword: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub id: usize,
    pub keyword: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub background: String,
    pub secondary: String,
    pub tertiary: String,
    pub disabled: String,
    pub text: String,
    pub text_secondary: String,
    pub text_tertiary: String,
    pub on_text: String,
    pub warning: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookmarkConfig {
    pub id: usize,
    pub name: String,
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugins: vec![],
            search_engines: vec![
                SearchEngine {
                    id: 0,
                    keyword: "ec".to_string(),
                    name: "Ecosia".to_string(),
                    url: "https://www.ecosia.org/search?method=index&q=%s".to_string(),
                },
                SearchEngine {
                    id: 1,
                    keyword: "gs".to_string(),
                    name: "Google".to_string(),
                    url: "https://www.google.com/search?q=%s".to_string(),
                },
                SearchEngine {
                    id: 2,
                    keyword: "ds".to_string(),
                    name: "DuckDuckGo".to_string(),
                    url: "https://duckduckgo.com/?q=%s".to_string(),
                },
            ],
            theme: Theme::default(),
            height: 800,
            width: 900,
            default_search_engine: 0,
            bookmarks: vec![],
            enable_emojis: true,
            enable_bookmarks: true,
            enable_session_manager: true,
            emojis_keyword: "em".to_string(),
            bookmarks_keyword: "bm".to_string(),
            session_manager_keyword: "sm".to_string(),
            show_bookmarks_favicon: true,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: "#141414".to_string(),
            secondary: "#1F1F1F".to_string(),
            tertiary: "#383838".to_string(),
            disabled: "#1F1F1F".to_string(),
            text: "#F2F2F2".to_string(),
            text_secondary: "#E5E5E5".to_string(),
            text_tertiary: "#C2C2C2".to_string(),
            on_text: "#000000".to_string(),
            warning: "#FFAD72".to_string(),
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Failed to get config dir")?
        .join("tuicher");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let path = get_config_dir()?.join("config.json");

    if !path.exists() {
        let json = serde_json::to_string(&Config::default())?;
        fs::write(&path, &json)?;

        debug!("Wrote config file");

        return Ok(Config::default());
    }

    let bytes = fs::read(&path)?;
    let config: Config = serde_json::from_slice(&bytes)?;

    Ok(config)
}

pub fn write_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = get_config_dir()?.join("config.json");
    let json = serde_json::to_string(config)?;
    fs::write(&path, &json)?;

    return Ok(());
}
