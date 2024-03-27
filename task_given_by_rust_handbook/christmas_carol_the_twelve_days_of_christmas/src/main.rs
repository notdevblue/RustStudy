fn main() {
    let lyrics = [
        "partridge in a pear tree",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    let number_to_string = [
        "first",
        "second",
        "third",
        "forth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for idx in 0..number_to_string.len() {
        println!("On the {} day of Christmas,",  number_to_string[idx]);
        for idx_inner_lyrics in (1..(idx + 1)).rev() {
            println!("{}", lyrics[idx_inner_lyrics]);
        }

        if idx == 0 {
            println!("A {}.", lyrics[0]);
        } else {
            print!("And {}", lyrics[0]);

            println!("{}", if idx == number_to_string.len() - 1 { "!" } else { "." });
        }

        println!("");
    }
}
