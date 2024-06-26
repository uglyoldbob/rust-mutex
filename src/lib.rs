pub use tracing_mutex::parking_lot;

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    inner: tracing_mutex::parking_lot::MutexGuard<'a, T>,
}

impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = tracing_mutex::parking_lot::MutexGuard<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.drop_obtained();
    }
}

pub struct Mutex<T> {
    inner: tracing_mutex::parking_lot::Mutex<T>,
    obtained: tracing_mutex::parking_lot::Mutex<Option<backtrace::Backtrace>>,
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: tracing_mutex::parking_lot::Mutex::new(t),
            obtained: tracing_mutex::parking_lot::Mutex::new(None),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        let mut t = self.is_obtained().lock();
        if let Some(s) = t.as_ref() {
            println!("Deadlock: {:?}", s);
        }
        t.replace(backtrace::Backtrace::new());
        MutexGuard {
            mutex: &self,
            inner: self.inner.lock(),
        }
    }

    pub fn is_obtained(&self) -> &tracing_mutex::parking_lot::Mutex<Option<backtrace::Backtrace>> {
        &self.obtained
    }

    fn drop_obtained(&self) {
        let mut a = self.obtained.lock();
        a.take();
    }
}

