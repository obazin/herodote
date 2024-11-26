use chrono::DateTime;
use std::cmp;

/// Transforms and truncates a given input string to create a valid filename.
///
/// The `normalized_filename_string` function processes an input string to:
/// - Filter out non-alphanumeric and non-whitespace characters.
/// - Convert whitespace to underscores.
/// - Truncate the string to a specified maximum length, prioritizing complete words where possible.
///
/// # Parameters
/// - `input: &str`: The input string to normalize and truncate. This can include mixed characters, with the expectation
///   that non-alphanumeric and non-whitespace characters will be removed.
/// - `max_length: usize`: The maximum permissible length for the output string. The function ensures that the returned
///   filename string does not exceed this length.
///
/// # Returns
/// - `String`: A string containing only alphanumeric characters and underscores, truncated as necessary to comply
///   with the `max_length` constraint.
pub fn normalized_filename_string(input: &str, max_length: usize) -> String {
    // Remove invalid characters and normalize to ASCII
    let normalized: String = input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .map(|c| if c.is_whitespace() { '_' } else { c })
        .collect();

    // If the string is already within the max length, return as is
    if normalized.len() <= max_length {
        return normalized;
    }

    // Truncate, prioritizing full words
    let mut result = String::new();
    let mut current_length = 0;
    for word in normalized.split('_') {
        let word_length = word.len();
        // Check if adding this word would exceed the max length
        if current_length + word_length + if current_length > 0 { 1 } else { 0 } > max_length {
            // If truncation must occur mid-word, add an underscore and stop
            if current_length == 0 {
                result.push_str(&word[..cmp::min(word_length, max_length - 1)]);
                result.push('_');
            }
            break;
        }
        // Add the word to the result
        if !result.is_empty() {
            result.push('_');
        }
        result.push_str(word);
        current_length += word_length + 1; // Include underscore
    }
    result
}

/// Converts a given epoch time (in seconds as a floating-point number) to a formatted date string
///
/// # Arguments
///
/// * `epoch_time` - A `f64` representing the epoch time. This value includes both whole seconds
///   and fractional seconds, where the fractional part represents nanoseconds.
///
/// # Returns
///
/// * A `String` representing the date in the format `YYYY-MM-DD`. If the conversion from the
///   timestamp fails, an empty string is returned.
///
/// # Example
///
/// ```rust
/// use crate::utils::date_from_epoch_time;
///
/// let epoch_time = 1638316800.0; // This represents December 1, 2021
/// let date = date_from_epoch_time(epoch_time);
/// assert_eq!(date, "2021-12-01");
/// ```
///
pub fn date_from_epoch_time(epoch_time: f64) -> String {
    // Separate into seconds and nanoseconds
    let seconds = epoch_time as i64;
    let nanoseconds = ((epoch_time - seconds as f64) * 1_000_000_000.0) as u32;

    let Some(datetime) = DateTime::from_timestamp(seconds, nanoseconds) else {
        return String::new();
    };
    datetime.format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::{date_from_epoch_time, normalized_filename_string};

    #[test]
    fn test_normalized_filename_string() {
        assert_eq!(
            normalized_filename_string("This is a Test!", 20),
            "This_is_a_Test"
        );
        assert_eq!(
            normalized_filename_string("Special @#$%^&*() Characters", 25),
            "Special__Characters"
        );
        assert_eq!(
            normalized_filename_string("Word Boundaries Work Well", 10),
            "Word"
        );
        assert_eq!(normalized_filename_string("SingleWord", 50), "SingleWord");
    }

    #[test]
    fn test_date_from_epoch_time() {
        assert_eq!(
            date_from_epoch_time(1672531200.0), // Jan 1, 2023, UTC
            "2023-01-01"
        );
        assert_eq!(
            date_from_epoch_time(0.0), // Unix epoch start
            "1970-01-01"
        );
        assert_eq!(
            date_from_epoch_time(-1.0), // Negative time (pre-epoch)
            "1969-12-31"
        );
    }
}
