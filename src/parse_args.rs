use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Stalcraft Clan Tool")]
pub struct Args {
    /// Путь к скриншоту для обработки
    #[arg(short, long, value_name = "FILE")]
    pub path: Option<PathBuf>,

    /// Использовать уведомления GNOME (libnotify)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub announce: bool,

    /// Включить расширенный вывод (доступно только в DEBUG сборке)
    #[cfg(debug_assertions)]
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}