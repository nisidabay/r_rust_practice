// ex01_http_status.rs — Match HTTP status codes to descriptions
// WHY: HTTP status codes are a perfect case for range + literal matching.
// Match each code group to its standard description.

fn describe_status(code: u16) -> &'static str {
    // TODO: Match the status code using ranges and literals.
    //   100..=199 => "Informational"
    //   200..=299 => "Success"
    //   300..=399 => "Redirection"
    //   400..=499 => "Client Error"
    //   500..=599 => "Server Error"
    //   Also match specific well-known codes:
    //     200 => "OK"
    //     201 => "Created"
    //     204 => "No Content"
    //     301 => "Moved Permanently"
    //     304 => "Not Modified"
    //     400 => "Bad Request"
    //     401 => "Unauthorized"
    //     403 => "Forbidden"
    //     404 => "Not Found"
    //     500 => "Internal Server Error"
    //     502 => "Bad Gateway"
    //     503 => "Service Unavailable"
    //   _ => "Unknown Code"
    //
    // Hint: order matters — specific codes before range arms.
    // Hint: use | to combine multiple specific codes into one arm.
    todo!("implement describe_status")
}

fn main() {
    let codes = [200, 201, 204, 301, 304, 400, 401, 403, 404, 500, 502, 503, 99, 600];

    println!("=== HTTP Status Code Descriptions ===");
    for &code in &codes {
        println!("{code:>3} => {}", describe_status(code));
    }
}
