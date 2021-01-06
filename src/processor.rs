//! Common utility functions for working with ARM Cortex-M processors.

#![cfg_attr(feature = "std", allow(unreachable_code, unused_mut))]

/// Waits for interrupt.
///
/// It is a hint instruction. It suspends execution, in the lowest power state
/// available consistent with a fast wakeup without the need for software
/// restoration, until a reset, asynchronous exception or other event occurs.
#[inline]
pub fn wait_for_int() {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { llvm_asm!("wfi" :::: "volatile") };
}

/// Waits for event.
///
/// It is a hint instruction. If the Event Register is clear, it suspends
/// execution in the lowest power state available consistent with a fast wakeup
/// without the need for software restoration, until a reset, exception or other
/// event occurs.
///
/// See also [`send_event`].
#[inline]
pub fn wait_for_event() {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { llvm_asm!("wfe" :::: "volatile") };
}

/// Sends event.
///
/// It is a hint instruction. It causes an event to be signaled to all CPUs
/// within the multiprocessor system.
///
/// See also [`wait_for_event`].
#[inline]
pub fn send_event() {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { llvm_asm!("sev" :::: "volatile") };
}

/// Requests system reset.
///
/// Generates a system reset request to the microcontroller's system reset
/// control logic. Because the system reset control logic is not a part of the
/// processor design, the exact timing of the reset is device-specific.
///
/// The debug logic is not affected.
#[allow(clippy::empty_loop)]
#[inline]
pub fn self_reset() -> ! {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe {
        use crate::{map::reg::scb, reg::prelude::*};
        use drone_core::token::Token;
        llvm_asm!("
            dmb
            cpsid f
        "   :
            :
            :
            : "volatile"
        );
        scb::Aircr::<Urt>::take().store(|r| r.write_vectkey(0x05FA).set_sysresetreq());
        loop {}
    }
}

/// Spins the `cycles` number of processor cycles in a loop.
#[allow(unused_assignments, unused_variables)]
#[inline(always)]
pub fn spin(mut cycles: u32) {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe {
        llvm_asm!("
        0:
            subs $0, $0, #3
            bhi 0b
        "   : "+r"(cycles)
            :
            : "cc"
            : "volatile"
        );
    }
}

/// Enables the FPU.
///
/// The FPU is disabled from reset. You must enable it before you can use any
/// floating-point instructions.
///
/// # Safety
///
/// * The processor must be in privileged mode
/// * The function rewrites contents of FPU_CPACR register without taking into
///   account register tokens
#[cfg(feature = "floating-point-unit")]
#[inline]
pub unsafe fn fpu_init(full_access: bool) {
    const FPU_CPACR: usize = 0xE000_ED88;
    unsafe {
        core::ptr::write_volatile(
            FPU_CPACR as *mut u32,
            if full_access {
                0xF // full access
            } else {
                0x5 // privileged access only
            } << 20,
        );
    }
}

pub mod interrupt {
    //! Utility function for working with global interrupts.
    
    use core::sync::atomic::{Ordering, compiler_fence};

    /// Disables all interrupts
    #[inline]
    pub fn disable() {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe { asm!("cpsid i") };
        compiler_fence(Ordering::SeqCst);
    }

    /// Enables all (not masked) interrupts
    ///
    /// # Safety
    ///
    /// - Do not call this function inside a critical section.
    #[inline]
    pub unsafe fn enable() {
        #[cfg(feature = "std")]
        return unimplemented!();
        compiler_fence(Ordering::SeqCst);
        unsafe { asm!("cpsie i") };
    }

    #[inline]
    fn primask() -> u32 {
        #[cfg(feature = "std")]
        return unimplemented!();
        let r;
        unsafe { asm!("mrs {}, PRIMASK", out(reg) r) };
        r
    }

    /// Critical section token.
    pub struct CriticalSection;

    /// Execute the closure `f` in an interrupt-free context.
    #[inline]
    pub fn critical<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        let pm = primask();

        // Disable interrupts - they may already be disabled if this is a nested critical section.
        disable();

        let cs = CriticalSection;
        let r = f(&cs);

        // Only enable interrupt if interrupts were active when entering.
        if pm & 1 == 0 {
            unsafe { enable() }
        }

        r
    }
}