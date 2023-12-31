// Copyright 2017 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::BinaryHeap;
use std::sync::{Mutex, MutexGuard};
use std::usize;

use crate::InitCell;

// Thread ID manager which allocates thread IDs. It attempts to aggressively
// reuse thread IDs where possible to avoid cases where a ThreadLocal grows
// indefinitely when it is used by many short-lived threads.
struct ThreadIdManager {
    limit: usize,
    free_list: BinaryHeap<usize>,
}
impl ThreadIdManager {
    fn new() -> ThreadIdManager {
        ThreadIdManager {
            limit: usize::MAX,
            free_list: BinaryHeap::new(),
        }
    }
    fn alloc(&mut self) -> usize {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            let id = self.limit;
            self.limit = self.limit.checked_sub(1).expect("Ran out of thread IDs");
            id
        }
    }
    fn free(&mut self, id: usize) {
        self.free_list.push(id);
    }
}


fn thread_id_manager() -> MutexGuard<'static, ThreadIdManager> {
    static THREAD_ID_MANAGER: InitCell<Mutex<ThreadIdManager>> = InitCell::new();

    THREAD_ID_MANAGER.get_or_init(|| Mutex::new(ThreadIdManager::new())).lock().unwrap()
}

// Non-zero integer which is unique to the current thread while it is running.
// A thread ID may be reused after a thread exits.
struct ThreadId(usize);
impl ThreadId {
    fn new() -> ThreadId {
        ThreadId(thread_id_manager().alloc())
    }
}
impl Drop for ThreadId {
    fn drop(&mut self) {
        thread_id_manager().free(self.0)
    }
}
thread_local!(static THREAD_ID: ThreadId = ThreadId::new());

/// Returns a non-zero ID for the current thread
pub fn get() -> Option<usize> {
    THREAD_ID.try_with(|x| x.0).ok()
}
