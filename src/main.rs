fn main() {
    // On the n'th day of Christmas, my true love sent to me
    // Twelve drummers drumming
    // Eleven pipers piping
    // Ten lords a-leaping
    // Nine ladies dancing
    // Eight maids a-milking
    // Seven swans a-swimming
    // Six geese a-laying
    // Five golden rings
    // Four calling birds
    // Three french hens
    // Two turtle doves and
    // A partridge in a pear tree
    let lyrics_lines: [&str; 14] = ["On the ", "day of Christmas, my true love sent to me ",
                                    "A partridge in a pear tree ", "Two turtle doves and ",
                                    "Three french hens ", "Four calling birds ",
                                    "Five golden rings", "Six geese a-laying",
                                    "Seven swans a-swimming ", "Eight maids a-milking ",
                                    "Nine ladies dancing ", "Ten lords a-leaping ",
                                    "Eleven pipers piping", "Twelve drummers drumming  "];
    let days: [&str; 12] = ["first ", "second ", "third ", "fourth ",
                            "fifth ", "sixth ", "seventh ", "eighth ",
                            "ninth ", "tenth ", "eleventh ", "twelfth "];
    let mut verse: String = String::new();

    for i in 0..12 {
        // let verse = lyrics_lines[0] + days[i] + lyrics_lines[1] + "\n";
        verse.push_str(lyrics_lines[0]);
        verse.push_str(days[i]);
        verse.push_str(lyrics_lines[1]);
        verse.push_str("\n");
        for n in 0..i+1{
            // let verse = verse + lyrics_lines[2 + i - n] + "\n";
            verse.push_str(lyrics_lines[2 + i - n]);
            verse.push_str("\n")
        }
        // let carol = carol + verse + "\n\n";
        verse.push_str("\n\n");
    }

    println!("{verse}");
}
