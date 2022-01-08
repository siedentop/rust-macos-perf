use crate::bindings;

#[derive(Debug, Clone)]
pub struct PerformanceCounters {
    pub cycles: f64,
    pub branches: f64,
    pub missed_branches: f64,
    pub instructions: f64,
}
impl From<bindings::performance_counters> for PerformanceCounters {
    fn from(counters: bindings::performance_counters) -> Self {
        PerformanceCounters {
            cycles: counters.cycles,
            branches: counters.branches,
            missed_branches: counters.missed_branches,
            instructions: counters.instructions,
        }
    }
}

impl std::ops::Div<usize> for PerformanceCounters {
    type Output = PerformanceCounters;
    fn div(self, rhs: usize) -> Self::Output {
        let rhs = rhs as f64;
        PerformanceCounters {
            cycles: self.cycles / rhs,
            branches: self.branches / rhs,
            missed_branches: self.missed_branches / rhs,
            instructions: self.instructions / rhs,
        }
    }
}

impl std::ops::Sub<PerformanceCounters> for PerformanceCounters {
    type Output = PerformanceCounters;
    fn sub(self, rhs: PerformanceCounters) -> Self::Output {
        PerformanceCounters {
            cycles: self.cycles - rhs.cycles,
            branches: self.branches - rhs.branches,
            missed_branches: self.missed_branches - rhs.missed_branches,
            instructions: self.instructions - rhs.instructions,
        }
    }
}
