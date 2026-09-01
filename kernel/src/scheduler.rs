//! Scheduling: Earliest-Deadline-First (EDF) for real-time perception/sync
//! workloads, plus a best-effort round-robin class for everything else.
//! Blueprint reference: §4.2.

use crate::process::ProcessId;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingClass {
    /// Hard/soft real-time task with a deadline (perception pipeline,
    /// sync-engine tick). Scheduled EDF: earliest deadline runs first.
    RealTime { deadline_ns: u64 },
    /// Everything else: round robin, no deadline guarantees.
    BestEffort,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ScheduledTask {
    pid: ProcessId,
    class: SchedulingClass,
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap; we want the *smallest* deadline first,
        // so invert the comparison. RealTime always outranks BestEffort.
        match (self.class, other.class) {
            (SchedulingClass::RealTime { deadline_ns: a }, SchedulingClass::RealTime { deadline_ns: b }) => {
                b.cmp(&a)
            }
            (SchedulingClass::RealTime { .. }, SchedulingClass::BestEffort) => Ordering::Greater,
            (SchedulingClass::BestEffort, SchedulingClass::RealTime { .. }) => Ordering::Less,
            (SchedulingClass::BestEffort, SchedulingClass::BestEffort) => Ordering::Equal,
        }
    }
}
impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The kernel scheduler. A thin, deterministic core: `tick()` pops the
/// highest-priority runnable task. Real-time correctness (missed-deadline
/// detection, admission control) is intentionally left as an extension
/// point — see `docs/roadmap/ROADMAP.md` Stage 2.
#[derive(Default)]
pub struct Scheduler {
    real_time_queue: BinaryHeap<ScheduledTask>,
    best_effort_queue: std::collections::VecDeque<ProcessId>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submit(&mut self, pid: ProcessId, class: SchedulingClass) {
        match class {
            SchedulingClass::RealTime { .. } => {
                self.real_time_queue.push(ScheduledTask { pid, class })
            }
            SchedulingClass::BestEffort => self.best_effort_queue.push_back(pid),
        }
    }

    /// Returns the next process to run, preferring the real-time queue.
    pub fn tick(&mut self) -> Option<ProcessId> {
        if let Some(task) = self.real_time_queue.pop() {
            return Some(task.pid);
        }
        self.best_effort_queue.pop_front()
    }

    /// Estimated headroom before the tightest deadline in the RT queue.
    pub fn tightest_deadline(&self) -> Option<Duration> {
        self.real_time_queue.peek().and_then(|t| match t.class {
            SchedulingClass::RealTime { deadline_ns } => Some(Duration::from_nanos(deadline_ns)),
            SchedulingClass::BestEffort => None,
        })
    }
}
