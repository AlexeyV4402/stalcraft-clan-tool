use std::io::Write;

use crate::core::types::{ActiveData, AppContext};
use crate::files::cw_records::load_drafts;
use crate::logic::node_menu::{Node, process_menu};
use crate::{handle_cli_error, menu_tree};
use crate::config::Config;
use anyhow::Result;

use super::functions::print_grid;

pub enum ProcessDraftType {
    EDITNAME,
    EDITDATE,
    EDITRESERVE,
    EDITTOP15,
    EDITSUPPLY,
    EDITATTENDANCE,
    DUPLICATE,
    DELETE,
    WRITE
}

pub fn edit_draft(ctx: &mut AppContext, command: ProcessDraftType) {
    match command {
        ProcessDraftType::EDITNAME => {
            println!()
        },
        ProcessDraftType::EDITDATE => todo!(),
        ProcessDraftType::EDITRESERVE => todo!(),
        ProcessDraftType::EDITTOP15 => todo!(),
        ProcessDraftType::EDITSUPPLY => todo!(),
        ProcessDraftType::EDITATTENDANCE => todo!(),
        ProcessDraftType::DUPLICATE => todo!(),
        ProcessDraftType::DELETE => todo!(),
        ProcessDraftType::WRITE => todo!(),
    }
}

pub fn drafts_list(ctx: &mut AppContext) {
    let drafts = handle_cli_error!(load_drafts(&Config::global().cw_draft_path));
    let names: Vec<String> = drafts
        .iter()
        .map(|d| {
            // 1. Находим, где заканчивается текст (до первого \0)
            let pos = d.name.iter().position(|&b| b == 0).unwrap_or(d.name.len());
            
            // 2. Декодируем только живую часть в String
            String::from_utf8_lossy(&d.name[..pos]).into_owned()
        })
    .collect();
    print_grid(&names, 30, 3);


    let mut input = String::default();


    print!("\nВыберите номер (1-{}) или Enter для выхода: ", drafts.len());
    handle_cli_error!(std::io::stdout().flush());

    handle_cli_error!(std::io::stdin().read_line(&mut input));
    

    let choice: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= drafts.len() => num,
        _ => {
            println!("Выход");
            return;
        }
    };
    ctx.current_idx = choice - 1;
    ctx.data = crate::core::types::ActiveData::Drafts(drafts);
    handle_cli_error!(process_draft(ctx));
}


fn handle_edit_name(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::EDITNAME); }
fn handle_edit_data(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::EDITDATE); }
fn handle_edit_reserve(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::EDITRESERVE); }
fn handle_edit_top15(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::EDITTOP15); }
fn handle_edit_supply(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::EDITSUPPLY); }
fn handle_edit_attendance(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::EDITATTENDANCE); }
fn handle_duplicate(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::DUPLICATE); }
fn handle_delete(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::DELETE); }
fn handle_write(ctx: &mut AppContext) { let _ = edit_draft(ctx, ProcessDraftType::WRITE); }


pub fn process_draft(ctx: &mut AppContext) -> Result<()> {
    let mut list: Vec<Node> = Vec::new();

    let root_text = "\
        Выберите режим работы:\n\
        ".to_string();
    
    if let ActiveData::Drafts(d) = &ctx.data {
        println!("{}", d[ctx.current_idx]);
    }

    let root_id = menu_tree!(@insert list, Option::<usize>::None, root_text, None);

     menu_tree!(list, Some(root_id), {
        "Редактировать название" => handle_edit_name,
        "Редактировать дату" => handle_edit_data,
        "Редактировать запас" => handle_edit_reserve,
        "Редактировать топ 15" => handle_edit_top15,
        "Редактировать получивших расходники" => handle_edit_supply,
        "Редактировать посещаемость" => handle_edit_attendance,
        "Дублировать черновик" => handle_duplicate,
        "Удалить черновик" => handle_delete,
        "Записать черновик" => handle_write
    });

    process_menu(ctx, &list)?;
    Ok(())
}


// pub fn add_stage() {
//     print!("Введите дату этапа в формате ДД.ММ.ГГ (Enter для сегодня): ");
//     std::io::stdout().flush().expect("[OS ERROR]: Can't flush output");
    
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("[OS ERROR]: Can't read input");
//     let input = input.trim();

//     let timestamp = if input.is_empty() {
//         let today_local = chrono::Local::now().date_naive();
//         today_local.and_hms_opt(0, 0, 0)
//             .unwrap()
//             .and_utc()
//             .timestamp()
//     } else {
//         match NaiveDate::parse_from_str(input, "%d.%m.%y") {
//             Ok(d) => {
//                 d.and_hms_opt(0, 0, 0)
//                     .expect("[DATA ERROR]: Incorrect time")
//                     .and_utc()
//                     .timestamp()
//             },
//             Err(_) => {
//                 println!("Ошибка формата! Использую текущую дату.");
//                 chrono::Local::now().date_naive()
//                     .and_hms_opt(0, 0, 0)
//                     .unwrap()
//                     .and_utc()
//                     .timestamp()
//             }
//         }
//     };

//     let header_members = read_header_file(&Config::global().header_file_path).expect("[OS ERROR]: Can't load header file");
//     trace_log!("[LOAD]: Loaded header_members: {:#?}", header_members);

//     let real_indices: Vec<usize> = header_members.iter().enumerate()
//             .filter(|(_, m)| m.is_active)
//             .map(|(idx, _)| idx)
//             .collect();

//     let membership = get_membership_mask(&real_indices);

//     println!("Выберите отсутствующих:");
//     let attendance = input_mask(&header_members, &real_indices, membership);

//     println!("Выберите не получивших закуп:");
//     let supply = input_mask(&header_members, &real_indices, membership);

//     let record = KVRecord{
//         timestamp,
//         membership,
//         attendance,
//         supply
//     };
//     trace_log!("[CREATE]: Created KVRecord: {:#?}", record);

//     save_kv_record(&Config::global().kv_history_path, record).expect("[OS ERROR]: Can't update KV data");
// }


// pub fn duplicate_last_stage() {
//     duplicate_last_kv(
//         &Config::global().kv_history_path, 
//         chrono::Local::now()
//             .date_naive()
//             .and_hms_opt(0, 0, 0)
//             .unwrap()
//             .and_utc()
//             .timestamp())
//         .expect("[OS ERROR]: Can't save KVRecord");
// }


// fn toggle_bit_by_index(mask: &mut u64, display_num: usize, indices: &[usize], state: bool) {
//     // Проверка границ: display_num — это то, что ввел юзер (1..N)
//     if display_num > 0 && display_num <= indices.len() {
//         let bit_pos = indices[display_num - 1]; // Получаем реальный бит 0..34
//         if state {
//             *mask |= 1 << bit_pos;  // Включить (1)
//         } else {
//             *mask &= !(1 << bit_pos); // Выключить (0)
//         }
//     }
// }


// fn get_membership_mask(indices: &[usize]) -> u64 {
//     let mut mask = 0u64;
//     for &real_idx in indices {
//         mask |= 1 << real_idx;
//     }
//     mask
// }


// pub fn input_mask(members: &[Member], real_indices: &[usize], membership_mask: u64) -> u64 {

//     let nick_to_idx: std::collections::HashMap<String, usize> = real_indices.iter()
//         .map(|&idx| (members[idx].nick.to_lowercase(), idx))
//         .collect();

//     println!("\n--- Список для отметки ---");
//     let display_list: Vec<String> = real_indices.iter()
//         .map(|&idx| {
//             let m = &members[idx];
//             format!("{}", m.nick)
//         })
//         .collect();

//     print_grid(&display_list, 24, 3);

//     println!("\n[РЕЖИМЫ]: '1-5 !3' (номера) ИЛИ 'Ziv Asunaro !Ace' (ники)");
//     print!("Ввод: ");
//     std::io::stdout().flush().unwrap();

//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("Read error");
//     let tokens: Vec<&str> = input.trim().split_whitespace().collect();

//     if tokens.is_empty() { return membership_mask; } // По умолчанию были все

//     let mut exclude_mask: u64 = 0;

//     for token in tokens {
//         let is_remove = token.starts_with('!');
//         let clean_token = token.trim_start_matches('!');
        
//         // 1. Сначала проверяем на диапазон (есть дефис и начинается с цифры)
//         if clean_token.contains('-') && clean_token.chars().next().unwrap_or(' ').is_ascii_digit() {
//             let parts: Vec<&str> = clean_token.split('-').collect();
//             if let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
//                 for num in start..=end {
//                     toggle_bit_by_index(&mut exclude_mask, num, &real_indices, !is_remove);
//                 }
//                 continue; // Переходим к следующему токену
//             }
//         }

//         // 2. Проверяем на одиночное число
//         if let Ok(num) = clean_token.parse::<usize>() {
//             toggle_bit_by_index(&mut exclude_mask, num, &real_indices, !is_remove);
//         } 
//         // 3. Иначе — это ник
//         else {
//             if let Some(&bit_pos) = nick_to_idx.get(&clean_token.to_lowercase()) {
//                 if is_remove { exclude_mask &= !(1 << bit_pos); } 
//                 else { exclude_mask |= 1 << bit_pos; }
//             } else {
//                 println!("! Игрок '{}' не найден", clean_token);
//             }
//         }
//     }

//     // ИТОГ: Из состава вычитаем отмеченных (прогульщиков)
//     membership_mask & !exclude_mask
// }