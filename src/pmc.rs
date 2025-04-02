#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scer: Scer,
    scdr: Scdr,
    scsr: Scsr,
    _reserved3: [u8; 0x04],
    pcer0: Pcer0,
    pcdr0: Pcdr0,
    pcsr0: Pcsr0,
    ckgr_uckr: CkgrUckr,
    ckgr_mor: CkgrMor,
    ckgr_mcfr: CkgrMcfr,
    ckgr_pllar: CkgrPllar,
    _reserved10: [u8; 0x04],
    mckr: Mckr,
    _reserved11: [u8; 0x04],
    usb: Usb,
    _reserved12: [u8; 0x04],
    pck: [Pck; 8],
    ier: Ier,
    idr: Idr,
    sr: Sr,
    imr: Imr,
    fsmr: Fsmr,
    fspr: Fspr,
    focr: Focr,
    _reserved20: [u8; 0x68],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved22: [u8; 0x14],
    pcer1: Pcer1,
    pcdr1: Pcdr1,
    pcsr1: Pcsr1,
    pcr: Pcr,
    ocr: Ocr,
    slpwk_er0: SlpwkEr0,
    slpwk_dr0: SlpwkDr0,
    slpwk_sr0: SlpwkSr0,
    slpwk_asr0: SlpwkAsr0,
    _reserved31: [u8; 0x0c],
    pmmr: Pmmr,
    slpwk_er1: SlpwkEr1,
    slpwk_dr1: SlpwkDr1,
    slpwk_sr1: SlpwkSr1,
    slpwk_asr1: SlpwkAsr1,
    slpwk_aipr: SlpwkAipr,
}
impl RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    #[inline(always)]
    pub const fn scer(&self) -> &Scer {
        &self.scer
    }
    #[doc = "0x04 - System Clock Disable Register"]
    #[inline(always)]
    pub const fn scdr(&self) -> &Scdr {
        &self.scdr
    }
    #[doc = "0x08 - System Clock Status Register"]
    #[inline(always)]
    pub const fn scsr(&self) -> &Scsr {
        &self.scsr
    }
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn pcer0(&self) -> &Pcer0 {
        &self.pcer0
    }
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    #[inline(always)]
    pub const fn pcdr0(&self) -> &Pcdr0 {
        &self.pcdr0
    }
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    #[inline(always)]
    pub const fn pcsr0(&self) -> &Pcsr0 {
        &self.pcsr0
    }
    #[doc = "0x1c - UTMI Clock Register"]
    #[inline(always)]
    pub const fn ckgr_uckr(&self) -> &CkgrUckr {
        &self.ckgr_uckr
    }
    #[doc = "0x20 - Main Oscillator Register"]
    #[inline(always)]
    pub const fn ckgr_mor(&self) -> &CkgrMor {
        &self.ckgr_mor
    }
    #[doc = "0x24 - Main Clock Frequency Register"]
    #[inline(always)]
    pub const fn ckgr_mcfr(&self) -> &CkgrMcfr {
        &self.ckgr_mcfr
    }
    #[doc = "0x28 - PLLA Register"]
    #[inline(always)]
    pub const fn ckgr_pllar(&self) -> &CkgrPllar {
        &self.ckgr_pllar
    }
    #[doc = "0x30 - Master Clock Register"]
    #[inline(always)]
    pub const fn mckr(&self) -> &Mckr {
        &self.mckr
    }
    #[doc = "0x38 - USB Clock Register"]
    #[inline(always)]
    pub const fn usb(&self) -> &Usb {
        &self.usb
    }
    #[doc = "0x40..0x60 - Programmable Clock Register"]
    #[inline(always)]
    pub const fn pck(&self, n: usize) -> &Pck {
        &self.pck[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Programmable Clock Register"]
    #[inline(always)]
    pub fn pck_iter(&self) -> impl Iterator<Item = &Pck> {
        self.pck.iter()
    }
    #[doc = "0x60 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x64 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x68 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x6c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x70 - Fast Startup Mode Register"]
    #[inline(always)]
    pub const fn fsmr(&self) -> &Fsmr {
        &self.fsmr
    }
    #[doc = "0x74 - Fast Startup Polarity Register"]
    #[inline(always)]
    pub const fn fspr(&self) -> &Fspr {
        &self.fspr
    }
    #[doc = "0x78 - Fault Output Clear Register"]
    #[inline(always)]
    pub const fn focr(&self) -> &Focr {
        &self.focr
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
    #[doc = "0x100 - Peripheral Clock Enable Register 1"]
    #[inline(always)]
    pub const fn pcer1(&self) -> &Pcer1 {
        &self.pcer1
    }
    #[doc = "0x104 - Peripheral Clock Disable Register 1"]
    #[inline(always)]
    pub const fn pcdr1(&self) -> &Pcdr1 {
        &self.pcdr1
    }
    #[doc = "0x108 - Peripheral Clock Status Register 1"]
    #[inline(always)]
    pub const fn pcsr1(&self) -> &Pcsr1 {
        &self.pcsr1
    }
    #[doc = "0x10c - Peripheral Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
    }
    #[doc = "0x110 - Oscillator Calibration Register"]
    #[inline(always)]
    pub const fn ocr(&self) -> &Ocr {
        &self.ocr
    }
    #[doc = "0x114 - SleepWalking Enable Register 0"]
    #[inline(always)]
    pub const fn slpwk_er0(&self) -> &SlpwkEr0 {
        &self.slpwk_er0
    }
    #[doc = "0x118 - SleepWalking Disable Register 0"]
    #[inline(always)]
    pub const fn slpwk_dr0(&self) -> &SlpwkDr0 {
        &self.slpwk_dr0
    }
    #[doc = "0x11c - SleepWalking Status Register 0"]
    #[inline(always)]
    pub const fn slpwk_sr0(&self) -> &SlpwkSr0 {
        &self.slpwk_sr0
    }
    #[doc = "0x120 - SleepWalking Activity Status Register 0"]
    #[inline(always)]
    pub const fn slpwk_asr0(&self) -> &SlpwkAsr0 {
        &self.slpwk_asr0
    }
    #[doc = "0x130 - PLL Maximum Multiplier Value Register"]
    #[inline(always)]
    pub const fn pmmr(&self) -> &Pmmr {
        &self.pmmr
    }
    #[doc = "0x134 - SleepWalking Enable Register 1"]
    #[inline(always)]
    pub const fn slpwk_er1(&self) -> &SlpwkEr1 {
        &self.slpwk_er1
    }
    #[doc = "0x138 - SleepWalking Disable Register 1"]
    #[inline(always)]
    pub const fn slpwk_dr1(&self) -> &SlpwkDr1 {
        &self.slpwk_dr1
    }
    #[doc = "0x13c - SleepWalking Status Register 1"]
    #[inline(always)]
    pub const fn slpwk_sr1(&self) -> &SlpwkSr1 {
        &self.slpwk_sr1
    }
    #[doc = "0x140 - SleepWalking Activity Status Register 1"]
    #[inline(always)]
    pub const fn slpwk_asr1(&self) -> &SlpwkAsr1 {
        &self.slpwk_asr1
    }
    #[doc = "0x144 - SleepWalking Activity In Progress Register"]
    #[inline(always)]
    pub const fn slpwk_aipr(&self) -> &SlpwkAipr {
        &self.slpwk_aipr
    }
}
#[doc = "SCER (w) register accessor: System Clock Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scer`] module"]
#[doc(alias = "SCER")]
pub type Scer = crate::Reg<scer::ScerSpec>;
#[doc = "System Clock Enable Register"]
pub mod scer;
#[doc = "SCDR (w) register accessor: System Clock Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scdr`] module"]
#[doc(alias = "SCDR")]
pub type Scdr = crate::Reg<scdr::ScdrSpec>;
#[doc = "System Clock Disable Register"]
pub mod scdr;
#[doc = "SCSR (r) register accessor: System Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`] module"]
#[doc(alias = "SCSR")]
pub type Scsr = crate::Reg<scsr::ScsrSpec>;
#[doc = "System Clock Status Register"]
pub mod scsr;
#[doc = "PCER0 (w) register accessor: Peripheral Clock Enable Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcer0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcer0`] module"]
#[doc(alias = "PCER0")]
pub type Pcer0 = crate::Reg<pcer0::Pcer0Spec>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pcer0;
#[doc = "PCDR0 (w) register accessor: Peripheral Clock Disable Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdr0`] module"]
#[doc(alias = "PCDR0")]
pub type Pcdr0 = crate::Reg<pcdr0::Pcdr0Spec>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pcdr0;
#[doc = "PCSR0 (r) register accessor: Peripheral Clock Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsr0`] module"]
#[doc(alias = "PCSR0")]
pub type Pcsr0 = crate::Reg<pcsr0::Pcsr0Spec>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pcsr0;
#[doc = "CKGR_UCKR (rw) register accessor: UTMI Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_uckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_uckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_uckr`] module"]
#[doc(alias = "CKGR_UCKR")]
pub type CkgrUckr = crate::Reg<ckgr_uckr::CkgrUckrSpec>;
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "CKGR_MOR (rw) register accessor: Main Oscillator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_mor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mor`] module"]
#[doc(alias = "CKGR_MOR")]
pub type CkgrMor = crate::Reg<ckgr_mor::CkgrMorSpec>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (rw) register accessor: Main Clock Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_mcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mcfr`] module"]
#[doc(alias = "CKGR_MCFR")]
pub type CkgrMcfr = crate::Reg<ckgr_mcfr::CkgrMcfrSpec>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: PLLA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_pllar`] module"]
#[doc(alias = "CKGR_PLLAR")]
pub type CkgrPllar = crate::Reg<ckgr_pllar::CkgrPllarSpec>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "MCKR (rw) register accessor: Master Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mckr`] module"]
#[doc(alias = "MCKR")]
pub type Mckr = crate::Reg<mckr::MckrSpec>;
#[doc = "Master Clock Register"]
pub mod mckr;
#[doc = "USB (rw) register accessor: USB Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb`] module"]
#[doc(alias = "USB")]
pub type Usb = crate::Reg<usb::UsbSpec>;
#[doc = "USB Clock Register"]
pub mod usb;
#[doc = "PCK (rw) register accessor: Programmable Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pck`] module"]
#[doc(alias = "PCK")]
pub type Pck = crate::Reg<pck::PckSpec>;
#[doc = "Programmable Clock Register"]
pub mod pck;
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
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "FSMR (rw) register accessor: Fast Startup Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmr`] module"]
#[doc(alias = "FSMR")]
pub type Fsmr = crate::Reg<fsmr::FsmrSpec>;
#[doc = "Fast Startup Mode Register"]
pub mod fsmr;
#[doc = "FSPR (rw) register accessor: Fast Startup Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fspr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fspr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fspr`] module"]
#[doc(alias = "FSPR")]
pub type Fspr = crate::Reg<fspr::FsprSpec>;
#[doc = "Fast Startup Polarity Register"]
pub mod fspr;
#[doc = "FOCR (w) register accessor: Fault Output Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`focr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@focr`] module"]
#[doc(alias = "FOCR")]
pub type Focr = crate::Reg<focr::FocrSpec>;
#[doc = "Fault Output Clear Register"]
pub mod focr;
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
#[doc = "PCER1 (w) register accessor: Peripheral Clock Enable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcer1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcer1`] module"]
#[doc(alias = "PCER1")]
pub type Pcer1 = crate::Reg<pcer1::Pcer1Spec>;
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pcer1;
#[doc = "PCDR1 (w) register accessor: Peripheral Clock Disable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdr1`] module"]
#[doc(alias = "PCDR1")]
pub type Pcdr1 = crate::Reg<pcdr1::Pcdr1Spec>;
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pcdr1;
#[doc = "PCSR1 (r) register accessor: Peripheral Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsr1`] module"]
#[doc(alias = "PCSR1")]
pub type Pcsr1 = crate::Reg<pcsr1::Pcsr1Spec>;
#[doc = "Peripheral Clock Status Register 1"]
pub mod pcsr1;
#[doc = "PCR (rw) register accessor: Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`] module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "Peripheral Control Register"]
pub mod pcr;
#[doc = "OCR (rw) register accessor: Oscillator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr`] module"]
#[doc(alias = "OCR")]
pub type Ocr = crate::Reg<ocr::OcrSpec>;
#[doc = "Oscillator Calibration Register"]
pub mod ocr;
#[doc = "SLPWK_ER0 (w) register accessor: SleepWalking Enable Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slpwk_er0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_er0`] module"]
#[doc(alias = "SLPWK_ER0")]
pub type SlpwkEr0 = crate::Reg<slpwk_er0::SlpwkEr0Spec>;
#[doc = "SleepWalking Enable Register 0"]
pub mod slpwk_er0;
#[doc = "SLPWK_DR0 (w) register accessor: SleepWalking Disable Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slpwk_dr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_dr0`] module"]
#[doc(alias = "SLPWK_DR0")]
pub type SlpwkDr0 = crate::Reg<slpwk_dr0::SlpwkDr0Spec>;
#[doc = "SleepWalking Disable Register 0"]
pub mod slpwk_dr0;
#[doc = "SLPWK_SR0 (r) register accessor: SleepWalking Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_sr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_sr0`] module"]
#[doc(alias = "SLPWK_SR0")]
pub type SlpwkSr0 = crate::Reg<slpwk_sr0::SlpwkSr0Spec>;
#[doc = "SleepWalking Status Register 0"]
pub mod slpwk_sr0;
#[doc = "SLPWK_ASR0 (r) register accessor: SleepWalking Activity Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_asr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_asr0`] module"]
#[doc(alias = "SLPWK_ASR0")]
pub type SlpwkAsr0 = crate::Reg<slpwk_asr0::SlpwkAsr0Spec>;
#[doc = "SleepWalking Activity Status Register 0"]
pub mod slpwk_asr0;
#[doc = "PMMR (rw) register accessor: PLL Maximum Multiplier Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmr`] module"]
#[doc(alias = "PMMR")]
pub type Pmmr = crate::Reg<pmmr::PmmrSpec>;
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmmr;
#[doc = "SLPWK_ER1 (w) register accessor: SleepWalking Enable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slpwk_er1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_er1`] module"]
#[doc(alias = "SLPWK_ER1")]
pub type SlpwkEr1 = crate::Reg<slpwk_er1::SlpwkEr1Spec>;
#[doc = "SleepWalking Enable Register 1"]
pub mod slpwk_er1;
#[doc = "SLPWK_DR1 (w) register accessor: SleepWalking Disable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slpwk_dr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_dr1`] module"]
#[doc(alias = "SLPWK_DR1")]
pub type SlpwkDr1 = crate::Reg<slpwk_dr1::SlpwkDr1Spec>;
#[doc = "SleepWalking Disable Register 1"]
pub mod slpwk_dr1;
#[doc = "SLPWK_SR1 (r) register accessor: SleepWalking Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_sr1`] module"]
#[doc(alias = "SLPWK_SR1")]
pub type SlpwkSr1 = crate::Reg<slpwk_sr1::SlpwkSr1Spec>;
#[doc = "SleepWalking Status Register 1"]
pub mod slpwk_sr1;
#[doc = "SLPWK_ASR1 (r) register accessor: SleepWalking Activity Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_asr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_asr1`] module"]
#[doc(alias = "SLPWK_ASR1")]
pub type SlpwkAsr1 = crate::Reg<slpwk_asr1::SlpwkAsr1Spec>;
#[doc = "SleepWalking Activity Status Register 1"]
pub mod slpwk_asr1;
#[doc = "SLPWK_AIPR (r) register accessor: SleepWalking Activity In Progress Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_aipr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpwk_aipr`] module"]
#[doc(alias = "SLPWK_AIPR")]
pub type SlpwkAipr = crate::Reg<slpwk_aipr::SlpwkAiprSpec>;
#[doc = "SleepWalking Activity In Progress Register"]
pub mod slpwk_aipr;
