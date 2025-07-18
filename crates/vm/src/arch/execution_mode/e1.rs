use openvm_instructions::riscv::RV32_REGISTER_AS;

use crate::arch::{execution_mode::E1ExecutionCtx, VmSegmentState};

pub struct E1Ctx {
    instret_end: u64,
    sp_ops: u64,
}

impl E1Ctx {
    pub fn new(instret_end: Option<u64>) -> Self {
        E1Ctx {
            instret_end: if let Some(end) = instret_end {
                end
            } else {
                u64::MAX
            },
            sp_ops: 0,
        }
    }

    pub fn sp_ops(&self) -> u64 {
        self.sp_ops
    }
}

impl Default for E1Ctx {
    fn default() -> Self {
        Self::new(None)
    }
}

impl E1ExecutionCtx for E1Ctx {
    #[inline(always)]
    fn on_memory_operation(&mut self, address_space: u32, ptr: u32, _size: u32) {
        if address_space == RV32_REGISTER_AS && ptr == 2 {
            self.sp_ops += 1
        }
    }

    #[inline(always)]
    fn should_suspend<F>(vm_state: &mut VmSegmentState<F, Self>) -> bool {
        vm_state.instret >= vm_state.ctx.instret_end
    }
}
