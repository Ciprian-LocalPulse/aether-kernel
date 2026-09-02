//! Scheduling: Earliest-Deadline-First (EDF) for real-time perception/sync
//! workloads, plus a best-effort round-robin class for everything else.
//! Blueprint reference: §4.2; admission control per
//! `docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md` §7.3.

use crate::process::ProcessId;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingClass {
    /// Hard/soft real-time task with a deadline (perception pipeline,
    /// sync-engine tick). Scheduled EDF: earliest deadline runs first.
    /// `budget_ns` is the task's declared Worst-Case Execution Time
    /// (WCET) — the resource the admission test is protecting.
    RealTime { deadline_ns: u64, budget_ns: u64 },
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
            (
                SchedulingClass::RealTime { deadline_ns: a, .. },
                SchedulingClass::RealTime { deadline_ns: b, .. },
            ) => b.cmp(&a),
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

/// Why an admission request was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum AdmissionError {
    /// Admitting this task would push total EDF utilization over the
    /// schedulability bound (U <= 1) within the configured period —
    /// i.e. the real-time queue is already fully booked and cannot
    /// honestly promise this task's deadline too.
    #[error(
        "EDF utilization bound exceeded: {projected_ns}/{period_ns} ns \
         requested (> 100% of the scheduling period) — task rejected"
    )]
    UtilizationExceeded { projected_ns: u64, period_ns: u64 },
}

/// The kernel scheduler. `tick()` pops the highest-priority runnable
/// task; `try_admit()` performs EDF admission control before a
/// real-time task is allowed onto the real-time queue at all — a
/// scheduler that always accepts real-time work can silently promise
/// deadlines it cannot keep, which is worse than rejecting the work
/// up front.
pub struct Scheduler {
    real_time_queue: BinaryHeap<ScheduledTask>,
    best_effort_queue: std::collections::VecDeque<ProcessId>,
    /// Sum of `budget_ns` across all currently-admitted real-time tasks.
    admitted_budget_ns: u64,
    /// The scheduling period the utilization bound is measured against
    /// (i.e. the window within which admitted tasks' budgets must fit).
    /// Defaults to 1ms, matching the blueprint's sub-10ms sync-latency
    /// target order of magnitude; callers driving a different tick rate
    /// should set this via `with_period`.
    period_ns: u64,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            real_time_queue: BinaryHeap::new(),
            best_effort_queue: std::collections::VecDeque::new(),
            admitted_budget_ns: 0,
            period_ns: 1_000_000, // 1ms
        }
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_period(period_ns: u64) -> Self {
        Self { period_ns, ..Self::default() }
    }

    /// Admit a real-time task, rejecting it if doing so would exceed the
    /// EDF schedulability bound (simplified test: total admitted budget
    /// over the period must stay <= 1.0). Best-effort tasks always
    /// succeed — they carry no deadline promise to break.
    pub fn try_admit(&mut self, pid: ProcessId, class: SchedulingClass) -> Result<(), AdmissionError> {
        if let SchedulingClass::RealTime { budget_ns, .. } = class {
            let projected = self.admitted_budget_ns + budget_ns;
            if projected > self.period_ns {
                return Err(AdmissionError::UtilizationExceeded {
                    projected_ns: projected,
                    period_ns: self.period_ns,
                });
            }
            self.admitted_budget_ns = projected;
        }
        self.submit(pid, class);
        Ok(())
    }

    /// Enqueue a task without an admission check. Used internally by
    /// `try_admit` after a successful check, and directly for
    /// best-effort work (which has no budget to admit against).
    fn submit(&mut self, pid: ProcessId, class: SchedulingClass) {
        match class {
            SchedulingClass::RealTime { .. } => {
                self.real_time_queue.push(ScheduledTask { pid, class })
            }
            SchedulingClass::BestEffort => self.best_effort_queue.push_back(pid),
        }
    }

    /// Returns the next process to run, preferring the real-time queue.
    /// Popping a real-time task frees its budget back to the admission
    /// pool, since it's no longer occupying scheduled time.
    pub fn tick(&mut self) -> Option<ProcessId> {
        if let Some(task) = self.real_time_queue.pop() {
            if let SchedulingClass::RealTime { budget_ns, .. } = task.class {
                self.admitted_budget_ns = self.admitted_budget_ns.saturating_sub(budget_ns);
            }
            return Some(task.pid);
        }
        self.best_effort_queue.pop_front()
    }

    /// Estimated headroom before the tightest deadline in the RT queue.
    pub fn tightest_deadline(&self) -> Option<Duration> {
        self.real_time_queue.peek().and_then(|t| match t.class {
            SchedulingClass::RealTime { deadline_ns, .. } => Some(Duration::from_nanos(deadline_ns)),
            SchedulingClass::BestEffort => None,
        })
    }

    /// Current admitted-budget utilization as a fraction of the period
    /// (0.0..=1.0 when healthy). Exposed for observability tooling
    /// (`security/` auditors, the Network Analyzer, etc.).
    pub fn utilization(&self) -> f64 {
        self.admitted_budget_ns as f64 / self.period_ns as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rt(deadline_ns: u64, budget_ns: u64) -> SchedulingClass {
        SchedulingClass::RealTime { deadline_ns, budget_ns }
    }

    #[test]
    fn admits_tasks_within_the_utilization_bound() {
        let mut sched = Scheduler::with_period(1_000_000); // 1ms period
        assert!(sched.try_admit(ProcessId(1), rt(500_000, 300_000)).is_ok());
        assert!(sched.try_admit(ProcessId(2), rt(600_000, 400_000)).is_ok());
        // 300_000 + 400_000 = 700_000 <= 1_000_000: still within budget.
        assert!((sched.utilization() - 0.7).abs() < 1e-9);
    }

    #[test]
    fn rejects_task_that_would_exceed_the_utilization_bound() {
        let mut sched = Scheduler::with_period(1_000_000);
        sched.try_admit(ProcessId(1), rt(500_000, 700_000)).unwrap();
        // A second task needing 400_000ns would push total to 1_100_000,
        // over the 1_000_000ns period — must be rejected, not silently
        // accepted and left to miss its deadline later.
        let result = sched.try_admit(ProcessId(2), rt(600_000, 400_000));
        assert!(matches!(
            result,
            Err(AdmissionError::UtilizationExceeded { .. })
        ));
    }

    #[test]
    fn best_effort_tasks_are_never_rejected_by_admission_control() {
        let mut sched = Scheduler::with_period(1); // an absurdly tight period
        assert!(sched.try_admit(ProcessId(1), SchedulingClass::BestEffort).is_ok());
    }

    #[test]
    fn ticking_a_real_time_task_frees_its_budget() {
        let mut sched = Scheduler::with_period(1_000_000);
        sched.try_admit(ProcessId(1), rt(500_000, 900_000)).unwrap();
        assert!((sched.utilization() - 0.9).abs() < 1e-9);

        sched.tick(); // runs and dequeues the task
        assert_eq!(sched.utilization(), 0.0);

        // Budget is free again, so a new task of the same size is admitted.
        assert!(sched.try_admit(ProcessId(2), rt(500_000, 900_000)).is_ok());
    }

    #[test]
    fn tick_prefers_earliest_deadline_within_real_time_queue() {
        let mut sched = Scheduler::with_period(10_000_000);
        sched.try_admit(ProcessId(1), rt(9_000_000, 1_000_000)).unwrap();
        sched.try_admit(ProcessId(2), rt(1_000_000, 1_000_000)).unwrap();
        assert_eq!(sched.tick(), Some(ProcessId(2))); // earlier deadline first
        assert_eq!(sched.tick(), Some(ProcessId(1)));
    }
}
