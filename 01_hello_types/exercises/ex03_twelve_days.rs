// ex03_twelve_days — Print "The Twelve Days of Christmas" lyrics
// Uses: arrays for lookup tables, ranges for iteration, string formatting.
// Each verse adds a new gift and repeats earlier gifts, building
// cumulative output. Demonstrates how arrays and ranges work together.

fn main() {
    // Parallel arrays: day names, gifts, and ordinal suffixes.
    // Using &str (string slices) — immutable, fixed at compile time.
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    // For each day (0..12), print the verse.
    for day_idx in 0..days.len() {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            days[day_idx]
        );

        // Print gifts from current day down to day 0 (inclusive).
        // Range is (0..=day_idx).rev() — inclusive range reversed.
        for gift_idx in (0..=day_idx).rev() {
            // Special case: day 0 is "a Partridge..."
            // All other days: prepend "and" for day 0's gift.
            if day_idx > 0 && gift_idx == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift_idx]);
        }
        println!(); // blank line between verses
    }
}
