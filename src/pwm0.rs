#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk: Clk,
    ena: Ena,
    dis: Dis,
    sr: Sr,
    ier1: Ier1,
    idr1: Idr1,
    imr1: Imr1,
    isr1: Isr1,
    scm: Scm,
    dmar: Dmar,
    scuc: Scuc,
    scup: Scup,
    scupupd: Scupupd,
    ier2: Ier2,
    idr2: Idr2,
    imr2: Imr2,
    isr2: Isr2,
    oov: Oov,
    os: Os,
    oss: Oss,
    osc: Osc,
    ossupd: Ossupd,
    oscupd: Oscupd,
    fmr: Fmr,
    fsr: Fsr,
    fcr: Fcr,
    fpv1: Fpv1,
    fpe: Fpe,
    _reserved28: [u8; 0x0c],
    elmr: [Elmr; 2],
    _reserved29: [u8; 0x1c],
    sspr: Sspr,
    sspup: Sspup,
    _reserved31: [u8; 0x08],
    smmr: Smmr,
    _reserved32: [u8; 0x0c],
    fpv2: Fpv2,
    _reserved33: [u8; 0x20],
    wpcr: Wpcr,
    wpsr: Wpsr,
    _reserved35: [u8; 0x44],
    pwm_cmp: [PwmCmp; 8],
    _reserved36: [u8; 0x50],
    pwm_ch_num: [PwmChNum; 4],
    _reserved37: [u8; 0x0180],
    cmupd0: Cmupd0,
    _reserved38: [u8; 0x1c],
    cmupd1: Cmupd1,
    _reserved39: [u8; 0x08],
    etrg1: Etrg1,
    lebr1: Lebr1,
    _reserved41: [u8; 0x0c],
    cmupd2: Cmupd2,
    _reserved42: [u8; 0x08],
    etrg2: Etrg2,
    lebr2: Lebr2,
    _reserved44: [u8; 0x0c],
    cmupd3: Cmupd3,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x04 - PWM Enable Register"]
    #[inline(always)]
    pub const fn ena(&self) -> &Ena {
        &self.ena
    }
    #[doc = "0x08 - PWM Disable Register"]
    #[inline(always)]
    pub const fn dis(&self) -> &Dis {
        &self.dis
    }
    #[doc = "0x0c - PWM Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn idr1(&self) -> &Idr1 {
        &self.idr1
    }
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn imr1(&self) -> &Imr1 {
        &self.imr1
    }
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn isr1(&self) -> &Isr1 {
        &self.isr1
    }
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    #[inline(always)]
    pub const fn scm(&self) -> &Scm {
        &self.scm
    }
    #[doc = "0x24 - PWM DMA Register"]
    #[inline(always)]
    pub const fn dmar(&self) -> &Dmar {
        &self.dmar
    }
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    #[inline(always)]
    pub const fn scuc(&self) -> &Scuc {
        &self.scuc
    }
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    #[inline(always)]
    pub const fn scup(&self) -> &Scup {
        &self.scup
    }
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    #[inline(always)]
    pub const fn scupupd(&self) -> &Scupupd {
        &self.scupupd
    }
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn ier2(&self) -> &Ier2 {
        &self.ier2
    }
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn idr2(&self) -> &Idr2 {
        &self.idr2
    }
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    #[inline(always)]
    pub const fn imr2(&self) -> &Imr2 {
        &self.imr2
    }
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn isr2(&self) -> &Isr2 {
        &self.isr2
    }
    #[doc = "0x44 - PWM Output Override Value Register"]
    #[inline(always)]
    pub const fn oov(&self) -> &Oov {
        &self.oov
    }
    #[doc = "0x48 - PWM Output Selection Register"]
    #[inline(always)]
    pub const fn os(&self) -> &Os {
        &self.os
    }
    #[doc = "0x4c - PWM Output Selection Set Register"]
    #[inline(always)]
    pub const fn oss(&self) -> &Oss {
        &self.oss
    }
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    #[inline(always)]
    pub const fn osc(&self) -> &Osc {
        &self.osc
    }
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    #[inline(always)]
    pub const fn ossupd(&self) -> &Ossupd {
        &self.ossupd
    }
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    #[inline(always)]
    pub const fn oscupd(&self) -> &Oscupd {
        &self.oscupd
    }
    #[doc = "0x5c - PWM Fault Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0x60 - PWM Fault Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x64 - PWM Fault Clear Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x68 - PWM Fault Protection Value Register 1"]
    #[inline(always)]
    pub const fn fpv1(&self) -> &Fpv1 {
        &self.fpv1
    }
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    #[inline(always)]
    pub const fn fpe(&self) -> &Fpe {
        &self.fpe
    }
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register 0"]
    #[inline(always)]
    pub const fn elmr(&self, n: usize) -> &Elmr {
        &self.elmr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register 0"]
    #[inline(always)]
    pub fn elmr_iter(&self) -> impl Iterator<Item = &Elmr> {
        self.elmr.iter()
    }
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    #[inline(always)]
    pub const fn sspr(&self) -> &Sspr {
        &self.sspr
    }
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    #[inline(always)]
    pub const fn sspup(&self) -> &Sspup {
        &self.sspup
    }
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    #[inline(always)]
    pub const fn smmr(&self) -> &Smmr {
        &self.smmr
    }
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    #[inline(always)]
    pub const fn fpv2(&self) -> &Fpv2 {
        &self.fpv2
    }
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    #[inline(always)]
    pub const fn wpcr(&self) -> &Wpcr {
        &self.wpcr
    }
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x130..0x1b0 - PWM Comparison 0 Value Register"]
    #[inline(always)]
    pub const fn pwm_cmp(&self, n: usize) -> &PwmCmp {
        &self.pwm_cmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x1b0 - PWM Comparison 0 Value Register"]
    #[inline(always)]
    pub fn pwm_cmp_iter(&self) -> impl Iterator<Item = &PwmCmp> {
        self.pwm_cmp.iter()
    }
    #[doc = "0x200..0x280 - PWM Channel Mode Register"]
    #[inline(always)]
    pub const fn pwm_ch_num(&self, n: usize) -> &PwmChNum {
        &self.pwm_ch_num[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x280 - PWM Channel Mode Register"]
    #[inline(always)]
    pub fn pwm_ch_num_iter(&self) -> impl Iterator<Item = &PwmChNum> {
        self.pwm_ch_num.iter()
    }
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cmupd0(&self) -> &Cmupd0 {
        &self.cmupd0
    }
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cmupd1(&self) -> &Cmupd1 {
        &self.cmupd1
    }
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    #[inline(always)]
    pub const fn etrg1(&self) -> &Etrg1 {
        &self.etrg1
    }
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    #[inline(always)]
    pub const fn lebr1(&self) -> &Lebr1 {
        &self.lebr1
    }
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cmupd2(&self) -> &Cmupd2 {
        &self.cmupd2
    }
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    #[inline(always)]
    pub const fn etrg2(&self) -> &Etrg2 {
        &self.etrg2
    }
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    #[inline(always)]
    pub const fn lebr2(&self) -> &Lebr2 {
        &self.lebr2
    }
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cmupd3(&self) -> &Cmupd3 {
        &self.cmupd3
    }
}
#[doc = "CLK (rw) register accessor: PWM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "ENA (w) register accessor: PWM Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ena::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ena`] module"]
#[doc(alias = "ENA")]
pub type Ena = crate::Reg<ena::EnaSpec>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS (w) register accessor: PWM Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`] module"]
#[doc(alias = "DIS")]
pub type Dis = crate::Reg<dis::DisSpec>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR (r) register accessor: PWM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER1 (w) register accessor: PWM Interrupt Enable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`] module"]
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::Ier1Spec>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: PWM Interrupt Disable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr1`] module"]
#[doc(alias = "IDR1")]
pub type Idr1 = crate::Reg<idr1::Idr1Spec>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: PWM Interrupt Mask Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`imr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`] module"]
#[doc(alias = "IMR1")]
pub type Imr1 = crate::Reg<imr1::Imr1Spec>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "ISR1 (r) register accessor: PWM Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`] module"]
#[doc(alias = "ISR1")]
pub type Isr1 = crate::Reg<isr1::Isr1Spec>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "SCM (rw) register accessor: PWM Sync Channels Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm`] module"]
#[doc(alias = "SCM")]
pub type Scm = crate::Reg<scm::ScmSpec>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "DMAR (w) register accessor: PWM DMA Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`] module"]
#[doc(alias = "DMAR")]
pub type Dmar = crate::Reg<dmar::DmarSpec>;
#[doc = "PWM DMA Register"]
pub mod dmar;
#[doc = "SCUC (rw) register accessor: PWM Sync Channels Update Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc`] module"]
#[doc(alias = "SCUC")]
pub type Scuc = crate::Reg<scuc::ScucSpec>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "SCUP (rw) register accessor: PWM Sync Channels Update Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scup`] module"]
#[doc(alias = "SCUP")]
pub type Scup = crate::Reg<scup::ScupSpec>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "SCUPUPD (w) register accessor: PWM Sync Channels Update Period Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scupupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scupupd`] module"]
#[doc(alias = "SCUPUPD")]
pub type Scupupd = crate::Reg<scupupd::ScupupdSpec>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "IER2 (w) register accessor: PWM Interrupt Enable Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`] module"]
#[doc(alias = "IER2")]
pub type Ier2 = crate::Reg<ier2::Ier2Spec>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: PWM Interrupt Disable Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`] module"]
#[doc(alias = "IDR2")]
pub type Idr2 = crate::Reg<idr2::Idr2Spec>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: PWM Interrupt Mask Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`] module"]
#[doc(alias = "IMR2")]
pub type Imr2 = crate::Reg<imr2::Imr2Spec>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "ISR2 (r) register accessor: PWM Interrupt Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`isr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr2`] module"]
#[doc(alias = "ISR2")]
pub type Isr2 = crate::Reg<isr2::Isr2Spec>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "OOV (rw) register accessor: PWM Output Override Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oov::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oov::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oov`] module"]
#[doc(alias = "OOV")]
pub type Oov = crate::Reg<oov::OovSpec>;
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "OS (rw) register accessor: PWM Output Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`os::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`os::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os`] module"]
#[doc(alias = "OS")]
pub type Os = crate::Reg<os::OsSpec>;
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "OSS (w) register accessor: PWM Output Selection Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oss`] module"]
#[doc(alias = "OSS")]
pub type Oss = crate::Reg<oss::OssSpec>;
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "OSC (w) register accessor: PWM Output Selection Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc`] module"]
#[doc(alias = "OSC")]
pub type Osc = crate::Reg<osc::OscSpec>;
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "OSSUPD (w) register accessor: PWM Output Selection Set Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ossupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ossupd`] module"]
#[doc(alias = "OSSUPD")]
pub type Ossupd = crate::Reg<ossupd::OssupdSpec>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "OSCUPD (w) register accessor: PWM Output Selection Clear Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscupd`] module"]
#[doc(alias = "OSCUPD")]
pub type Oscupd = crate::Reg<oscupd::OscupdSpec>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "FMR (rw) register accessor: PWM Fault Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`] module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "FSR (r) register accessor: PWM Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`] module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "FCR (w) register accessor: PWM Fault Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "FPV1 (rw) register accessor: PWM Fault Protection Value Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpv1`] module"]
#[doc(alias = "FPV1")]
pub type Fpv1 = crate::Reg<fpv1::Fpv1Spec>;
#[doc = "PWM Fault Protection Value Register 1"]
pub mod fpv1;
#[doc = "FPE (rw) register accessor: PWM Fault Protection Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpe`] module"]
#[doc(alias = "FPE")]
pub type Fpe = crate::Reg<fpe::FpeSpec>;
#[doc = "PWM Fault Protection Enable Register"]
pub mod fpe;
#[doc = "ELMR (rw) register accessor: PWM Event Line 0 Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`elmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elmr`] module"]
#[doc(alias = "ELMR")]
pub type Elmr = crate::Reg<elmr::ElmrSpec>;
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod elmr;
#[doc = "SSPR (rw) register accessor: PWM Spread Spectrum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspr`] module"]
#[doc(alias = "SSPR")]
pub type Sspr = crate::Reg<sspr::SsprSpec>;
#[doc = "PWM Spread Spectrum Register"]
pub mod sspr;
#[doc = "SSPUP (w) register accessor: PWM Spread Spectrum Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspup`] module"]
#[doc(alias = "SSPUP")]
pub type Sspup = crate::Reg<sspup::SspupSpec>;
#[doc = "PWM Spread Spectrum Update Register"]
pub mod sspup;
#[doc = "SMMR (rw) register accessor: PWM Stepper Motor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr`] module"]
#[doc(alias = "SMMR")]
pub type Smmr = crate::Reg<smmr::SmmrSpec>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "FPV2 (rw) register accessor: PWM Fault Protection Value 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpv2`] module"]
#[doc(alias = "FPV2")]
pub type Fpv2 = crate::Reg<fpv2::Fpv2Spec>;
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod fpv2;
#[doc = "WPCR (w) register accessor: PWM Write Protection Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr`] module"]
#[doc(alias = "WPCR")]
pub type Wpcr = crate::Reg<wpcr::WpcrSpec>;
#[doc = "PWM Write Protection Control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: PWM Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "PWM Write Protection Status Register"]
pub mod wpsr;
#[doc = "PWM Comparison 0 Value Register"]
pub use self::pwm_cmp::PwmCmp;
#[doc = r"Cluster"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = "PWM Channel Mode Register"]
pub use self::pwm_ch_num::PwmChNum;
#[doc = r"Cluster"]
#[doc = "PWM Channel Mode Register"]
pub mod pwm_ch_num;
#[doc = "CMUPD0 (w) register accessor: PWM Channel Mode Update Register (ch_num = 0)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmupd0`] module"]
#[doc(alias = "CMUPD0")]
pub type Cmupd0 = crate::Reg<cmupd0::Cmupd0Spec>;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod cmupd0;
#[doc = "CMUPD1 (w) register accessor: PWM Channel Mode Update Register (ch_num = 1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmupd1`] module"]
#[doc(alias = "CMUPD1")]
pub type Cmupd1 = crate::Reg<cmupd1::Cmupd1Spec>;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod cmupd1;
#[doc = "ETRG1 (rw) register accessor: PWM External Trigger Register (trg_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`etrg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etrg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etrg1`] module"]
#[doc(alias = "ETRG1")]
pub type Etrg1 = crate::Reg<etrg1::Etrg1Spec>;
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod etrg1;
#[doc = "LEBR1 (rw) register accessor: PWM Leading-Edge Blanking Register (trg_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`lebr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lebr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lebr1`] module"]
#[doc(alias = "LEBR1")]
pub type Lebr1 = crate::Reg<lebr1::Lebr1Spec>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod lebr1;
#[doc = "CMUPD2 (w) register accessor: PWM Channel Mode Update Register (ch_num = 2)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmupd2`] module"]
#[doc(alias = "CMUPD2")]
pub type Cmupd2 = crate::Reg<cmupd2::Cmupd2Spec>;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod cmupd2;
#[doc = "ETRG2 (rw) register accessor: PWM External Trigger Register (trg_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`etrg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etrg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etrg2`] module"]
#[doc(alias = "ETRG2")]
pub type Etrg2 = crate::Reg<etrg2::Etrg2Spec>;
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod etrg2;
#[doc = "LEBR2 (rw) register accessor: PWM Leading-Edge Blanking Register (trg_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`lebr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lebr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lebr2`] module"]
#[doc(alias = "LEBR2")]
pub type Lebr2 = crate::Reg<lebr2::Lebr2Spec>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod lebr2;
#[doc = "CMUPD3 (w) register accessor: PWM Channel Mode Update Register (ch_num = 3)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmupd3`] module"]
#[doc(alias = "CMUPD3")]
pub type Cmupd3 = crate::Reg<cmupd3::Cmupd3Spec>;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod cmupd3;
