#[cfg(debug_assertions)]
pub static DEBUG_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[macro_export]
macro_rules! trace_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)] // Код макроса живет только в дебаге
        {
            if $crate::debug::DEBUG_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
                println!($($arg)*);
            }
        }
        // В релизе макрос превращается в пустоту
    };
}