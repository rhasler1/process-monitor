use super::Error;

use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug)]
pub struct ProcessSample {
    /// Cpu usage across all cores.
    ///
    /// This represents the total CPU usage at the
    /// time the sample was collected and is not a
    /// delta over the sample interval.
    cpu_total:  f32,
    /// Average CPU usage across all cores.
    ///
    /// This represents the average CPU usage at the
    /// time the sample was collected and is not a
    /// delta over the sample interval.
    cpu_avg:    f32,

    /// Memory usage in bytes at the
    /// time the sample was collected.
    mem:        u64
}

/// Records historical resources usage for a single process.
///
/// Samples are stored in a fixed-size ring buffer, with
/// the most recently added sample at the front of the buffer.
///
/// The configured sample interval is used to determine how many
/// samples represent a given time span when calculating historical
/// cpu usage statistics.
///
/// # Invariants:
///
/// - Samples are expected to be collected at approximately `sample_interval`.
/// - `sample_interval` is between one and sixty seconds inclusive.
/// - `sample_interval` is an integral number of seconds.
/// - The number of stored samples never exceeds `capacity`.
#[derive(Debug)]
pub struct ProcessStats {
    /// Ring buffer containing historical process samples.
    samples:            VecDeque<ProcessSample>,
    /// Expected interval between consecutive samples.
    sample_interval:    Duration,
    /// Maximum number of samples retained in the ring buffer.
    capacity:           usize
}

impl ProcessStats {
    /// Maximum number of samples retained in the default ring buffer.
    const DEFAULT_CAPACITY: usize = 60;

    /// Creates an empty `ProcessStats` with the specified sample interval.
    ///
    /// The sample interval must be a whole number of
    /// seconds between one and sixty seconds inclusive.
    ///
    /// # Errors:
    /// Returns error if `sample_interval` if invariant is broken.
    pub fn new(sample_interval: Duration) -> Result<Self, Error> {
        let secs = sample_interval.as_secs();

        if secs == 0 || secs > 60 {
            return Err(Error::ProcessStatsError)
        }

        if sample_interval.subsec_nanos() != 0 {
            return Err(Error::ProcessStatsError)
        }

        Ok(Self {
            samples:    VecDeque::new(),
            sample_interval,
            capacity:   Self::DEFAULT_CAPACITY,
        })
    }

    /// Adds a process resource-usage sample to the
    /// front of the ring buffer, ensuring capacity
    /// is not exceeded by evacuating the oldest sample.
    pub fn push(
        &mut self,
        cpu_total:  f32,
        cpu_avg:    f32,
        mem:        u64) {
        if self.samples.len() == self.capacity {
            self.samples.pop_back();
        }

        self.samples.push_front(ProcessSample {
            cpu_total, 
            cpu_avg,
            mem 
        });
    }

    /// Returns the approximate mean average CPU usage over the
    /// last minute.
    ///
    /// The number of samples included is calculated from the
    /// configured sample interval.
    ///
    /// Returns `None` when fewer samples than required to represent
    /// one minute are currently available.
    pub fn mean_cpu_usage_last_minute(&self) -> Option<f32> {
        self.mean_last_time_span_f32(|sample| sample.cpu_avg, Duration::from_secs(60))
    }

    /// Returns the approximate mean average CPU usage over the
    /// last 30s.
    ///
    /// The number of samples included is calculated from the
    /// configured sample interval.
    ///
    /// Returns `None` when fewer samples than required to represent
    /// one minute are currently available.
    pub fn mean_cpu_usage_last_30s(&self) -> Option<f32> {
        self.mean_last_time_span_f32(|sample| sample.cpu_avg, Duration::from_secs(30))
    }

    /// Returns the approximate mean total CPU usage over the
    /// last minute.
    ///
    /// The number of samples included is calculated from the
    /// configured sample interval.
    ///
    /// Returns `None` when fewer samples than required to represent
    /// one minute are currently available.
    pub fn mean_cpu_usage_as_total_last_minute(&self) -> Option<f32> {
        self.mean_last_time_span_f32(|sample| sample.cpu_total, Duration::from_secs(60))
    }

    /// Returns the approximate mean total CPU usage over the
    /// last minute.
    ///
    /// The number of samples included is calculated from the
    /// configured sample interval.
    ///
    /// Returns `None` when fewer samples than required to represent
    /// one minute are currently available.
    pub fn mean_cpu_usage_as_total_last_30s(&self) -> Option<f32> {
        self.mean_last_time_span_f32(|sample| sample.cpu_total, Duration::from_secs(30))
    }

    // Helper
    fn mean_last_time_span_f32<F>(&self, get_cpu: F, time_span: Duration) -> Option<f32>
    where
        F: Fn(&ProcessSample) -> f32,
    {
        let samples_in_time_span = (time_span.as_secs() / self.sample_interval.as_secs()) as usize;

        if self.samples.len() < samples_in_time_span {
            return None;
        }

        let mut sum = 0.0;

        for sample in self.samples.iter().take(samples_in_time_span) {
            sum += get_cpu(sample);
        }

        Some(sum / samples_in_time_span as f32)
    }

    /// Returns the approximate mean mem usage in bytes over the last minute.
    pub fn mean_mem_usage_last_minute(&self) -> Option<u64> {
        self.mean_last_time_span_u64(|sample| sample.mem, Duration::from_secs(60))
    }

    /// Returns the approximate mean mem usage in bytes over the last 30s.
    pub fn mean_mem_usage_last_30s(&self) -> Option<u64> {
        self.mean_last_time_span_u64(|sample| sample.mem, Duration::from_secs(30))
    }

    /// Helper
    fn mean_last_time_span_u64<F>(&self, get_mem: F, time_span: Duration) -> Option<u64>
    where
        F: Fn(&ProcessSample) -> u64
    {
        let samples_in_time_span = (time_span.as_secs() / self.sample_interval.as_secs()) as usize;

        if self.samples.len() < samples_in_time_span {
            return None;
        }

        let mut sum = 0;

        for sample in self.samples.iter().take(samples_in_time_span) {
            sum += get_mem(sample);
        }

        Some(sum / samples_in_time_span as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_accepts_one_second_interval() {
        let res = ProcessStats::new(Duration::from_secs(1));

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.sample_interval, Duration::from_secs(1));
        assert_eq!(res.capacity, ProcessStats::DEFAULT_CAPACITY);
        assert!(res.samples.is_empty());
    }

    #[test]
    fn test_new_accepts_sixty_second_interval() {
        let result = ProcessStats::new(Duration::from_secs(60));

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_rejects_zero_interval() {
        let result = ProcessStats::new(Duration::ZERO);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_rejects_interval_greater_than_sixty_seconds() {
        let result = ProcessStats::new(Duration::from_secs(61));

        assert!(result.is_err());
    }

    #[test]
    fn test_new_rejects_fractional_second_interval() {
        let result = ProcessStats::new(Duration::from_millis(1500));

        assert!(result.is_err());
    }

    #[test]
    fn test_push_adds_sample() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();

        stats.push(100.0, 50.0, 1024);

        assert_eq!(stats.samples.len(), 1);

        let sample = stats.samples.front().unwrap();

        assert_eq!(sample.cpu_total, 100.0);
        assert_eq!(sample.cpu_avg, 50.0);
        assert_eq!(sample.mem, 1024);
    }

    #[test]
    fn test_push_adds_newest_sample_to_front() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();

        stats.push(10.0, 1.0, 100);
        stats.push(20.0, 2.0, 200);
        stats.push(30.0, 3.0, 300);

        assert_eq!(stats.samples.len(), 3);

        assert_eq!(stats.samples[0].cpu_avg, 3.0);
        assert_eq!(stats.samples[1].cpu_avg, 2.0);
        assert_eq!(stats.samples[2].cpu_avg, 1.0);
    }

    #[test]
    fn test_push_evicts_oldest_sample_when_capacity_is_reached() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();

        for i in 0..=ProcessStats::DEFAULT_CAPACITY {
            stats.push(i as f32, i as f32, i as u64);
        }

        assert_eq!(stats.samples.len(), ProcessStats::DEFAULT_CAPACITY);

        // The newest sample should be retained.
        assert_eq!(
            stats.samples.front().unwrap().cpu_avg,
            ProcessStats::DEFAULT_CAPACITY as f32
        );

        // The oldest sample should be sample 1, since sample 0 was evicted.
        assert_eq!(stats.samples.back().unwrap().cpu_avg, 1.0);
    }

    #[test]
    fn test_mean_cpu_usage_last_minute_returns_none_when_insufficient_samples() {

        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
                                 
        for i in 0..59 {
            stats.push(i as f32, i as f32, 0);
        }

        assert_eq!(stats.mean_cpu_usage_last_minute(), None);
    }

    #[test]
    fn test_mean_cpu_usage_last_minute_uses_sixty_samples_for_one_second_interval() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        for i in 1..=60 {
            stats.push(0.0, i as f32, 0);
        }

        let mean = stats.mean_cpu_usage_last_minute().unwrap();

        assert_eq!(mean, 30.5);
    }

    #[test]
    fn test_mean_cpu_usage_last_minute_uses_twelve_samples_for_five_second_interval() {
        let mut stats = ProcessStats::new(Duration::from_secs(5)).unwrap();
        
        for i in 1..=12 {
            stats.push(0.0, i as f32, 0);
        }

        let mean = stats.mean_cpu_usage_last_minute().unwrap();

        assert_eq!(mean, 6.5);
    }

    #[test]
    fn test_mean_cpu_usage_last_30s_returns_none_when_insufficient_samples() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        for i in 0..29 {
            stats.push(0.0, i as f32, 0);
        }

        assert_eq!(stats.mean_cpu_usage_last_30s(), None);
    }

    #[test]
    fn test_mean_cpu_usage_last_30s_uses_thirty_samples_for_one_second_interval() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();

        for i in 1..=30 {
            stats.push(0.0, i as f32, 0);
        }

        let mean = stats.mean_cpu_usage_last_30s().unwrap();

        assert_eq!(mean, 15.5);
    }

    #[test]
    fn test_mean_cpu_usage_last_30s_uses_six_samples_for_five_second_interval() {
        let mut stats = ProcessStats::new(Duration::from_secs(5)).unwrap();
        
        for i in 1..=6 {
            stats.push(0.0, i as f32, 0);
        }

        let mean = stats.mean_cpu_usage_last_30s().unwrap();

        assert_eq!(mean, 3.5);
    }

    #[test]
    fn test_mean_cpu_usage_as_total_last_minute_uses_cpu_total() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        for i in 1..=60 {
            stats.push(i as f32, 999.0, 0);
        }

        let mean = stats.mean_cpu_usage_as_total_last_minute().unwrap();

        assert_eq!(mean, 30.5);
    }

    #[test]
    fn test_mean_cpu_usage_as_total_last_30s_uses_cpu_total() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        for i in 1..=30 {
            stats.push(i as f32, 999.0, 0);
        }

        let mean = stats.mean_cpu_usage_as_total_last_30s().unwrap();

        assert_eq!(mean, 15.5);
    }

    #[test]
    fn test_mean_uses_only_samples_in_requested_time_span() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        // The first 30 samples are the ones that should be included.
        for i in 1..=60 {
            stats.push(0.0, i as f32, 0);
        }

        let mean = stats.mean_cpu_usage_last_30s().unwrap();

        // Because push() puts the newest sample first, the 30 samples
        // used are 60 down through 31.
        assert_eq!(mean, 45.5);
    }

    #[test]
    fn test_mean_returns_none_when_no_samples_exist() {
        let stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        assert_eq!(stats.mean_cpu_usage_last_minute(), None);
        assert_eq!(stats.mean_cpu_usage_last_30s(), None);
        assert_eq!(stats.mean_cpu_usage_as_total_last_minute(), None);
        assert_eq!(stats.mean_cpu_usage_as_total_last_30s(), None);
    }

    #[test]
    fn test_memory_value_does_not_affect_cpu_means() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        for i in 1..=60 {
            stats.push(0.0, i as f32, i * 1000);
        }

        assert_eq!(stats.mean_cpu_usage_last_minute().unwrap(), 30.5);
        assert_eq!(
            stats.mean_cpu_usage_as_total_last_minute().unwrap(),
            0.0
        );
    }

    #[test]
    fn test_sixty_second_interval_with_thirty_second_span_returns_none() {
        let mut stats = ProcessStats::new(Duration::from_secs(1)).unwrap();
        
        stats.push(10.0, 10.0, 0);

        // A 30-second window cannot contain even one 60-second sample.
        //
        // This currently exposes a bug: samples_in_time_span == 0,
        // resulting in 0/0 and therefore NaN.
        assert_eq!(stats.mean_cpu_usage_last_30s(), None);
    }
}

