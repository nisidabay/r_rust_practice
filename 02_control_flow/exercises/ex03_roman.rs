// Print Roman numerals for numbers 1 to 100.
// We decompose each number into tens and units digits,
// then look up the corresponding Roman numeral symbols.

fn main() {
    // Roman numeral lookup tables indexed by digit value (0-9).
    let units = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];

    // For numbers up to 100, we only need tens and units (100 is "C").
    for n in 1..=100 {
        let numeral = if n == 100 {
            // 100 is a special case: just "C" (one hundred).
            "C".to_string()
        } else {
            // Decompose into tens and units digits.
            // 47 → ten=4, one=7 → "XL" + "VII" = "XLVII"
            let ten = n / 10;
            let one = n % 10;
            format!("{}{}", tens[ten as usize], units[one as usize])
        };

        println!("{:3} = {}", n, numeral);
    }
}
