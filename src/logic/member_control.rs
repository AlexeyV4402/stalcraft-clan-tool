use crate::net::stalcraft_api::Requester;
use crate::trace_log;
use crate::config::{Config, MAX_MEMBERS};
use std::collections::{HashMap, HashSet};
use crate::files::header::{read_header_file, save_header_file};
use crate::core::types::{Member};
use anyhow::{Result, bail};


pub fn update_member_list() -> Result<()> {
    let actual_header_path = &Config::global().header_file_path;
    
    let requester = Requester::default();
    let api_nicks = requester.clan_members()?;

    if api_nicks.is_empty() { 
        bail!("Пустой список людей");
    }

    let mut members = read_header_file(actual_header_path)?;
    trace_log!("[LOAD]: Loaded header_members: {:#?}", members);

    sync_members(&mut members, api_nicks)?;

    trace_log!("[CREATE]: Created new header file: {:#?}", members);

    save_header_file(actual_header_path, &members)?;

    Ok(())
}


/// header_members - Массив ячеек с заголовочного файла
/// api_nicks - Вектор ников, полученных с API
fn sync_members(header_members: &mut [Member], api_nicks: Vec<String>) -> Result<()> {
    // Создание словаря сопоставления <НИК - НОМЕР ЯЧЕЙКИ> из занятых ячеек заголовочного файла
    let mut header_nick_index: HashMap<String, usize> = header_members.iter().enumerate()
        .filter(|(_, m)| m.is_active)
        .map(|(i, m)| (m.nick.clone(), i))
        .collect();

    // Множество ников из API
    let api_set: HashSet<String> = api_nicks.into_iter().collect();

    // Очистка ранее занятых ячеек, которых сейчас нет в API
    for (nick, &idx) in &header_nick_index {
        if !api_set.contains(nick) {
            header_members[idx].is_active = false;
        }
    }
    // Добавление ников из API
    for nick in api_set {
        if !header_nick_index.contains_key(&nick) {
            if let Some(empty_idx) = header_members.iter().position(|m| !m.is_active) {
                header_members[empty_idx] = Member {
                    is_active: true,
                    nick: nick.clone(),
                    discord: String::default(),
                };
                header_nick_index.insert(nick, empty_idx);
            } else {
                bail!("More than {} members", MAX_MEMBERS);
            }
        }
    }
    Ok(())
}