// 04_buffered_io.rs — BufReader, BufWriter, BufRead
// Buffered I/O minimizes system calls by reading/writing in batches.
// Uses: std::io::{BufReader, BufWriter, BufRead}

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    demo_bufreader()?;
    demo_bufwriter()?;
    demo_bufread_traits()?;
    println!("All buffered I/O demos done.");
    Ok(())
}

fn demo_bufreader() -> io::Result<()> {
    // Create a test file with some content.
    let path = "/tmp/_demo_buf.txt";
    std::fs::write(path, "apple\nbanana\ncherry\ndate\nelderberry\nfig\n")?;

    // BufReader wraps a File (or any Read impl) with an internal buffer.
    // Default buffer size is 8 KB.

    // --- read_line: reads a single line ---
    // Reading line by line with BufReader avoids allocating for the whole file.
    println!("=== read_line (first 3 lines) ===");
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    for _ in 0..3 {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        print!("  read {} bytes: {}", n, line);
    }

    // --- lines() iterator ---
    // The most ergonomic way to read lines (skips the allocation dance).
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    println!("=== lines() iterator ===");
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("  line {}: {}", i + 1, line);
    }

    std::fs::remove_file(path)?;
    Ok(())
}

fn demo_bufwriter() -> io::Result<()> {
    let path = "/tmp/_demo_bufw.txt";

    // BufWriter reduces syscalls by buffering writes.
    // Data is flushed to disk when the buffer fills or the writer drops.
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Buffered line 1")?;
    writeln!(writer, "Buffered line 2")?;
    writeln!(writer, "Buffered line 3")?;

    // Explicit flush: forces buffered data to the underlying writer.
    // Auto-flush happens on Drop, but explicit flush catches IO errors early.
    writer.flush()?;
    println!("Wrote 3 lines via BufWriter, flushed.");

    // Check contents
    let contents = std::fs::read_to_string(path)?;
    println!("File contents:\n{}", contents);

    std::fs::remove_file(path)?;
    Ok(())
}

fn demo_bufread_traits() -> io::Result<()> {
    // BufRead (trait) extends Read with:
    // - read_line
    // - lines()
    // - read_until / split (delimiter-based reading)
    // - fill_buf / consume (direct buffer access)

    let data = b"alpha,beta,gamma,delta\n";
    let mut reader = BufReader::new(&data[..]);

    // read_until: reads until delimiter byte
    let mut buf = Vec::new();
    let n = reader.read_until(b',', &mut buf)?;
    println!("read_until comma: {:?} ({} bytes)", String::from_utf8_lossy(&buf), n);

    // split: iterator over delimiter-separated chunks
    // Need to re-create because BufReader consumed some
    let reader = BufReader::new(&data[..]);
    println!("=== split on comma ===");
    for segment in reader.split(b',') {
        let segment = segment?;
        println!("  chunk: {:?}", String::from_utf8_lossy(&segment));
    }

    Ok(())
}
