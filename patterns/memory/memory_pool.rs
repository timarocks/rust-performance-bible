//! Memory Pool Implementation
//! 
//! A high-performance memory pool for avoiding allocations in hot paths.
//! Used in article 001-memory-is-not-free.md

use std::cell::RefCell;
use std::mem::MaybeUninit;

/// A memory pool that can allocate T without heap allocation
pub struct MemoryPool<T> {
    storage: RefCell<Vec<MaybeUninit<T>>>,
    free_list: RefCell<Vec<usize>>,
    capacity: usize,
}

impl<T> MemoryPool<T> {
    /// Create a new pool with the given capacity
    pub fn new(capacity: usize) -> Self {
        let mut storage = Vec::with_capacity(capacity);
        let mut free_list = Vec::with_capacity(capacity);
        
        // Initialize with uninitialized memory
        for i in 0..capacity {
            storage.push(MaybeUninit::uninit());
            free_list.push(capacity - i - 1); // Reverse order for cache
        }
        
        Self {
            storage: RefCell::new(storage),
            free_list: RefCell::new(free_list),
            capacity,
        }
    }
    
    /// Allocate a slot from the pool
    pub fn allocate(&self) -> Option<PoolGuard<T>> {
        let mut free_list = self.free_list.borrow_mut();
        
        if let Some(index) = free_list.pop() {
            Some(PoolGuard {
                pool: self,
                index,
                _phantom: std::marker::PhantomData,
            })
        } else {
            None
        }
    }
    
    /// Return a slot to the pool
    fn deallocate(&self, index: usize) {
        self.free_list.borrow_mut().push(index);
    }
}

/// RAII guard for pool allocations
pub struct PoolGuard<'a, T> {
    pool: &'a MemoryPool<T>,
    index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Drop for PoolGuard<'a, T> {
    fn drop(&mut self) {
        self.pool.deallocate(self.index);
    }
}

impl<'a, T> std::ops::Deref for PoolGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe {
            let storage = self.pool.storage.borrow();
            &*storage[self.index].as_ptr()
        }
    }
}

impl<'a, T> std::ops::DerefMut for PoolGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let mut storage = self.pool.storage.borrow_mut();
            &mut *storage[self.index].as_mut_ptr()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::<String>::new(10);
        
        // Allocate and use
        {
            let mut guard = pool.allocate().unwrap();
            *guard = String::from("Hello, World!");
            assert_eq!(&**guard, "Hello, World!");
        }
        
        // Should be able to allocate again after drop
        let guard2 = pool.allocate().unwrap();
        assert!(guard2.index < 10);
    }
}