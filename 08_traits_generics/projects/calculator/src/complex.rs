// src/complex.rs — Complex number type for the generic calculator.

use std::fmt;
use std::str::FromStr;

/// A complex number with real and imaginary parts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    /// Create a new complex number.
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else if self.real == 0.0 {
            if self.imag == 1.0 {
                write!(f, "i")
            } else if self.imag == -1.0 {
                write!(f, "-i")
            } else {
                write!(f, "{}i", self.imag)
            }
        } else if self.imag > 0.0 {
            if self.imag == 1.0 {
                write!(f, "{}+i", self.real)
            } else {
                write!(f, "{}+{}i", self.real, self.imag)
            }
        } else {
            // imag < 0
            if self.imag == -1.0 {
                write!(f, "{}-i", self.real)
            } else {
                write!(f, "{}{}i", self.real, self.imag)
            }
        }
    }
}

impl FromStr for Complex {
    type Err = String;

    /// Parse a complex number from a string.
    ///
    /// Accepted formats:
    ///   "3+4i", "3-4i", "4i", "3", "-i", "i"
    ///   "+3+4i", "-3-4i", etc.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err("empty string".into());
        }

        // Handle "i" or "-i" or "+i"
        if s == "i" {
            return Ok(Complex::new(0.0, 1.0));
        }
        if s == "-i" {
            return Ok(Complex::new(0.0, -1.0));
        }
        if s == "+i" {
            return Ok(Complex::new(0.0, 1.0));
        }

        // If there's no 'i', it's a real number.
        if !s.contains('i') {
            let real: f64 = s
                .parse()
                .map_err(|_| format!("cannot parse '{s}' as a number"))?;
            return Ok(Complex::new(real, 0.0));
        }

        // It contains 'i'. Split into real and imaginary parts.
        let s = s.trim_end_matches('i');

        // If after removing 'i' the string is empty, it was just "i" (handled above)
        if s.is_empty() {
            return Ok(Complex::new(0.0, 1.0));
        }

        // Find the position of the sign between real and imaginary parts.
        // Look for '+' or '-' after position 0 (since the first char could be a sign).
        let sign_pos = s[1..]
            .find(|c: char| c == '+' || c == '-')
            .map(|pos| pos + 1);

        match sign_pos {
            None => {
                // No sign after position 0 means it's purely imaginary: e.g., "5i", "-3i"
                let imag: f64 = s
                    .parse()
                    .map_err(|_| format!("cannot parse imaginary part '{s}'"))?;
                Ok(Complex::new(0.0, imag))
            }
            Some(pos) => {
                let real_str = &s[..pos];
                let imag_str = &s[pos..];
                let real: f64 = real_str
                    .parse()
                    .map_err(|_| format!("cannot parse real part '{real_str}'"))?;
                let imag: f64 = if imag_str == "+" {
                    1.0
                } else if imag_str == "-" {
                    -1.0
                } else {
                    imag_str
                        .parse()
                        .map_err(|_| format!("cannot parse imaginary part '{imag_str}'"))?
                };
                Ok(Complex::new(real, imag))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_real() {
        assert_eq!("3.14".parse::<Complex>().unwrap(), Complex::new(3.14, 0.0));
        assert_eq!("-5".parse::<Complex>().unwrap(), Complex::new(-5.0, 0.0));
        assert_eq!("0".parse::<Complex>().unwrap(), Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_parse_pure_imag() {
        assert_eq!("4i".parse::<Complex>().unwrap(), Complex::new(0.0, 4.0));
        assert_eq!("-3i".parse::<Complex>().unwrap(), Complex::new(0.0, -3.0));
        assert_eq!("i".parse::<Complex>().unwrap(), Complex::new(0.0, 1.0));
        assert_eq!("-i".parse::<Complex>().unwrap(), Complex::new(0.0, -1.0));
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            "3+4i".parse::<Complex>().unwrap(),
            Complex::new(3.0, 4.0)
        );
        assert_eq!(
            "3-4i".parse::<Complex>().unwrap(),
            Complex::new(3.0, -4.0)
        );
        assert_eq!(
            "-1+2i".parse::<Complex>().unwrap(),
            Complex::new(-1.0, 2.0)
        );
        assert_eq!(
            "-1-2i".parse::<Complex>().unwrap(),
            Complex::new(-1.0, -2.0)
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(Complex::new(3.0, 4.0).to_string(), "3+4i");
        assert_eq!(Complex::new(3.0, -4.0).to_string(), "3-4i");
        assert_eq!(Complex::new(0.0, 5.0).to_string(), "5i");
        assert_eq!(Complex::new(5.0, 0.0).to_string(), "5");
        assert_eq!(Complex::new(0.0, 1.0).to_string(), "i");
        assert_eq!(Complex::new(0.0, -1.0).to_string(), "-i");
    }
}
