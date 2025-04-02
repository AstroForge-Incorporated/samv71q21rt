#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    tr: Tr,
    cr: Cr,
    _reserved3: [u8; 0x04],
    lpr: Lpr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    mdr: Mdr,
    cfr1: Cfr1,
    ocms: Ocms,
    ocms_key1: OcmsKey1,
    ocms_key2: OcmsKey2,
}
impl RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    #[inline(always)]
    pub const fn tr(&self) -> &Tr {
        &self.tr
    }
    #[doc = "0x08 - SDRAMC Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10 - SDRAMC Low Power Register"]
    #[inline(always)]
    pub const fn lpr(&self) -> &Lpr {
        &self.lpr
    }
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    #[inline(always)]
    pub const fn mdr(&self) -> &Mdr {
        &self.mdr
    }
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    #[inline(always)]
    pub const fn cfr1(&self) -> &Cfr1 {
        &self.cfr1
    }
    #[doc = "0x2c - SDRAMC OCMS Register"]
    #[inline(always)]
    pub const fn ocms(&self) -> &Ocms {
        &self.ocms
    }
    #[doc = "0x30 - SDRAMC OCMS KEY1 Register"]
    #[inline(always)]
    pub const fn ocms_key1(&self) -> &OcmsKey1 {
        &self.ocms_key1
    }
    #[doc = "0x34 - SDRAMC OCMS KEY2 Register"]
    #[inline(always)]
    pub const fn ocms_key2(&self) -> &OcmsKey2 {
        &self.ocms_key2
    }
}
#[doc = "MR (rw) register accessor: SDRAMC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "SDRAMC Mode Register"]
pub mod mr;
#[doc = "TR (rw) register accessor: SDRAMC Refresh Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`] module"]
#[doc(alias = "TR")]
pub type Tr = crate::Reg<tr::TrSpec>;
#[doc = "SDRAMC Refresh Timer Register"]
pub mod tr;
#[doc = "CR (rw) register accessor: SDRAMC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "SDRAMC Configuration Register"]
pub mod cr;
#[doc = "LPR (rw) register accessor: SDRAMC Low Power Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpr`] module"]
#[doc(alias = "LPR")]
pub type Lpr = crate::Reg<lpr::LprSpec>;
#[doc = "SDRAMC Low Power Register"]
pub mod lpr;
#[doc = "IER (w) register accessor: SDRAMC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: SDRAMC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: SDRAMC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: SDRAMC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "SDRAMC Interrupt Status Register"]
pub mod isr;
#[doc = "MDR (rw) register accessor: SDRAMC Memory Device Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdr`] module"]
#[doc(alias = "MDR")]
pub type Mdr = crate::Reg<mdr::MdrSpec>;
#[doc = "SDRAMC Memory Device Register"]
pub mod mdr;
#[doc = "CFR1 (rw) register accessor: SDRAMC Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr1`] module"]
#[doc(alias = "CFR1")]
pub type Cfr1 = crate::Reg<cfr1::Cfr1Spec>;
#[doc = "SDRAMC Configuration Register 1"]
pub mod cfr1;
#[doc = "OCMS (rw) register accessor: SDRAMC OCMS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`] module"]
#[doc(alias = "OCMS")]
pub type Ocms = crate::Reg<ocms::OcmsSpec>;
#[doc = "SDRAMC OCMS Register"]
pub mod ocms;
#[doc = "OCMS_KEY1 (w) register accessor: SDRAMC OCMS KEY1 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms_key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms_key1`] module"]
#[doc(alias = "OCMS_KEY1")]
pub type OcmsKey1 = crate::Reg<ocms_key1::OcmsKey1Spec>;
#[doc = "SDRAMC OCMS KEY1 Register"]
pub mod ocms_key1;
#[doc = "OCMS_KEY2 (w) register accessor: SDRAMC OCMS KEY2 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms_key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms_key2`] module"]
#[doc(alias = "OCMS_KEY2")]
pub type OcmsKey2 = crate::Reg<ocms_key2::OcmsKey2Spec>;
#[doc = "SDRAMC OCMS KEY2 Register"]
pub mod ocms_key2;
