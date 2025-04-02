#[repr(C)]
#[doc = "Channel Control Register (channel = 0)"]
#[doc(alias = "TC_CHANNEL")]
pub struct TcChannel {
    ccr: Ccr,
    _reserved_1_cmr: [u8; 0x04],
    smmr: Smmr,
    rab: Rab,
    cv: Cv,
    ra: Ra,
    rb: Rb,
    rc: Rc,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    emr: Emr,
}
impl TcChannel {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr_waveform_mode(&self) -> &CmrWaveformMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr_capture_mode(&self) -> &CmrCaptureMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn smmr(&self) -> &Smmr {
        &self.smmr
    }
    #[doc = "0x0c - Register AB (channel = 0)"]
    #[inline(always)]
    pub const fn rab(&self) -> &Rab {
        &self.rab
    }
    #[doc = "0x10 - Counter Value (channel = 0)"]
    #[inline(always)]
    pub const fn cv(&self) -> &Cv {
        &self.cv
    }
    #[doc = "0x14 - Register A (channel = 0)"]
    #[inline(always)]
    pub const fn ra(&self) -> &Ra {
        &self.ra
    }
    #[doc = "0x18 - Register B (channel = 0)"]
    #[inline(always)]
    pub const fn rb(&self) -> &Rb {
        &self.rb
    }
    #[doc = "0x1c - Register C (channel = 0)"]
    #[inline(always)]
    pub const fn rc(&self) -> &Rc {
        &self.rc
    }
    #[doc = "0x20 - Status Register (channel = 0)"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
}
#[doc = "CCR (w) register accessor: Channel Control Register (channel = 0)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr;
#[doc = "CMR_CAPTURE_MODE (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr_capture_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_capture_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr_capture_mode`] module"]
#[doc(alias = "CMR_CAPTURE_MODE")]
pub type CmrCaptureMode = crate::Reg<cmr_capture_mode::CmrCaptureModeSpec>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr_capture_mode;
#[doc = "CMR_WAVEFORM_MODE (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr_waveform_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_waveform_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr_waveform_mode`] module"]
#[doc(alias = "CMR_WAVEFORM_MODE")]
pub type CmrWaveformMode = crate::Reg<cmr_waveform_mode::CmrWaveformModeSpec>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr_waveform_mode;
#[doc = "SMMR (rw) register accessor: Stepper Motor Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr`] module"]
#[doc(alias = "SMMR")]
pub type Smmr = crate::Reg<smmr::SmmrSpec>;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr;
#[doc = "RAB (r) register accessor: Register AB (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rab::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rab`] module"]
#[doc(alias = "RAB")]
pub type Rab = crate::Reg<rab::RabSpec>;
#[doc = "Register AB (channel = 0)"]
pub mod rab;
#[doc = "CV (r) register accessor: Counter Value (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv`] module"]
#[doc(alias = "CV")]
pub type Cv = crate::Reg<cv::CvSpec>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv;
#[doc = "RA (rw) register accessor: Register A (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra`] module"]
#[doc(alias = "RA")]
pub type Ra = crate::Reg<ra::RaSpec>;
#[doc = "Register A (channel = 0)"]
pub mod ra;
#[doc = "RB (rw) register accessor: Register B (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb`] module"]
#[doc(alias = "RB")]
pub type Rb = crate::Reg<rb::RbSpec>;
#[doc = "Register B (channel = 0)"]
pub mod rb;
#[doc = "RC (rw) register accessor: Register C (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc`] module"]
#[doc(alias = "RC")]
pub type Rc = crate::Reg<rc::RcSpec>;
#[doc = "Register C (channel = 0)"]
pub mod rc;
#[doc = "SR (r) register accessor: Status Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register (channel = 0)"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register (channel = 0)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register (channel = 0)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr;
#[doc = "EMR (rw) register accessor: Extended Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`] module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "Extended Mode Register (channel = 0)"]
pub mod emr;
