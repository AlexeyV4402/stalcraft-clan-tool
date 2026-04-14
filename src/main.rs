mod core;
mod files;
mod logic;
mod net;
mod ui;
mod config;
mod parse_args;

use std::path::PathBuf;

use clap::Parser;
use rayon::ThreadPoolBuilder;

use crate::core::types::AppContext;
use crate::files::header::read_header_file;
use crate::logic::node_menu::*;
use crate::core::debug;

use crate::config::Config;
use crate::parse_args::Args;

use crate::logic::ocr::process_screenshot;
use crate::ui::terminal::members_system::edit_additional_info;
use crate::ui::terminal::members_system::input_additional_info;
use crate::ui::terminal::members_system::update_members;
use crate::ui::terminal::parties_system::analyse_file;
use crate::ui::terminal::stages_system::drafts_list;
use crate::ui::terminal::process_screen::input_screenshot;

use crate::ui::linux::send_notification;


pub fn test(ctx: &mut AppContext) {
    let members = read_header_file(&PathBuf::from("~/Documents/Stalcraft/Data/Header")).unwrap();
    let mask: u64 = !0b00000000111001111111011011111111110;
    let present_nicks: Vec<&str> = members.iter().enumerate()
        .filter(|(i, _)| (mask >> i) & 1 == 1)
        .map(|(_, m)| m.nick.as_str())
        .collect();
    println!("{:#?}", present_nicks);
}


fn main() {
    Config::init();

    let mut ctx = AppContext::default();

    ThreadPoolBuilder::new()
        .num_threads(Config::global().max_threads) // Ограничиваем количество одновременно работающих ядер
        .thread_name(|i| format!("ocr-thread-{}", i)) // Опционально: даем имена для отладки
        .build_global()
        .expect("Не удалось инициализировать пул потоков Rayon");

    let args = Args::parse();

    let mut run_app = true;
    let mut confidence= Ok(100);

    #[cfg(debug_assertions)]
    if args.debug {
        debug::DEBUG_ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    if let Some(screenshot_path) = args.path {
        confidence = process_screenshot(&screenshot_path);
        run_app = false;
    }

    if args.announce {
        let body: String;
        if let Ok(conf) = confidence {
            if conf == 100 {
                body = "Черновик КВ успешно сохранён".to_string()
            } else {
                body = format!("Черновик сохранён.\nТочность: {}%", conf)
            }
        } else {
            body = format!("Ошибка сохранения: {}", confidence.err().unwrap())
        }
        send_notification("StalcraftClanTool", &body, "stalcraft_clan_tool");
    }

    if !run_app {
        return;
    }
    
    let list = fill_list();
    process_menu(&mut ctx, &list).unwrap();
}


fn fill_list() -> Vec<Node> {
    let mut list: Vec<Node> = Vec::new();

    let root_text = "\
        Приветствуем вас в StalcraftClanTool - инструмент для оптимизации работы с кланом.\n\
        Запомните, что _ в меню выбора означает любой не представленный вариант.\n\
        Выберите режим работы:\n".to_string();
    let root_id = menu_tree!(@insert list, Option::<usize>::None, root_text, None);

    menu_tree!(list, Some(root_id), {
        "Участники" => {
            "Обновить из API" => update_members,
            "Редактировать дополнительные данные" => edit_additional_info,
            "Ввести дополнительные данные" => input_additional_info,
            "Тестовая функция" => test
        },
        "Отряды" => {
            "Оптимизация таблицы" => analyse_file
        },
        "Этапы" => {
            "Черновики" => drafts_list,
            "Загрузить скриншот" => input_screenshot
        }
    });
    return list;
}