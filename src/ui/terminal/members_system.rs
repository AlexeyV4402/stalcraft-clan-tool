use super::functions::print_grid;
use crate::core::types::AppContext;
use crate::logic::member_control::update_member_list;
use crate::trace_log;
use crate::config::{Config};
use std::{io::Write};
use crate::files::header::{read_header_file, save_header_file};
use crate::handle_cli_error;


pub fn edit_additional_info(_: &mut AppContext) {
    let actual_header_path = &Config::global().header_file_path;

    let mut header_members = handle_cli_error!(read_header_file(actual_header_path));
    trace_log!("[LOAD]: Loaded header_members: {:#?}", header_members);
    let mut input = String::new();

    loop {
        println!("Выберите номер игрока; (_ => leave): ");

        // Реальные позиции непустых ячеек в заголовочном файле
        let real_indices: Vec<usize> = header_members.iter().enumerate()
            .filter(|(_, m)| m.is_active)
            .map(|(idx, _)| idx)
            .collect();

        let display_list: Vec<String> = real_indices.iter()
            .map(|&idx| {
                let m = &header_members[idx];
                format!("{} — {}", m.nick, m.discord)
            })
            .collect();

        print_grid(&display_list, 48, 2);

        print!("\nВыберите номер (1-{}) или Enter для выхода: ", real_indices.len());
        handle_cli_error!(std::io::stdout().flush());

        input.clear();
        handle_cli_error!(std::io::stdin().read_line(&mut input));
        

        let choice: usize = match input.trim().parse() {
            Ok(num) if num > 0 && num <= real_indices.len() => num,
            _ => {
                println!("Сохранение и выход...");

                handle_cli_error!(save_header_file(actual_header_path, &header_members));
                return;
            }
        };

        let target_idx = real_indices[choice - 1];
        
        println!("Введите Discord для {}: ", header_members[target_idx].nick);
        input.clear();
        handle_cli_error!(std::io::stdin().read_line(&mut input));
        
        let new_discord = input.trim();
        if !new_discord.is_empty() {
            header_members[target_idx].discord = new_discord.to_string();
            println!("Обновлено!");
        }
    }

}


pub fn input_additional_info(_: &mut AppContext) {
    let actual_header_path = &Config::global().header_file_path;

    let mut header_members = handle_cli_error!(read_header_file(actual_header_path));
    trace_log!("[LOAD]: Loaded header_members: {:#?}", header_members);

    println!(
        "Далее представленны ники людей из клана;\n\
        Вводите к каждому нику username из discord;\n\
        Для пропуска укажите любой текст короче 2-х символов\
        "
    );

    let mut input = String::new();
    header_members.iter_mut()
        .filter(|m| m.is_active && m.discord.is_empty()) 
        .for_each(|m| {
            print!("{}: ", m.nick);
            handle_cli_error!(std::io::stdout().flush());
            handle_cli_error!(std::io::stdin().read_line(&mut input));
            if input.len() >= 2 {
                m.discord = input.trim().to_string();
            }
            input.clear();
        }
    );

    handle_cli_error!(save_header_file(actual_header_path, &header_members));
}


pub fn update_members(_: &mut AppContext) {
    handle_cli_error!(update_member_list());
    println!("Данные успешно обновлены");
}