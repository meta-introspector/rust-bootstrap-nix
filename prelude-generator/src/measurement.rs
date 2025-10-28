use std::time::Instant;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::Serialize;

// A simple struct to hold function metrics
#[derive(Debug, Serialize, Clone)]
pub struct FunctionMetrics {
    #[serde(skip)]
    pub start_time: Instant,
    #[serde(skip)]
    pub end_time: Option<Instant>,
    pub duration_micros: Option<u128>, // Store duration in microseconds
    pub call_count: u64,
}

impl FunctionMetrics {
    pub fn new() -> Self {
        FunctionMetrics {
            start_time: Instant::now(),
            end_time: None,
            duration_micros: None,
            call_count: 0,
        }
    }
}

// Global storage for metrics, protected by a Mutex
lazy_static::lazy_static! {
    static ref METRICS: Mutex<HashMap<String, FunctionMetrics>> = Mutex::new(HashMap::new());
}

// Function to be called at the start of a wrapped function
pub fn record_function_entry(function_name: &str) {
    let mut metrics = METRICS.lock().unwrap();
    let entry = metrics.entry(function_name.to_string()).or_insert_with(FunctionMetrics::new);
    entry.start_time = Instant::now();
    entry.call_count += 1;
    // println!("[MEASURE] Entering function: {}"); // Removed print
}

// Function to be called at the end of a wrapped function
pub fn record_function_exit(function_name: &str) {
    let mut metrics = METRICS.lock().unwrap();
    if let Some(entry) = metrics.get_mut(function_name) {
        entry.end_time = Some(Instant::now());
        let duration = entry.end_time.unwrap().duration_since(entry.start_time);
        entry.duration_micros = Some(duration.as_micros());
        // println!("[MEASURE] Exiting function: {} (Duration: {:?})"); // Removed print
    }
}

// New function to retrieve collected metrics
pub fn get_collected_metrics() -> HashMap<String, FunctionMetrics> {
    METRICS.lock().unwrap().clone()
}
