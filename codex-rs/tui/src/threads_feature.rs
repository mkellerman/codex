use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

static THREADS_ENABLED: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_threads_enabled(enabled: bool) {
    THREADS_ENABLED.store(enabled, Ordering::Relaxed);
}

pub(crate) fn is_threads_enabled() -> bool {
    THREADS_ENABLED.load(Ordering::Relaxed)
}
