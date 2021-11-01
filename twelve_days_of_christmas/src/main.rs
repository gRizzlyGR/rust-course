fn sing() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let verses = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for i in 0..days.len() {
        println!(
            "On the {} day of Christmas,\n\tMy true love sent to me:",
            days[i]
        );

        for j in (0..i + 1).rev() {
            println!("\t\t{}", verses[j]);
        }
    }
}

fn main() {
    sing();
}
