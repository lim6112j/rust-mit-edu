pub struct ThreadPool;
impl ThreadPool {
    /// create new thread pool
    ///
    /// the size is the number of threads  
    ///
    /// # panics
    /// the new function will panic if size is zero
    ///
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {

    }
}
