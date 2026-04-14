pub fn print_grid(items: &[String], column_width: usize, columns_count: usize) {
    let total_len = items.len();
    if total_len == 0 { return; }

    let cols = if total_len < 10 { 
        1 
    } else { 
        columns_count
    };

    // 2. Расчет строк
    let rows = (total_len + cols - 1) / cols;

    for r in 0..rows {
        let mut line = String::new();
        
        for c in 0..cols {
            let idx = r + (c * rows);
            
            if idx < total_len {
                let entry = format!("{:>2}) {}", idx + 1, items[idx]);
                // Добавляем отступ (padding) между колонками для красоты
                line.push_str(&format!("{:<width$}", entry, width = column_width));
            }
        }
        println!("{}", line);
    }
}