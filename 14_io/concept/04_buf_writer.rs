// 04_buf_writer.rs — BufWriter for efficient writes
//
// Every write!() or write_all() call is a syscall — expensive.
// BufWriter buffers writes in memory and flushes to disk in large chunks.
// Fewer syscalls = faster writes, especially for many small writes.

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let path = "/tmp/14_io_bufwriter_test.txt";

    // WITHOUT BufWriter: each write_all() makes a syscall
    // File::create opens the file, each write triggers a write(2) syscall
    {
        let mut file = File::create("/tmp/no_buf.txt").unwrap();
        for i in 0..10_000 {
            // 10,000 syscalls — slow!
            file.write_all(format!("line {}\n", i).as_bytes()).unwrap();
        }
        // file dropped, closed
    }
    println!("Without BufWriter: slow but it worked");

    // WITH BufWriter: buffers in memory, flushes in ~8KB chunks
    {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);

        for i in 0..10_000 {
            // All writes go to a memory buffer — no syscall until buffer is full
            writeln!(writer, "line {}", i).unwrap();
        }
        // BufWriter::flush() is called automatically on drop,
        // writing any remaining buffered data to disk
        // writer.flush().unwrap();  // explicit flush if needed
    } // auto-flush on drop
    println!("With BufWriter: much faster (fewer syscalls)");

    // Verify the output
    let contents = std::fs::read_to_string(path).unwrap();
    let line_count = contents.lines().count();
    println!("Wrote {} lines to {}", line_count, path);

    // First and last lines
    println!("First: {:?}", contents.lines().next().unwrap());
    println!("Last:  {:?}", contents.lines().last().unwrap());

    // Explicit flush is important when you need to guarantee data is on disk
    // before continuing (e.g., before reading what you just wrote)
    {
        let file = File::create("/tmp/flush_test.txt").unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "This data is buffered").unwrap();

        // Without flush, data might not be on disk yet
        writer.flush().unwrap();  // force write to disk NOW

        // Now it's safe to read the file
        let contents = std::fs::read_to_string("/tmp/flush_test.txt").unwrap();
        println!("\nAfter flush, file contains: {:?}", contents.trim());
    }

    // Cleanup
    // std::fs::remove_file(path).ok();
    // std::fs::remove_file("/tmp/no_buf.txt").ok();
    // std::fs::remove_file("/tmp/flush_test.txt").ok();
}
