// Loop labels allow `break` and `continue` to target a specific outer
// loop when loops are nested — by default they only affect the innermost one.

fn main() {
    // --- Break out of a specific outer loop ---
    // We use `'label_name: loop { ... }` to name a loop.
    // Then `break 'label_name` exits that specific loop.
    let mut outer_count = 0;
    // Label this outer loop as `'outer`.
    'outer: loop {
        outer_count += 1;
        let mut inner_count = 0;

        loop {
            inner_count += 1;
            println!("outer={outer_count}, inner={inner_count}");

            if inner_count >= 3 {
                // Break the outer loop, not just the inner one.
                break 'outer;
            }
            if outer_count >= 2 && inner_count >= 2 {
                // Break the inner loop only (default behavior).
                break;
            }
        }
    }
    println!("Exited outer loop at outer_count={outer_count}");

    // --- Loop labels work with while too ---
    let mut x = 0;
    'outer_while: while x < 3 {
        let mut y = 0;
        while y < 3 {
            y += 1;
            if x == 1 && y == 2 {
                // Break the outer while loop.
                break 'outer_while;
            }
            println!("while: x={x}, y={y}");
        }
        x += 1;
    }
    // Note: `y` is not accessible here because it was declared inside the
    // inner while loop's scope (each outer iteration creates a new y).
    println!("Exited outer while at x={x}");

    // --- Loop labels work with for too ---
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut target = 0;

    'outer_for: for row in &matrix {
        for cell in row {
            if *cell == 5 {
                target = 5;
                // Break both loops — we found what we were looking for.
                break 'outer_for;
            }
        }
    }
    println!("Found target: {target}");

    // --- continue with labels ---
    // `continue 'label` skips to the next iteration of the labeled loop.
    let mut total = 0;
    'outer_cont: for i in 1..=4 {
        for j in 1..=4 {
            if i == j {
                // Skip to next iteration of the OUTER loop.
                continue 'outer_cont;
            }
            total += i * j;
        }
    }
    // When i==j, we skip the rest of the inner loop AND go to the NEXT i.
    // i=1: j=1 → skip (continue outer), so no products for i=1 at all.
    // i=2: j=1 → 2*1=2, j=2 → skip to i=3
    // i=3: j=1 → 3*1=3, j=2 → 3*2=6, j=3 → skip to i=4
    // i=4: j=1 → 4*1=4, j=2 → 4*2=8, j=3 → 4*3=12, j=4 → skip (end)
    // total = 2 + 3 + 6 + 4 + 8 + 12 = 35
    println!("Labeled continue total: {total}");

    // --- Loop labels with break returning a value ---
    // Labels work with break-value too.
    let found_val = 'search: loop {
        for n in 1..100 {
            if n % 7 == 0 && n % 11 == 0 {
                // Break outer with a value (n is the LCM of 7 and 11).
                break 'search n;
            }
        }
        // Safety: without this, the outer loop would be infinite
        // if the inner loop never finds a match.
        break 'search 0;
    };
    println!("Found number divisible by 7 and 11: {found_val}");
}
