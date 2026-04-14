use std::io::Write;
use anyhow::Result;
use crate::core::types::AppContext;

#[macro_export]
macro_rules! menu_tree {
    // 1. Вспомогательный макрос для вставки (без изменений)
    (@insert $list:ident, $parent:expr, $display_name:expr, $func:expr) => {{
        let id = $list.len();
        let mut name = String::new();
        if let Some(p_idx) = $parent {
            let p: usize = p_idx;
            $list[p].text.push_str(&$display_name);
            $list[p].children.push(id);
        } else {
            name = $display_name;
        }
        $list.push(Node {
            children: Vec::new(),
            parent: $parent,
            text: name,
            function: $func,
        });
        id
    }};

    // 2. Обработка ПОДМЕНЮ (в фигурных скобках)
    // Проверяем ПЕРВЫМ, чтобы скобки не съелись как выражение
    (@branch $list:ident, $p:expr, $txt:expr, { $($sub:tt)* }) => {{
        let id = menu_tree!(@insert $list, $p, $txt, None);
        menu_tree!($list, Some(id), { $($sub)* });
    }};

    // 3. Обработка ФУНКЦИИ или ЗАМЫКАНИЯ (выражение)
    // :expr идеально захватывает |ctx: &mut AppContext| edit_draft(ctx)
    (@branch $list:ident, $p:expr, $txt:expr, $func:expr) => {
        menu_tree!(@insert $list, $p, $txt, Some(Box::new(move |ctx| ($func)(ctx))));
    };

    // 4. ГЛАВНЫЙ ЦИКЛ
    ($list:ident, $parent:expr, { $($name:expr => $action:tt),* $(,)? }) => {{
        let mut _num = 0;
        $(
            _num += 1;
            let formatted = format!("{}) {}\n", _num, $name);
            // Передаем как tt, чтобы @branch сам выбрал: блок {} или выражение
            menu_tree!(@branch $list, $parent, formatted, $action);
        )*

        // Отрисовка выхода (без изменений)
        if let Some(p_idx) = $parent {
            let p: usize = p_idx;
            $list[p].text.push_str("_) Выход\nВвод: ");
        } else if !$list.is_empty() {
            $list[0].text.push_str("_) Выход\nВвод: ");
        }
    }};
}



pub struct Node {
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub text: String,
    pub function: Option<Box<dyn Fn(&mut AppContext)>>
}


pub fn process_menu(ctx: &mut AppContext, list: &Vec<Node>) -> Result<()> {
    let mut current_node = &list[0];
    loop {
        // 1. Если в узле есть функция — выполняем её и СРАЗУ откатываемся к родителю
        if let Some(func) = current_node.function.as_ref() {
            func(ctx);
            // После выполнения функции возвращаемся к родителю
            if let Some(parent_idx) = current_node.parent {
                current_node = &list[parent_idx];
            } else {
                break; // Если функции в корне (что вряд ли), выходим
            }
            // Важно: продолжаем цикл, чтобы отрисовать меню родителя
            continue; 
        }

        // 2. Если функции нет — значит это узел-меню, печатаем текст
        print!("{}", current_node.text);
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input_trimmed = input.trim();

        // Если ввод пустой или не число — считаем как "Выход" (на уровень вверх)
        let number: usize = input_trimmed.parse().unwrap_or(0);

        if number >= 1 && number <= current_node.children.len() {
            // Переходим к выбранному ребенку
            current_node = &list[current_node.children[number - 1]];
        } else {
            // Режим "Выход" (_)
            match current_node.parent {
                Some(idx) => current_node = &list[idx],
                None => break, // Мы в корне и нажали "выход" — закрываем программу
            }
        }
    }
    Ok(())
}