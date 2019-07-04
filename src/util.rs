pub fn is_chinese(words: &str) -> bool {
    for b in words.chars() {
        if b >= '\u{4E00}' && b <= '\u{9FA5}' {
            return true;
        }
    }
    return false;
}
