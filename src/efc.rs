#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eefc_fmr: EefcFmr,
    eefc_fcr: EefcFcr,
    eefc_fsr: EefcFsr,
    eefc_frr: EefcFrr,
    _reserved4: [u8; 0xd4],
    eefc_wpmr: EefcWpmr,
}
impl RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    #[inline(always)]
    pub const fn eefc_fmr(&self) -> &EefcFmr {
        &self.eefc_fmr
    }
    #[doc = "0x04 - EEFC Flash Command Register"]
    #[inline(always)]
    pub const fn eefc_fcr(&self) -> &EefcFcr {
        &self.eefc_fcr
    }
    #[doc = "0x08 - EEFC Flash Status Register"]
    #[inline(always)]
    pub const fn eefc_fsr(&self) -> &EefcFsr {
        &self.eefc_fsr
    }
    #[doc = "0x0c - EEFC Flash Result Register"]
    #[inline(always)]
    pub const fn eefc_frr(&self) -> &EefcFrr {
        &self.eefc_frr
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn eefc_wpmr(&self) -> &EefcWpmr {
        &self.eefc_wpmr
    }
}
#[doc = "EEFC_FMR (rw) register accessor: EEFC Flash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefc_fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefc_fmr`] module"]
#[doc(alias = "EEFC_FMR")]
pub type EefcFmr = crate::Reg<eefc_fmr::EefcFmrSpec>;
#[doc = "EEFC Flash Mode Register"]
pub mod eefc_fmr;
#[doc = "EEFC_FCR (w) register accessor: EEFC Flash Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefc_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefc_fcr`] module"]
#[doc(alias = "EEFC_FCR")]
pub type EefcFcr = crate::Reg<eefc_fcr::EefcFcrSpec>;
#[doc = "EEFC Flash Command Register"]
pub mod eefc_fcr;
#[doc = "EEFC_FSR (r) register accessor: EEFC Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefc_fsr`] module"]
#[doc(alias = "EEFC_FSR")]
pub type EefcFsr = crate::Reg<eefc_fsr::EefcFsrSpec>;
#[doc = "EEFC Flash Status Register"]
pub mod eefc_fsr;
#[doc = "EEFC_FRR (r) register accessor: EEFC Flash Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_frr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefc_frr`] module"]
#[doc(alias = "EEFC_FRR")]
pub type EefcFrr = crate::Reg<eefc_frr::EefcFrrSpec>;
#[doc = "EEFC Flash Result Register"]
pub mod eefc_frr;
#[doc = "EEFC_WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefc_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefc_wpmr`] module"]
#[doc(alias = "EEFC_WPMR")]
pub type EefcWpmr = crate::Reg<eefc_wpmr::EefcWpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod eefc_wpmr;
