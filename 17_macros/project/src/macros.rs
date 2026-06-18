// macros.rs — Macro definitions for the macro_madness CLI tool

/// A basic hello macro.
#[macro_export]
macro_rules! say_hello {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

/// Build a Vec from a list or repeat pattern.
#[macro_export]
macro_rules! build_vec {
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($x);
        )+
        v
    }};
    ($x:expr; $count:expr) => {{
        let count = $count;
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push($x);
        }
        v
    }};
    () => {
        Vec::<i32>::new()
    };
}

/// Build a HashMap from key => value pairs.
#[macro_export]
macro_rules! make_map {
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }};
    () => {
        ::std::collections::HashMap::new()
    };
}

/// Colored print macros using ANSI escapes.
#[macro_export]
macro_rules! cprintln_red {
    ($($arg:tt)*) => {
        println!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! cprintln_green {
    ($($arg:tt)*) => {
        println!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! cprintln_yellow {
    ($($arg:tt)*) => {
        println!("\x1b[33m{}\x1b[0m", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! cprintln_blue {
    ($($arg:tt)*) => {
        println!("\x1b[34m{}\x1b[0m", format_args!($($arg)*));
    };
}

/// A small test harness macro.
#[macro_export]
macro_rules! run_tests {
    ($($name:expr => $body:block),+ $(,)?) => {{
        let mut passed = 0u32;
        let mut failed = 0u32;
        $(
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                $body
            }));
            match result {
                Ok(()) => {
                    println!("  ✓ {} — passed", $name);
                    passed += 1;
                }
                Err(_) => {
                    println!("  ✗ {} — FAILED (panicked)", $name);
                    failed += 1;
                }
            }
        )+
        if failed == 0 {
            cprintln_green!("  Result: all {} tests passed!", passed);
        } else {
            cprintln_red!("  Result: {} passed, {} failed", passed, failed);
        }
    }};
}

/// Recursive nested tuple builder.
#[macro_export]
macro_rules! nested_tuple {
    ($x:expr) => { $x };
    ($first:expr, $($rest:expr),+) => {
        ($first, nested_tuple!($($rest),+))
    };
}

/// Repeat an expression into a joined string.
#[macro_export]
macro_rules! repeat_items {
    ($item:expr, $count:expr, $sep:expr) => {{
        let mut s = String::new();
        let count = $count;
        for i in 0..count {
            if i > 0 {
                s.push_str($sep);
            }
            s.push_str(&format!("{}", $item));
        }
        s
    }};
}
