fn main() {
    let ordinal_number = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let lyrics = [
        "a partridge in a pear tree.",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..=11 {
        println!("{}.", i + 1);
        println!(
            "On the {} day of Christmas,\nmy true love sent to me",
            ordinal_number[i],
        );

        for (j, val) in lyrics[0..=i].iter().enumerate().rev() {
            if i != 0 && j == 0 {
                println!("And {}", val);
            } else if i != 0 && j == 0 {
                println!("{}", val);
            } else {
                println!("{},", val);
            }
        }
        println!("----------")
    }
}
