use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ReadinessProbe(pub Arc<AtomicBool>);

impl ReadinessProbe {
    pub fn new_ready() -> Self {
        tracing::debug!("ReadinessProbe::new_ready");
        let r = Self::default();
        r.set_ready(true);
        r
    }

    pub fn is_ready(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    pub fn set_ready(&self, v: bool) {
        tracing::debug!("ReadinessProbe::set_ready: v={}", v);
        self.0.store(v, Ordering::SeqCst);
    }
}
