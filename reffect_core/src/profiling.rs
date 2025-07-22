use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

static ENABLED: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

#[inline]
pub fn toggle(enabled: bool) {
    log::debug!("Profiling set to {enabled}");
    ENABLED.store(enabled, Ordering::Relaxed);
}

#[inline]
pub fn measure<R>(work: impl FnOnce() -> R, _log: impl FnOnce(Duration)) -> R {
    if cfg!(feature = "profile") {
        measure_always(work, _log)
    } else {
        work()
    }
}

#[inline]
pub fn measure_always<R>(work: impl FnOnce() -> R, log: impl FnOnce(Duration)) -> R {
    let start = Instant::now();
    let value = work();
    log(start.elapsed());
    value
}
