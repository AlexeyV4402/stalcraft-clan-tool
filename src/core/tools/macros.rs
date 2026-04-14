#[macro_export]
macro_rules! handle_cli_error {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => {
                // Используем красный цвет для привлечения внимания
                eprintln!("\n❌ [ОШИБКА]: {:#}", err); 
                println!("Нажмите Enter, чтобы продолжить...");
                
                // Ждем ввода, чтобы юзер успел прочитать
                let mut _unused = String::new();
                let _ = std::io::stdin().read_line(&mut _unused);
                
                return; // Выходим из функции меню
            }
        }
    };
}