// 07_env_args.rs — Command-line args, environment variables, current dir
// Uses: std::env::{args, var, vars, current_dir, set_current_dir}

use std::env;
use std::path::Path;

fn main() {
    // --- Command-line arguments ---
    // args() returns an iterator over the arguments.
    // The first element (index 0) is the program name.
    println!("=== Command-line arguments ===");
    let args: Vec<String> = env::args().collect();
    println!("Program: {}", args.get(0).unwrap_or(&"<unknown>".into()));
    println!("Total args: {}", args.len() - 1);
    for (i, arg) in args.iter().skip(1).enumerate() {
        println!("  arg[{}]: {}", i + 1, arg);
    }

    // Alternative: skip the program name
    println!("\nargs_os (raw OsString):");
    for arg in env::args_os().skip(1) {
        println!("  {:?}", arg);
    }

    // --- Environment variables ---
    println!("\n=== Environment variables ===");

    // var() returns Result<String, VarError>
    match env::var("HOME") {
        Ok(val) => println!("HOME = {}", val),
        Err(env::VarError::NotPresent) => println!("HOME not set"),
        Err(env::VarError::NotUnicode(_)) => println!("HOME contains invalid UTF-8"),
    }

    match env::var("PATH") {
        Ok(val) => {
            let count = val.split(':').count();
            println!("PATH has {} entries", count);
        }
        Err(_) => println!("PATH not set"),
    }

    // vars() returns all environment variables as (String, String) pairs
    println!("\nAll env vars (first 5 shown):");
    for (_i, (key, value)) in env::vars().take(5).enumerate() {
        println!("  {}={}", key, value);
    }

    // Setting environment variables (affects only this process and children)
    env::set_var("MY_APP_VAR", "hello_from_rust");
    match env::var("MY_APP_VAR") {
        Ok(val) => println!("\nMY_APP_VAR = {}", val),
        Err(_) => println!("\nMY_APP_VAR not set"),
    }

    // Removing
    env::remove_var("MY_APP_VAR");
    println!("MY_APP_VAR after remove: {:?}", env::var("MY_APP_VAR"));

    // --- Current directory ---
    println!("\n=== Current directory ===");

    match env::current_dir() {
        Ok(cwd) => println!("current_dir: {:?}", cwd),
        Err(e) => eprintln!("current_dir error: {}", e),
    }

    // Change current directory (chdir) — affects subsequent operations
    if let Err(e) = env::set_current_dir(Path::new("/tmp")) {
        eprintln!("set_current_dir error: {}", e);
    } else {
        println!("Changed cwd to /tmp");
        match env::current_dir() {
            Ok(cwd) => println!("new current_dir: {:?}", cwd),
            Err(e) => eprintln!("error: {}", e),
        }
    }

    // --- temp_dir (platform-specific temp directory) ---
    println!("\ntemp_dir: {:?}", env::temp_dir());

    // --- current_exe (path to this binary) ---
    match env::current_exe() {
        Ok(exe) => println!("current_exe: {:?}", exe),
        Err(e) => eprintln!("current_exe error: {}", e),
    }

    // --- consts (compile-time values) ---
    println!("\n=== Compile-time constants ===");
    println!("ARCH: {}", env::consts::ARCH);
    println!("OS: {}", env::consts::OS);
    println!("FAMILY: {}", env::consts::FAMILY);
    println!("EXE_SUFFIX: {}", env::consts::EXE_SUFFIX);
    println!("EXE_EXTENSION: {:?}", env::consts::EXE_EXTENSION);
}
