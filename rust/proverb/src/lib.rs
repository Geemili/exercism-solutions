use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        return proverb;
    }
    let mut i = 1;
    while i < list.len() {
        write!(proverb, "For want of a {} the {} was lost.\n", list[i-1], list[i]).unwrap();
        i += 1;
    }
    write!(proverb, "And all for the want of a {}.", list[0]).unwrap();
    proverb
}
