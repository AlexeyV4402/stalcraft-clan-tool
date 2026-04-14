use crate::config::{NICK_LEN_BYTES, DISCORD_LEN_BYTES, MAX_MEMBERS};
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;
use anyhow::{Context, Result};
use fs2::FileExt;
use crate::core::types::Member;
use crate::core::tools::mem::fill_fixed_vec;


pub fn save_header_file(path: &Path, members: &[Member]) -> Result<()> {
    let mut file = File::create(path).with_context(|| {format!("Ошибка сохранения заголовка:")})?;
    file.lock_exclusive().context("[OS ERROR] Файл занят")?;
    for i in 0..MAX_MEMBERS {
        if let Some(m) = members.get(i) {
            file.write_all(&[if m.is_active { 1 } else { 0 }])?;
            file.write_all(&fill_fixed_vec(&m.nick, NICK_LEN_BYTES))?;
            file.write_all(&fill_fixed_vec(&m.discord, DISCORD_LEN_BYTES))?;
        } else {
            file.write_all(&[0])?;
            file.write_all(&vec![0u8; NICK_LEN_BYTES])?;
            file.write_all(&vec![0u8; DISCORD_LEN_BYTES])?;
        }
    }
    
    file.sync_all()?;
    Ok(())
}


pub fn read_header_file(path: &Path) -> Result<[Member; MAX_MEMBERS]> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok(std::array::from_fn(|_| Member {
                is_active: false,
                nick: String::new(),
                discord: String::new(),
            }));
        }
        Err(e) => return Err(anyhow::Error::new(e).context("Ошибка сохранения заголовка")),
    };

    file.lock_exclusive().context("[OS ERROR] Файл занят")?;

    let record_size = 1 + NICK_LEN_BYTES + DISCORD_LEN_BYTES;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer.len() < MAX_MEMBERS * record_size {
        anyhow::bail!("Файл базы данных слишком короткий: ожидалось {} байт, найдено {}", MAX_MEMBERS * record_size, buffer.len());
    }

    // 2. Парсим данные из буфера в массив
    let mut chunks = buffer.chunks_exact(record_size);
    let members: [Member; MAX_MEMBERS] = std::array::from_fn(|_| {
        let chunk = chunks.next().unwrap();
        
        Member {
            is_active: chunk[0] == 1,
            nick: String::from_utf8_lossy(&chunk[1..1 + NICK_LEN_BYTES]).trim_matches('\0').to_string(),
            discord: String::from_utf8_lossy(&chunk[1 + NICK_LEN_BYTES..]).trim_matches('\0').to_string()
        }
    });

    Ok(members)
}