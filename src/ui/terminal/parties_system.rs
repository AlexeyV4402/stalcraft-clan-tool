use std::fs::File;
use std::io::BufReader;
use std::{collections::HashSet};

use xlsxwriter::{Workbook};
use calamine::{Reader, DataType, Xlsx, open_workbook};
use super::functions::print_grid;
use crate::core::types::AppContext;
use crate::files::header::read_header_file;
use crate::config::Config;
use crate::handle_cli_error;


pub fn analyse_file(_: &mut AppContext) {
    let raw_input: Xlsx<BufReader<File>> = handle_cli_error!(open_workbook(&Config::global().parties_file_path));

    let stem = Config::global().parties_file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = Config::global().parties_file_path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let save_path = Config::global().parties_file_path.with_file_name(format!("{} (Edited).{}", stem, extension));

    let mut input: Xlsx<_> = raw_input;

    
    let header_members = handle_cli_error!(read_header_file(&Config::global().header_file_path));

    let mut nicknames: HashSet<String> = header_members.into_iter()
        .filter(|m| m.is_active)
        .map(|m| m.nick.to_lowercase())
        .collect();
        
    let mut file_nicknames: HashSet<String> = HashSet::new();

    let range = handle_cli_error!(input.worksheet_range("Parties"));

    let output = handle_cli_error!(Workbook::new(&save_path.to_string_lossy()));
    let mut sheet = handle_cli_error!(output.add_worksheet(Some("Parties")));

    let mut text_to_write;
    
    for (x, y, data) in range.cells() {
        let cell_content = data.as_string().unwrap_or(" ".to_string()); 

        if x > 0 && y > 0 && x < 8 && y < 8 {
            let clean_text = cell_content.trim().to_lowercase();
            
            if !nicknames.contains(&clean_text) {
                text_to_write = String::new();
            } else {
                // insert возвращает false, если значение уже было (дубликат)
                if file_nicknames.insert(clean_text) {
                    text_to_write = cell_content; // Сохраняем оригинал (регистр)
                } else {
                    text_to_write = String::new(); // Дубликат — очищаем
                }
            }
        } else {
            text_to_write = cell_content;
        }
        
        handle_cli_error!(sheet.write_string(x as u32, y as u16, &text_to_write, None));
    }

    // Вывод тех, кого забыли вписать в таблицу
    nicknames.retain(|m| !file_nicknames.contains(m));
    
    let missing_list: Vec<String> = nicknames.into_iter().collect();
    if missing_list.is_empty() {
        println!("Все участники распределены по отрядам!");
    } else {
        println!("Люди, отсутствующие в отрядах:");
        print_grid(&missing_list, 24, 2);
    }
    handle_cli_error!(output.close());
}