
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Benchmark {
    // Risc0 receipt
    pub r0_receipt_blob: Vec<u8>,
    
    // Risc0 image_id
    pub r0_image_id: [u32; 8],
    
    // time it took to run this benchmark
    pub duration_msecs: u128,
    
    // creation time, seconds from the epoch(utc timezone)
    pub timestamp: i64,
}
