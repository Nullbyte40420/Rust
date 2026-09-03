//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
];

const GIFTS: [&str; 12] = [
    "a Partridge in a Pear Tree.",
    "two Turtle Doves,",
    "three French Hens,",
    "four Calling Birds,",
    "five Gold Rings,",
    "six Geese-a-Laying,",
    "seven Swans-a-Swimming,",
    "eight Maids-a-Milking,",
    "nine Ladies Dancing,",
    "ten Lords-a-Leaping,",
    "eleven Pipers Piping,",
    "twelve Drummers Drumming,"
];

fn main() {
    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", DAYS[i]);
        for j in (0..=i).rev() { // this loop will iterate over the range of numbers from i to 0
            if i > 0 && j == 0 {
                print!("and ");
            }
            println!("{}", GIFTS[j]);
        }
        println!();
    }
}