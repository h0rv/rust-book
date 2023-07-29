use std::error::Error;
use std::fmt;
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create new new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size > 0 {
            return Err(ThreadPoolError::ZeroThreads);
        }

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create threads and store them in the vector
        }

        Ok(ThreadPool { threads })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

#[derive(Debug)]
pub enum ThreadPoolError {
    ZeroThreads,
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadPoolError::ZeroThreads => write!(f, "Number of threads in pool must be >= 1"),
        }
    }
}
impl Error for ThreadPoolError {}
