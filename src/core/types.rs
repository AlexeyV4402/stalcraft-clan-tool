use std::fmt;
use crate::config::MAX_MEMBERS;


#[derive(Default)]
pub struct AppContext {
    pub current_idx: usize,
    pub data: ActiveData
}

#[derive(Default)]
pub enum ActiveData {
    #[default]
    None,
    Drafts(Vec<CWDraft>),
    Members(Vec<Member>),
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Member {
    pub is_active: bool,
    pub nick: String,        // UTF-8 фиксированной длины
    pub discord: String,     // UTF-8 фиксированной длины
}

#[repr(C)]
pub struct CWDraft {
    pub timestamp: i64,
    pub reserve: u64,
    pub top15: u64,
    pub supply: u64,
    
    pub attendance: u64,
    pub name: [u8; 64],
    pub file_name: [u8; 32]
}


#[cfg(debug_assertions)]
impl std::fmt::Debug for CWDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KVDraft")
            .field("timestamp", &self.timestamp)
            .field("reserve", &format_args!("{:0width$b}", &self.reserve, width=MAX_MEMBERS))
            .field("top15", &format_args!("{:0width$b}", &self.top15, width=MAX_MEMBERS))
            .field("supply", &format_args!("{:0width$b}", &self.supply, width=MAX_MEMBERS))

            .field("attendance", &format_args!("{:0width$b}", &self.attendance, width=MAX_MEMBERS))
            .field("name", &(String::from_utf8_lossy(&self.name).trim_end_matches('\0')))
            .field("file_name", &(String::from_utf8_lossy(&self.file_name).trim_end_matches('\0')))
            
            .finish()
    }
}


impl std::fmt::Display for CWDraft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_str = String::from_utf8_lossy(&self.name);
        let file_name_str = String::from_utf8_lossy(&self.file_name);
        write!(
            f,
            "Черновик: {}\n\
            Таймштамп: {}\n\
            Люди в запасе: {:0width$b}\n\
            Топ 15: {:0width$b}\n\
            Получившие расходники: {:0width$b}\n\
            Посещаемость: {:0width$b}\n\
            Имя файла: {}\n",
            name_str.trim_matches('\0'),
            self.timestamp,
            self.reserve,
            self.top15,
            self.supply,
            self.attendance,
            file_name_str.trim_matches('\0'),
            width = MAX_MEMBERS
        )
    }
}




#[repr(C)]
pub struct CWRecord {
    pub timestamp: i64,
    pub reserve: u64,
    pub top15: u64,
    pub supply: u64
}


#[cfg(debug_assertions)]
impl std::fmt::Debug for CWRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KVRecord")
            .field("timestamp", &self.timestamp)
            .field("reserve", &format_args!("{:#064b}", &self.reserve))
            .field("top15", &format_args!("{:#064b}", &self.top15))
            .field("supply", &format_args!("{:?}", &self.supply))
            .finish()
    }
}


pub enum CWType {
    BRAWL,
    TOURNAMENT,
    BASECAPTURING
}