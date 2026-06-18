// 07_option_result_combine — map, and_then, or_else, unwrap_or on Option/Result
//
// Option and Result have a rich set of combinator methods that let you
// chain operations without explicit match blocks.

fn main() {
    // ==================== Option combinators ====================

    // --- 1. map — transform inner value if Some ---
    let a: Option<i32> = Some(5);
    let b: Option<i32> = None;

    let mapped_a = a.map(|x| x * 2);
    let mapped_b = b.map(|x| x * 2);

    println!("Some(5).map(|x| x*2) = {:?}", mapped_a);
    println!("None.map(|x| x*2) = {:?}", mapped_b);

    // --- 2. and_then — chain operations that return Option ---
    // Also called "flat_map" — the closure returns Option, not bare value.
    let s: Option<&str> = Some("42");
    let parsed: Option<i32> = s.and_then(|s| s.parse().ok());

    let bad_s: Option<&str> = Some("abc");
    let parsed_bad: Option<i32> = bad_s.and_then(|s| s.parse().ok());

    let none_s: Option<&str> = None;
    let parsed_none: Option<i32> = none_s.and_then(|s| s.parse().ok());

    println!("Some(\"42\").and_then(parse) = {:?}", parsed);
    println!("Some(\"abc\").and_then(parse) = {:?}", parsed_bad);
    println!("None.and_then(parse) = {:?}", parsed_none);

    // --- 3. or / or_else — fallback if None ---
    let x: Option<i32> = None;
    println!("None.or(Some(0)) = {:?}", x.or(Some(0)));
    println!(
        "None.or_else(|| Some(42)) = {:?}",
        x.or_else(|| Some(42))
    );

    let y: Option<i32> = Some(10);
    println!("Some(10).or(Some(0)) = {:?}", y.or(Some(0))); // keeps 10

    // --- 4. unwrap_or / unwrap_or_else — get value or default ---
    let z: Option<i32> = None;
    println!("None.unwrap_or(-1) = {}", z.unwrap_or(-1));
    println!("None.unwrap_or_else(|| -1) = {}", z.unwrap_or_else(|| -1));

    let z2: Option<i32> = Some(99);
    println!("Some(99).unwrap_or(-1) = {}", z2.unwrap_or(-1));

    // ==================== Result combinators ====================

    // --- 5. map on Result — transform Ok value ---
    let ok: Result<i32, &str> = Ok(10);
    let err: Result<i32, &str> = Err("fail");

    println!("Ok(10).map(|x| x*2) = {:?}", ok.map(|x| x * 2));
    println!("Err.map(|x| x*2) = {:?}", err.map(|x| x * 2));

    // --- 6. map_err — transform the Err value ---
    let mapped_ok = ok.map_err(|e| format!("error: {e}"));
    let mapped_err = err.map_err(|e| format!("error: {e}"));
    println!("Ok(10).map_err(...) = {:?}", mapped_ok);
    println!("Err(\"fail\").map_err(...) = {:?}", mapped_err);

    // --- 7. and_then on Result — chain fallible operations ---
    fn parse_and_double(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| format!("{e}"))
            .map(|n| n * 2)
    }

    println!("and_then chain: {:?}", parse_and_double("21"));
    println!("and_then chain: {:?}", parse_and_double("abc"));

    // --- 8. or_else on Result — recover from error ---
    let result2: Result<i32, &str> = Ok(42);
    println!(
        "Ok(42).or_else(|_| Ok(0)) = {:?}",
        result2.or_else(|_| Ok::<_, &str>(0))
    );

    // --- 9. Combining Option and Result ---
    // ok_or: Option -> Result
    let opt: Option<i32> = Some(5);
    let res: Result<i32, &str> = opt.ok_or("was none");
    println!("Some(5).ok_or(...) = {:?}", res);

    let opt: Option<i32> = None;
    let res: Result<i32, &str> = opt.ok_or("was none");
    println!("None.ok_or(...) = {:?}", res);

    // result.ok(): Result -> Option
    let r: Result<i32, &str> = Ok(10);
    let o: Option<i32> = r.ok();
    println!("Ok(10).ok() = {:?}", o);

    let r: Result<i32, &str> = Err("fail");
    let o: Option<i32> = r.ok();
    println!("Err.ok() = {:?}", o);

    println!("--- 07_option_result_combine done ---");
}
