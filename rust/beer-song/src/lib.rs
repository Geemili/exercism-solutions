use std::fmt::Write;

pub fn verse(n: u32) -> String {
    let mut s = String::new();
    verse_raw(&mut s, n).expect("no failure to write to string");
    s
}

pub fn verse_raw<W: Write>(s: &mut W, n: u32) -> Result<(), std::fmt::Error> {
    match n {
        0 => write!(s, "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => write!(s, "{n} bottle of beer on the wall, {n} bottle of beer.\nTake it down and pass it around, {n_1} bottles of beer on the wall.\n", n=n, n_1="no more"),
        2 => write!(s, "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {n_1} bottle of beer on the wall.\n", n=n, n_1=1),
        _ => write!(s, "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {n_1} bottles of beer on the wall.\n", n=n, n_1=n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut current_verse = start;
    let mut verses = String::new();
    loop {
        if current_verse != start {
            verses.push('\n');
        }

        verse_raw(&mut verses, current_verse);

        if current_verse == end {
            break;
        }
        current_verse -= 1;
    }
    verses
}
