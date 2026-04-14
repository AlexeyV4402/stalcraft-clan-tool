use image::{DynamicImage, GenericImageView, Pixel};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tesseract::Tesseract;
use crate::core::types::Member;
use crate::files::header::read_header_file;
use crate::files::process_screen::load_screenshot;
use crate::trace_log;
use crate::core::tools::string::normalize_nick;
use strsim::levenshtein;
use crate::config::Config;
use crate::core::types::{CWDraft, CWType};
use crate::core::tools::time::get_ru_day;
use crate::core::tools::mem::fill_fixed_array;
use crate::files::cw_records::save_cw_draft;
use std::cell::RefCell;
use std::path::Path;
use chrono::{DateTime, Datelike, NaiveDateTime, NaiveTime, Utc}; 
use anyhow::Result;


pub fn process_screenshot(path: &Path) -> Result<u8> {

    let (attendance, top15, confidence) = parse_screenshot(load_screenshot(path)?)?;

    let file_name = Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let datetime_utc = match NaiveDateTime::parse_from_str(file_name, "CW_UTC_%Y%m%d_%H%M%S.png") {
        Ok(d) => {
            d.and_utc()
        },
        Err(_) => {
            chrono::Utc::now()
        }
    };

    let draft = CWDraft{
        timestamp: datetime_utc.timestamp(),
        reserve: 0u64,
        top15: top15,
        supply: 0u64,
        attendance: attendance,
        name: fill_fixed_array(&get_name(datetime_utc, confidence)),
        file_name: fill_fixed_array(&file_name)
    };

    save_cw_draft(&Config::global().cw_draft_path, draft)?;

    Ok(confidence)
}


pub fn parse_screenshot(img: DynamicImage) -> Result<(u64, u64, u8)> {
    let (w, h) = img.dimensions();

    let (l, r) = (Config::global().screenshot_left_indent, Config::global().screenshot_right_indent);

    let img = img.crop_imm(l, 0, w - l - r, h);
    
    let w = w - l - r;

    let row_h = 30;

    let threshold = 60;
    let mut actual_rows = 0;

    while (actual_rows + 1) * row_h <= h {
        let center_y = actual_rows * row_h + (row_h / 2);
        
        let pixel = img.get_pixel(w / 2, center_y).to_luma();
        if pixel[0] > threshold {
            actual_rows += 1;
        } else {
            break;
        }
    }

    let indices: Vec<u32> = (0..actual_rows).collect();

    let nicks: Result<Vec<String>> = indices.into_par_iter().map(|i| {
        let y_offset = i * row_h;
        let cropped = &img.crop_imm(0, y_offset, w, row_h);
        let resized_luma = image::imageops::resize(
        &cropped.to_luma8(),
        w * 2, 
        row_h * 2,
        image::imageops::FilterType::Lanczos3
        );

        #[cfg(debug_assertions)]
        {if crate::debug::DEBUG_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            resized_luma.save(format!("/home/AlexeyV/Documents/Rust/StalcraftClanTool/Data/test_{}.png", y_offset/row_h))?;
        }}
        
        extract_single_nick(&resized_luma)
    }).collect();

    let members = read_header_file(&Config::global().header_file_path).expect("[OS ERROR]: Can't load header file");

    let a = match_cw_nicks(nicks?, &members);

    let confidence = (a.2 as u16 * 100) / (actual_rows as u16);
    
    Ok((a.0, a.1, confidence as u8))
}


pub fn match_cw_nicks(nicks: Vec<String>, members: &[Member]) -> (u64, u64, u8) {
    let mut attendance_mask = 0u64;
    let mut top15_mask = 0u64;
    let mut counter = 0;
    
    for name in &nicks {
        let best_match = members.iter().enumerate()
            .filter(|(_, m)| m.is_active)
            .map(|(idx, m)| (idx, levenshtein(&normalize_nick(&name).to_lowercase(), &normalize_nick(&m.nick.to_lowercase()))))
            .min_by_key(|&(_, dist)| dist);

        if let Some((idx, dist)) = best_match {
            if dist <= 2 {
                attendance_mask |= 1 << idx;
                if counter == 14 {
                    top15_mask = attendance_mask;
                }
                trace_log!("[OCR] Нашли: {} -> {} (дист: {})", name, members[idx].nick, dist);
                counter += 1;
            }
        }
    }

    (attendance_mask, top15_mask, counter)
}


fn get_name(datetime_utc: DateTime<Utc>, confidence: u8) -> String {
    let cw_start = datetime_utc.with_time(NaiveTime::from_hms_opt(17, 0, 0).unwrap()).unwrap();

    let warm_up = chrono::Duration::minutes(5);

    let cw_type: CWType = match datetime_utc.weekday().num_days_from_monday() {
        0..=2 => CWType::BRAWL,
        3..=5 => CWType::TOURNAMENT,
        6 => CWType::BRAWL,
        _ => panic!("[OS/LIB ERROR]: More than 7 days in week")
    };

    let cw_duration = match cw_type {
        CWType::BRAWL => chrono::Duration::minutes(15),
        CWType::TOURNAMENT => chrono::Duration::minutes(20),
        CWType::BASECAPTURING => chrono::Duration::minutes(20)
    };

    let day_name = get_ru_day(datetime_utc);

    let delta = datetime_utc - cw_start;
    let total_seconds = delta.num_seconds();

    // Если время отрицательное или слишком большое (например, > 2 часов), это "Свободное время"
    if total_seconds < 0 || total_seconds > (warm_up + cw_duration).num_seconds() * 3 {
        return format!("{}: Свободное время", day_name);
    }

    // 2. Длительность одного полного цикла (Разминка + Игра)
    let cycle_secs = (warm_up + cw_duration).num_seconds();

    // 3. Вычисляем номер этапа (0, 1, 2) -> превращаем в (1, 2, 3)
    let stage_idx = (total_seconds / cycle_secs) + 1;

    // 4. Вычисляем остаток внутри текущего этапа
    let time_in_stage = total_seconds % cycle_secs;

    // 5. Определяем фазу
    let (phase, phase_time) = if time_in_stage < warm_up.num_seconds() {
        ("Разминка", time_in_stage)
    } else {
        ("Игра", time_in_stage - warm_up.num_seconds())
    };

    // 6. Форматируем результат
    format!(
        "{}: Этап {}: {}: {:02}:{:02} {}%",
        day_name,
        stage_idx,
        phase,
        phase_time / 60,
        phase_time % 60,
        confidence
    )
}


thread_local! {
    static TESS_INST: RefCell<Tesseract> = RefCell::new(
        Tesseract::new(None, Some("rus+eng")) // Загружаем оба пакета
            .expect("Init failed")
            .set_variable("tessedit_char_whitelist", 
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ")
            .expect("Whitelist error")
            .set_variable("debug_file", "/dev/null").unwrap()
    );
}


pub fn extract_single_nick(img: &image::GrayImage) -> Result<String> {
    let (w, h) = img.dimensions();
    
    TESS_INST.with(|tes_cell| {
        let mut tes = tes_cell.borrow_mut();
        
        // Загружаем полоску (Luma8 = 1 байт на пиксель)
        // В новых версиях переприсваиваем: *tes = tes.set_frame(...)
        let mut owned_tes = std::mem::replace(&mut *tes, Tesseract::new(None, None)?);
        
        owned_tes = owned_tes.set_frame(
            img.as_raw(), 
            w as i32, 
            h as i32, 
            1,       // 1 байт на пиксель (Luma)
            w as i32 // Шаг строки равен ширине
        )?;

        let text = owned_tes.get_text()?;
        
        // Возвращаем объект обратно в ячейку
        *tes = owned_tes;

        // Чистим результат: берем только первую строку и убираем мусор
        Ok(text.lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string())
    })
}