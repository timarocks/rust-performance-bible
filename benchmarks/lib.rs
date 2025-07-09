//! Benchmarking library for Rust Performance Bible
//! 
//! This module contains shared utilities and test data generation for benchmarks.

pub mod test_utils {
    use std::fmt::Write;

    /// Generate test log data with the specified number of lines
    pub fn generate_log_data(lines: usize) -> String {
        let mut data = String::with_capacity(lines * 100);
        for i in 0..lines {
            writeln!(
                &mut data,
                "2025-01-01T12:00:00|ERROR|Database connection failed|retry={}|timeout=30|attempt=1|user=admin|ip=127.0.0.1",
                i
            ).unwrap();
        }
        data
    }
}
