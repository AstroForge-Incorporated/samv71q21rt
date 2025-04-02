#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mmr: Mmr,
    smr: Smr,
    iadr: Iadr,
    cwgr: Cwgr,
    _reserved5: [u8; 0x0c],
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    rhr: Rhr,
    thr: Thr,
    smbtr: Smbtr,
    _reserved12: [u8; 0x08],
    filtr: Filtr,
    _reserved13: [u8; 0x04],
    swmr: Swmr,
    _reserved14: [u8; 0x94],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Master Mode Register"]
    #[inline(always)]
    pub const fn mmr(&self) -> &Mmr {
        &self.mmr
    }
    #[doc = "0x08 - Slave Mode Register"]
    #[inline(always)]
    pub const fn smr(&self) -> &Smr {
        &self.smr
    }
    #[doc = "0x0c - Internal Address Register"]
    #[inline(always)]
    pub const fn iadr(&self) -> &Iadr {
        &self.iadr
    }
    #[doc = "0x10 - Clock Waveform Generator Register"]
    #[inline(always)]
    pub const fn cwgr(&self) -> &Cwgr {
        &self.cwgr
    }
    #[doc = "0x20 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x24 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x28 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x2c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - Receive Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &Rhr {
        &self.rhr
    }
    #[doc = "0x34 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        &self.thr
    }
    #[doc = "0x38 - SMBus Timing Register"]
    #[inline(always)]
    pub const fn smbtr(&self) -> &Smbtr {
        &self.smbtr
    }
    #[doc = "0x44 - Filter Register"]
    #[inline(always)]
    pub const fn filtr(&self) -> &Filtr {
        &self.filtr
    }
    #[doc = "0x4c - SleepWalking Matching Register"]
    #[inline(always)]
    pub const fn swmr(&self) -> &Swmr {
        &self.swmr
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MMR (rw) register accessor: Master Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr`] module"]
#[doc(alias = "MMR")]
pub type Mmr = crate::Reg<mmr::MmrSpec>;
#[doc = "Master Mode Register"]
pub mod mmr;
#[doc = "SMR (rw) register accessor: Slave Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr`] module"]
#[doc(alias = "SMR")]
pub type Smr = crate::Reg<smr::SmrSpec>;
#[doc = "Slave Mode Register"]
pub mod smr;
#[doc = "IADR (rw) register accessor: Internal Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadr`] module"]
#[doc(alias = "IADR")]
pub type Iadr = crate::Reg<iadr::IadrSpec>;
#[doc = "Internal Address Register"]
pub mod iadr;
#[doc = "CWGR (rw) register accessor: Clock Waveform Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwgr`] module"]
#[doc(alias = "CWGR")]
pub type Cwgr = crate::Reg<cwgr::CwgrSpec>;
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
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
#[doc = "RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`] module"]
#[doc(alias = "RHR")]
pub type Rhr = crate::Reg<rhr::RhrSpec>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "SMBTR (rw) register accessor: SMBus Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smbtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smbtr`] module"]
#[doc(alias = "SMBTR")]
pub type Smbtr = crate::Reg<smbtr::SmbtrSpec>;
#[doc = "SMBus Timing Register"]
pub mod smbtr;
#[doc = "FILTR (rw) register accessor: Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`filtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filtr`] module"]
#[doc(alias = "FILTR")]
pub type Filtr = crate::Reg<filtr::FiltrSpec>;
#[doc = "Filter Register"]
pub mod filtr;
#[doc = "SWMR (rw) register accessor: SleepWalking Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swmr`] module"]
#[doc(alias = "SWMR")]
pub type Swmr = crate::Reg<swmr::SwmrSpec>;
#[doc = "SleepWalking Matching Register"]
pub mod swmr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
