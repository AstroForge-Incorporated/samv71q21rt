#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tc_channel: (),
    _reserved1: [u8; 0xc0],
    bcr: Bcr,
    bmr: Bmr,
    qier: Qier,
    qidr: Qidr,
    qimr: Qimr,
    qisr: Qisr,
    fmr: Fmr,
    _reserved8: [u8; 0x08],
    wpmr: Wpmr,
}
impl RegisterBlock {
    #[doc = "0x00..0x9c - Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn tc_channel(&self, n: usize) -> &TcChannel {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x9c - Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub fn tc_channel_iter(&self) -> impl Iterator<Item = &TcChannel> {
        (0..3).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64 * n).cast() })
    }
    #[doc = "0xc0 - Block Control Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0xc4 - Block Mode Register"]
    #[inline(always)]
    pub const fn bmr(&self) -> &Bmr {
        &self.bmr
    }
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn qier(&self) -> &Qier {
        &self.qier
    }
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn qidr(&self) -> &Qidr {
        &self.qidr
    }
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn qimr(&self) -> &Qimr {
        &self.qimr
    }
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    #[inline(always)]
    pub const fn qisr(&self) -> &Qisr {
        &self.qisr
    }
    #[doc = "0xd8 - Fault Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
}
#[doc = "Channel Control Register (channel = 0)"]
pub use self::tc_channel::TcChannel;
#[doc = r"Cluster"]
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_channel;
#[doc = "BCR (w) register accessor: Block Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: Block Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmr`] module"]
#[doc(alias = "BMR")]
pub type Bmr = crate::Reg<bmr::BmrSpec>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER (w) register accessor: QDEC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qier`] module"]
#[doc(alias = "QIER")]
pub type Qier = crate::Reg<qier::QierSpec>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR (w) register accessor: QDEC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qidr`] module"]
#[doc(alias = "QIDR")]
pub type Qidr = crate::Reg<qidr::QidrSpec>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR (r) register accessor: QDEC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qimr`] module"]
#[doc(alias = "QIMR")]
pub type Qimr = crate::Reg<qimr::QimrSpec>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR (r) register accessor: QDEC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qisr`] module"]
#[doc(alias = "QISR")]
pub type Qisr = crate::Reg<qisr::QisrSpec>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "FMR (rw) register accessor: Fault Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`] module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
