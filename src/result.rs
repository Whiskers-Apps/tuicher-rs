use std::path::{Path, PathBuf};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct TUIResult {
    pub icon_path: Option<PathBuf>,
    pub text: String,
    pub secondary_text: Option<String>,
    pub action: Option<Action>,
    pub info: String,
}

impl TUIResult {
    pub fn new<S: AsRef<str>>(text: S, info: S) -> Self {
        Self {
            text: text.as_ref().to_string(),
            secondary_text: None,
            action: None,
            icon_path: None,
            info: info.as_ref().to_string(),
        }
    }

    pub fn set_icon_path<P: AsRef<Path>>(&mut self, path: P) -> Self {
        self.icon_path = Some(path.as_ref().to_path_buf());
        self.clone()
    }

    pub fn set_secondary_text<S: AsRef<str>>(&mut self, text: S) -> Self {
        self.secondary_text = Some(text.as_ref().to_string());
        self.clone()
    }

    pub fn set_action(&mut self, action: Action) -> Self {
        self.action = Some(action);
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum Action {
    OpenApp(OpenApp),
    OpenFile(OpenFile),
    OpenURL(OpenURL),
    CopyText(CopyText),
    CopyImage(CopyImage),
    ShowResults(ShowResults),
    OpenSettings,
    Session(Session),
    Bookmark(Bookmark),
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct OpenApp {
    pub path: PathBuf,
}

impl OpenApp {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct OpenFile {
    pub path: PathBuf,
}

impl OpenFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct OpenURL {
    pub url: String,
}

impl OpenURL {
    pub fn new<S: AsRef<str>>(url: S) -> Self {
        Self {
            url: url.as_ref().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct CopyText {
    pub text: String,
}

impl CopyText {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct CopyImage {
    pub path: PathBuf,
}

impl CopyImage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct ShowResults {
    pub results: Vec<TUIResult>,
}

impl ShowResults {
    pub fn new(results: Vec<TUIResult>) -> Self {
        Self { results: results }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum Session {
    Shutdown,
    Restart,
    Suspend,
    Logout,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub enum Bookmark {
    Add(AddBookmark),
    Remove(RemoveBookmark),
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct AddBookmark {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug)]
pub struct RemoveBookmark {
    pub id: usize,
}
