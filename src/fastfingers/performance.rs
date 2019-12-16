use std::error;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
pub struct PerformanceMonitor {
    start: Option<Instant>,
    end: Option<Instant>,
    correct: u32,
    attempted: u32,
}

impl PerformanceMonitor {
    pub fn new() -> PerformanceMonitor {
        PerformanceMonitor {
            start: None,
            end: None,
            correct: 0,
            attempted: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), PerformanceMonitorError> {
        match self.start {
            Some(_) => Err(PerformanceMonitorError),
            None => {
                self.start = Some(Instant::now());
                Ok(())
            }
        }
    }

    pub fn end(&mut self) -> Result<(), PerformanceMonitorError> {
        match self.end {
            Some(_) => Err(PerformanceMonitorError),
            None => {
                self.end = Some(Instant::now());
                Ok(())
            }
        }
    }

    pub fn correct(&self) -> u32 {
        self.correct
    }

    pub fn attempted(&self) -> u32 {
        self.attempted
    }

    pub fn accuracy(&self) -> Result<f32, PerformanceMonitorError> {
        match self.attempted {
            0 => Err(PerformanceMonitorError),
            _ => Ok(self.correct as f32 / self.attempted as f32),
        }
    }

    pub fn duration(&self) -> Result<std::time::Duration, PerformanceMonitorError> {
        match self.start {
            Some(start) => match self.end {
                Some(end) => Ok(end - start),
                _ => Ok(Instant::now() - start),
            },
            None => Err(PerformanceMonitorError),
        }
    }

    pub fn wps(&self) -> Result<f32, PerformanceMonitorError> {
        match self.duration() {
            Ok(duration) => Ok(self.correct as f32 / duration.as_secs_f32()),
            Err(_) => Err(PerformanceMonitorError),
        }
    }

    pub fn wpm(&self) -> Result<f32, PerformanceMonitorError> {
        self.wps().map(|wps| wps * 60f32)
    }

    pub fn register(&mut self, entered: &str, expected: &str) {
        self.attempted += 1;
        if entered == expected {
            self.correct += 1;
        }
    }
}

impl fmt::Display for PerformanceMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Correct: {}\nAttempted: {}\nAccuracy: {}\nWPM: {}\nDuration: {:?}",
            self.correct(),
            self.attempted(),
            self.accuracy().unwrap_or(0 as f32),
            self.wpm().unwrap_or(0 as f32),
            self.duration().unwrap_or_default(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct PerformanceMonitorError;

impl fmt::Display for PerformanceMonitorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Performance monitor error.")
    }
}

impl error::Error for PerformanceMonitorError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
