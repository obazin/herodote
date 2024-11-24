use chrono::DateTime;
use std::cmp;

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

pub fn date_from_epoch_time(epoch_time: f64) -> String {
    // Separate into seconds and nanoseconds
    let seconds = epoch_time as i64;
    let nanoseconds = ((epoch_time - seconds as f64) * 1_000_000_000.0) as u32;

    let Some(datetime) = DateTime::from_timestamp(seconds, nanoseconds) else {
        return String::new();
    };
    datetime.format("%Y-%m-%d").to_string()
}
