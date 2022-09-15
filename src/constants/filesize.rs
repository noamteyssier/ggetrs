pub const KB_LIMIT: usize = 1024;
pub const MB_LIMIT: usize = 1_048_576;
pub const GB_LIMIT: usize = 1_073_741_824;
pub const TB_LIMIT: usize = 1_099_511_627_776;

/// Converts a bits size to a string representation
#[must_use]
pub fn convert_mem_label(size: usize) -> String {
    let (dividend, units) = if size < KB_LIMIT {
        (1.0, "B")
    } else if size < MB_LIMIT {
        (KB_LIMIT as f64, "K")
    } else if size < GB_LIMIT {
        (MB_LIMIT as f64, "M")
    } else if size < TB_LIMIT {
        (GB_LIMIT as f64, "G")
    } else {
        (TB_LIMIT as f64, "T")
    };
    let repr = (size as f64 / dividend).round() as usize;
    format!("{}{}", repr, units)
}
