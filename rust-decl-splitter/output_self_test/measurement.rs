pub use function_metrics::FunctionMetrics
lazy_static :: lazy_static ! { static ref METRICS : Mutex < HashMap < String , FunctionMetrics >> = Mutex :: new (HashMap :: new ()) ; }
pub use record_function_entry::record_function_entry
pub use record_function_exit::record_function_exit
pub use get_collected_metrics::get_collected_metrics
