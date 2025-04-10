#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    ctrl: Ctrl,
    sr: Sr,
    _reserved3: [u8; 0x04],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    uasr: Uasr,
    _reserved8: [u8; 0x0c],
    dscr: Dscr,
    hash: Hash,
    uihval: [Uihval; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x20 - Undefined Access Status Register"]
    #[inline(always)]
    pub const fn uasr(&self) -> &Uasr {
        &self.uasr
    }
    #[doc = "0x30 - Region Descriptor Area Start Address Register"]
    #[inline(always)]
    pub const fn dscr(&self) -> &Dscr {
        &self.dscr
    }
    #[doc = "0x34 - Region Hash Area Start Address Register"]
    #[inline(always)]
    pub const fn hash(&self) -> &Hash {
        &self.hash
    }
    #[doc = "0x38..0x58 - User Initial Hash Value 0 Register 0"]
    #[inline(always)]
    pub const fn uihval(&self, n: usize) -> &Uihval {
        &self.uihval[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x58 - User Initial Hash Value 0 Register 0"]
    #[inline(always)]
    pub fn uihval_iter(&self) -> impl Iterator<Item = &Uihval> {
        self.uihval.iter()
    }
}
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "UASR (r) register accessor: Undefined Access Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uasr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uasr`] module"]
#[doc(alias = "UASR")]
pub type Uasr = crate::Reg<uasr::UasrSpec>;
#[doc = "Undefined Access Status Register"]
pub mod uasr;
#[doc = "DSCR (rw) register accessor: Region Descriptor Area Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`] module"]
#[doc(alias = "DSCR")]
pub type Dscr = crate::Reg<dscr::DscrSpec>;
#[doc = "Region Descriptor Area Start Address Register"]
pub mod dscr;
#[doc = "HASH (rw) register accessor: Region Hash Area Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hash::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash`] module"]
#[doc(alias = "HASH")]
pub type Hash = crate::Reg<hash::HashSpec>;
#[doc = "Region Hash Area Start Address Register"]
pub mod hash;
#[doc = "UIHVAL (w) register accessor: User Initial Hash Value 0 Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval`] module"]
#[doc(alias = "UIHVAL")]
pub type Uihval = crate::Reg<uihval::UihvalSpec>;
#[doc = "User Initial Hash Value 0 Register 0"]
pub mod uihval;
