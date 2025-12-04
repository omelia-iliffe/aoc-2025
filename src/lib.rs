pub mod template;

// Use this file to add helper functions and additional modules.

pub fn index_2_cood(index: usize, line_len: usize) -> (usize, usize) {
    (index % line_len, index / line_len)
}
pub fn cood_2_index(c: (usize, usize), line_len: usize) -> usize {
    c.1 * line_len + c.0
}
