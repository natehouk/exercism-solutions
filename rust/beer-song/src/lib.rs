pub fn verse(n: u32) -> String {
    if n > 2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1)
    } else if n == 2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1)
    } else if n == 1 {
        format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n)
    } else if n == 0 {
        format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else {
        unimplemented!("error")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = String::new();
    for i in ((end)..(start + 1)).rev() {
        song.push_str(&verse(i));
        if i != end {
            song.push_str("\n");
        }
    }
    song
}
