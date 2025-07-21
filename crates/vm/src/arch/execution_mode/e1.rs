use std::collections::HashMap;

use crate::arch::{execution_mode::E1ExecutionCtx, VmSegmentState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemOp {
    Read,
    Write,
}

impl From<bool> for MemOp {
    fn from(is_write: bool) -> Self {
        if is_write {
            MemOp::Write
        } else {
            MemOp::Read
        }
    }
}

pub struct E1Ctx {
    instret_end: u64,
    stats: HashMap<(u32, u32, MemOp), u64>, // (address_space, ptr) => cnt
    loadstore_stats: HashMap<(u32, u32, MemOp), u64>, // (address_space, ptr) => cnt
}

impl E1Ctx {
    pub fn new(instret_end: Option<u64>) -> Self {
        E1Ctx {
            instret_end: if let Some(end) = instret_end {
                end
            } else {
                u64::MAX
            },
            stats: HashMap::new(),
            loadstore_stats: HashMap::new(),
        }
    }

    pub fn stats(&self) -> &HashMap<(u32, u32, MemOp), u64> {
        &self.stats
    }

    pub fn loadstore_stats(&self) -> &HashMap<(u32, u32, MemOp), u64> {
        &self.loadstore_stats
    }
}

impl Default for E1Ctx {
    fn default() -> Self {
        Self::new(None)
    }
}

impl E1ExecutionCtx for E1Ctx {
    #[inline(always)]
    fn on_memory_operation(
        &mut self,
        address_space: u32,
        ptr: u32,
        _size: u32,
        is_loadstore: bool,
        is_write: bool,
    ) {
        let mem_op = MemOp::from(is_write);
        if is_loadstore {
            *self
                .loadstore_stats
                .entry((address_space, ptr, mem_op))
                .or_insert(0) += 1;
        }
        *self.stats.entry((address_space, ptr, mem_op)).or_insert(0) += 1;
    }

    #[inline(always)]
    fn should_suspend<F>(vm_state: &mut VmSegmentState<F, Self>) -> bool {
        vm_state.instret >= vm_state.ctx.instret_end
    }
}
