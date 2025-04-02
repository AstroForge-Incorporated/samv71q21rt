#[repr(C)]
#[doc = "PWM Channel Mode Register"]
#[doc(alias = "PWM_CH_NUM")]
pub struct PwmChNum {
    cmr: Cmr,
    cdty: Cdty,
    cdtyupd: Cdtyupd,
    cprd: Cprd,
    cprdupd: Cprdupd,
    ccnt: Ccnt,
    dt: Dt,
    dtupd: Dtupd,
}
impl PwmChNum {
    #[doc = "0x00 - PWM Channel Mode Register"]
    #[inline(always)]
    pub const fn cmr(&self) -> &Cmr {
        &self.cmr
    }
    #[doc = "0x04 - PWM Channel Duty Cycle Register"]
    #[inline(always)]
    pub const fn cdty(&self) -> &Cdty {
        &self.cdty
    }
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register"]
    #[inline(always)]
    pub const fn cdtyupd(&self) -> &Cdtyupd {
        &self.cdtyupd
    }
    #[doc = "0x0c - PWM Channel Period Register"]
    #[inline(always)]
    pub const fn cprd(&self) -> &Cprd {
        &self.cprd
    }
    #[doc = "0x10 - PWM Channel Period Update Register"]
    #[inline(always)]
    pub const fn cprdupd(&self) -> &Cprdupd {
        &self.cprdupd
    }
    #[doc = "0x14 - PWM Channel Counter Register"]
    #[inline(always)]
    pub const fn ccnt(&self) -> &Ccnt {
        &self.ccnt
    }
    #[doc = "0x18 - PWM Channel Dead Time Register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x1c - PWM Channel Dead Time Update Register"]
    #[inline(always)]
    pub const fn dtupd(&self) -> &Dtupd {
        &self.dtupd
    }
}
#[doc = "CMR (rw) register accessor: PWM Channel Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`] module"]
#[doc(alias = "CMR")]
pub type Cmr = crate::Reg<cmr::CmrSpec>;
#[doc = "PWM Channel Mode Register"]
pub mod cmr;
#[doc = "CDTY (rw) register accessor: PWM Channel Duty Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty`] module"]
#[doc(alias = "CDTY")]
pub type Cdty = crate::Reg<cdty::CdtySpec>;
#[doc = "PWM Channel Duty Cycle Register"]
pub mod cdty;
#[doc = "CDTYUPD (w) register accessor: PWM Channel Duty Cycle Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdtyupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdtyupd`] module"]
#[doc(alias = "CDTYUPD")]
pub type Cdtyupd = crate::Reg<cdtyupd::CdtyupdSpec>;
#[doc = "PWM Channel Duty Cycle Update Register"]
pub mod cdtyupd;
#[doc = "CPRD (rw) register accessor: PWM Channel Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd`] module"]
#[doc(alias = "CPRD")]
pub type Cprd = crate::Reg<cprd::CprdSpec>;
#[doc = "PWM Channel Period Register"]
pub mod cprd;
#[doc = "CPRDUPD (w) register accessor: PWM Channel Period Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cprdupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprdupd`] module"]
#[doc(alias = "CPRDUPD")]
pub type Cprdupd = crate::Reg<cprdupd::CprdupdSpec>;
#[doc = "PWM Channel Period Update Register"]
pub mod cprdupd;
#[doc = "CCNT (r) register accessor: PWM Channel Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt`] module"]
#[doc(alias = "CCNT")]
pub type Ccnt = crate::Reg<ccnt::CcntSpec>;
#[doc = "PWM Channel Counter Register"]
pub mod ccnt;
#[doc = "DT (rw) register accessor: PWM Channel Dead Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`] module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "PWM Channel Dead Time Register"]
pub mod dt;
#[doc = "DTUPD (w) register accessor: PWM Channel Dead Time Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtupd`] module"]
#[doc(alias = "DTUPD")]
pub type Dtupd = crate::Reg<dtupd::DtupdSpec>;
#[doc = "PWM Channel Dead Time Update Register"]
pub mod dtupd;
