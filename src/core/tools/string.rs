pub fn normalize_nick(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| match c {
            'а' => 'a', 'р' => 'p', 'е' => 'e', 'т' => 't', 'о' => 'o', 
            'с' => 'c', 'х' => 'x', 'у' => 'y', 'н' => 'n', 'м' => 'm',
            '0' => 'o', '5' => 's',
            _ => c,
        })
        .collect()
}