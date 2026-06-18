// main.rs — macro_madness CLI tool
//
// A demo CLI that defines and exercises custom macros.
// Run with:
//   cargo run           — show standard demo
//   cargo run -- --dump — show macro expansion examples (textual)

#[macro_use]
mod macros;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dump_mode = args.len() > 1 && args[1] == "--dump";

    if dump_mode {
        run_dump_mode();
    } else {
        run_demo_mode();
    }
}

fn run_demo_mode() {
    println!("╔══════════════════════════════════════╗");
    println!("║       macro_madness — CLI Demo       ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // Declarative macro basics
    println!("── Declarative Macros ──────────────────");
    say_hello!("Rustacean");
    say_hello!("macro_madness");
    println!();

    // vec!-like macro
    println!("── Vec Builder ─────────────────────────");
    let v = build_vec![1, 2, 3, 4, 5];
    println!("build_vec! result: {:?}", v);

    let repeated = build_vec![0; 8];
    println!("build_vec![0; 8]: {:?}", repeated);
    println!();

    // hashmap macro
    println!("── HashMap Builder ─────────────────────");
    let map = make_map! {
        "name" => "macro_madness",
        "version" => "0.1.0",
        "lang" => "Rust",
    };
    for (k, v) in &map {
        println!("  {} = {}", k, v);
    }
    println!();

    // Colored output
    println!("── Colored Output ──────────────────────");
    cprintln_red!("This is an error message");
    cprintln_green!("This is a success message");
    cprintln_yellow!("This is a warning message");
    cprintln_blue!("This is informational");
    println!();

    // Test harness
    println!("── Test Harness ────────────────────────");
    run_tests! {
        "math" => {
            assert_eq!(2 + 2, 4);
        },
        "strings" => {
            assert_eq!("hello".len(), 5);
        },
        "booleans" => {
            assert!(true);
        },
    }
    println!();

    // Recursive macro example
    println!("── Recursive Macros ────────────────────");
    let t = nested_tuple!(1, 2, 3, 4);
    println!("nested_tuple!(1, 2, 3, 4) = {:?}", t);
    println!();

    // Repeat pattern
    println!("── Repeat Builder ──────────────────────");
    let items = repeat_items!("Rust", 5, ", ");
    println!("repeat_items! result: {}", items);
    println!();

    // Usage
    println!("── Usage ───────────────────────────────");
    println!("  cargo run               — this demo");
    println!("  cargo run -- --dump     — show macro expansions");
}

fn run_dump_mode() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║  macro_madness — Macro Expansion Dump    ║");
    println!("╚═══════════════════════════════════════════╝");
    println!();

    println!("What macro_rules! expansions look like:");
    println!();
    println!("say_hello!(\"Rustacean\") expands to:");
    println!("  println!(\"Hello, {{}}!\", \"Rustacean\");");
    println!();
    println!("build_vec![1, 2, 3] expands approximately to:");
    println!("  {{");
    println!("      let mut v = Vec::new();");
    println!("      v.push(1);");
    println!("      v.push(2);");
    println!("      v.push(3);");
    println!("      v");
    println!("  }}");
    println!();
    println!("make_map! expands approximately to:");
    println!("  {{");
    println!("      let mut map = std::collections::HashMap::new();");
    println!("      map.insert(\"key\", \"value\");");
    println!("      map");
    println!("  }}");
    println!();
    println!("nested_tuple!(1, 2, 3, 4) expands to:");
    println!("  (1, (2, (3, 4)))");
    println!();
    println!("This is a TEXTUAL expansion dump.");
    println!("For actual macro expansion, use:");
    println!("  cargo rustc -- -Z unstable-options --pretty expanded");
}
