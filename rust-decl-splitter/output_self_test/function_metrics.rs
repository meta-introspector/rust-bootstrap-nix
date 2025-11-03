
impl FunctionMetrics { pub fn new () -> Self { FunctionMetrics { start_time : Instant :: now () , end_time : None , duration_micros : None , call_count : 0 , } } }
