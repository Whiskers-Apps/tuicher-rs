use std::{
    error::Error,
    io::{self, Read, Write},
    path::PathBuf,
    process::exit,
};

use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};

use crate::result::TUIResult;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: String,
    pub settings_file: Option<PathBuf>, // Aceita ~ e ./
}

#[derive(Serialize, Deserialize, Decode, Encode)]
pub enum PluginAction {
    Results {
        text: String,
    },
    Run {
        custom_action: String,
        info: Option<Vec<String>>,
    },
}

pub fn get_plugin_action() -> Result<PluginAction, Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut buffer = Vec::new();

    stdin.read_to_end(&mut buffer)?;

    let (action, _): (PluginAction, usize) =
        bincode::decode_from_slice(&buffer, config::standard())?;

    Ok(action)
}

pub fn send_plugin_results(results: &Vec<TUIResult>) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    let bytes = bincode::encode_to_vec(results, config::standard())?;
    stdout.write_all(&bytes)?;
    stdout.flush()?;

    exit(0);
}
