#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__rdtscp;

#[derive(Clone, Copy, Debug)]
pub struct TscSample {
    pub ticks: u64,
    pub aux: u32,
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn read() -> TscSample {
    let mut aux = 0u32;

    let ticks = unsafe {
        __rdtscp(&mut aux)
    };

    TscSample { ticks, aux }
}
