use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::mem;
use std::path::Path;
use anyhow::{Context, Result};
use fs2::FileExt;
use crate::core::types::{CWRecord, CWDraft};
use crate::{as_u8_slice, trace_log};

// Гранаты - 9 видов
// Подвижность - 4 вида
// Защита - 2 вида
// Усиление - 15 видов (без ИРП)
// Кратковременное усиление - 2 вида

pub fn save_cw_record(path: &Path, record: CWRecord) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    file.lock_exclusive().context("[OS ERROR] Файл занят")?;
    
    let bytes = as_u8_slice!(&record);

    file.write_all(bytes)?;
    Ok(())
}


pub fn duplicate_last_cw(path: &Path, new_timestamp: i64) -> Result<()> {
    let mut file = std::fs::File::open(path)?;
    let size = std::mem::size_of::<CWRecord>() as i64;

    file.lock_exclusive().context("[OS ERROR] Файл занят")?;

    file.seek(SeekFrom::End(-size))?;

    let mut buf = vec![0u8; size as usize];
    file.read_exact(&mut buf)?;
    drop(file);


    let mut record: CWRecord = unsafe { std::ptr::read(buf.as_ptr() as *const _) };

    record.timestamp = new_timestamp;

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)?;

    file.lock_exclusive().context("[OS ERROR] Файл занят")?;

    let bytes = as_u8_slice!(&record);

    file.write_all(bytes)?;
    Ok(())
}


pub fn save_cw_draft(path: &Path, draft: CWDraft) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path).context("Ошибка сохранения черновика")?;

    file.lock_exclusive().context("[OS ERROR] Файл занят")?;

    let bytes = as_u8_slice!(&draft);

    trace_log!("Сохраняем черновики: {:#?}", draft);
    

    file.write_all(bytes)?;
    Ok(())
}


pub fn load_drafts(path: &Path) -> Result<Vec<CWDraft>> {
    let mut file = File::open(path).context("Не удалось открыть файл черновиков")?;

    file.lock_exclusive().context("[OS ERROR] Файл занят")?;

    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;
    unsafe {
        let struct_size = mem::size_of::<CWDraft>();

        if bytes.len() % struct_size != 0 {
            anyhow::bail!("[DATA ERROR] Файл черновиков повреждён");
        }

        let ptr = bytes.as_mut_ptr() as *mut CWDraft;
        let length = bytes.len() / struct_size;
        let capacity = bytes.capacity() / struct_size;

        mem::forget(bytes);

        Ok(Vec::from_raw_parts(ptr, length, capacity))
    }
}