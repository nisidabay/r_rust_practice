// ex01_http_status.rs — Define enum HttpStatus, match on HTTP status codes
// The enum carries the numeric code; match prints a human-readable description

enum HttpStatus {
    Ok,       // 200
    NotFound, // 404
    Error,    // 500
}

impl HttpStatus {
    fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::Error => 500,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "Request succeeded — resource found",
            HttpStatus::NotFound => "Resource not found on server",
            HttpStatus::Error => "Internal server error",
        }
    }
}

fn main() {
    let statuses = [HttpStatus::Ok, HttpStatus::NotFound, HttpStatus::Error];
    for s in &statuses {
        println!("{} {} — {}", s.code(), s.description(), s.code());
        // Output: "200 Request succeeded — resource found"
    }
}
