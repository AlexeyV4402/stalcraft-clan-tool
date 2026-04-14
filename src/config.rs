use serde::{Deserialize, Deserializer};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::fs;

pub const MAX_MEMBERS: usize = 35;

pub const NICK_LEN: usize = 16;
pub const DISCORD_LEN: usize = 32;
pub const NICK_LEN_BYTES: usize = NICK_LEN * 2;
pub const DISCORD_LEN_BYTES: usize = DISCORD_LEN * 2;

pub const DEMO_API: bool = false;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub stalcraft_api_token: String,
    pub stalcraft_clan_id: String,
    pub stalcraft_region: String,
    #[serde(deserialize_with = "expand_tilde_path")]
    pub header_file_path: PathBuf,
    #[serde(deserialize_with = "expand_tilde_path")]
    pub cw_history_path: PathBuf,
    #[serde(deserialize_with = "expand_tilde_path")]
    pub parties_file_path: PathBuf,
    #[serde(deserialize_with = "expand_tilde_path")]
    pub cw_draft_path: PathBuf,
    #[serde(deserialize_with = "expand_tilde_path")]
    pub screenshot_path: PathBuf,
    pub screenshot_left_indent: u32,
    pub screenshot_right_indent: u32,
    pub max_threads: usize
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    // Функция для первой загрузки
    pub fn init() {
        let content = fs::read_to_string(expand_tilde("~/Documents/Stalcraft/Config.toml"))
            .expect("[CONFIG ERROR]: Не удалось найти Config.toml");
        let decoded: Config = toml::from_str(&content)
            .expect("[CONFIG ERROR]: Ошибка в формате TOML");
        
        CONFIG.set(decoded).expect("Попытка повторной инициализации конфига");
    }

    // Функция для получения ссылки на данные в любом месте кода
    pub fn global() -> &'static Config {
        CONFIG.get().expect("[CONFIG ERROR]: Конфиг не инициализирован!")
    }
}


fn expand_tilde_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    // 1. Читаем строку из TOML
    let s = String::deserialize(deserializer)?;
    Ok(expand_tilde(s))
}



pub fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if !path.starts_with("~") {
        return path.to_path_buf();
    }

    match home::home_dir() {
        Some(home) => {
            // Убираем "~" и соединяем с домашней папкой
            let without_tilde = path.strip_prefix("~").unwrap_or(path);
            home.join(without_tilde)
        }
        None => path.to_path_buf(),
    }
}