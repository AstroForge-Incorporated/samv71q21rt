#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    ohciicr: Ohciicr,
    _reserved1: [u8; 0x1c],
    cktrim: Cktrim,
}
impl RegisterBlock {
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    #[inline(always)]
    pub const fn ohciicr(&self) -> &Ohciicr {
        &self.ohciicr
    }
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    #[inline(always)]
    pub const fn cktrim(&self) -> &Cktrim {
        &self.cktrim
    }
}
#[doc = "OHCIICR (rw) register accessor: OHCI Interrupt Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ohciicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ohciicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ohciicr`] module"]
#[doc(alias = "OHCIICR")]
pub type Ohciicr = crate::Reg<ohciicr::OhciicrSpec>;
#[doc = "OHCI Interrupt Configuration Register"]
pub mod ohciicr;
#[doc = "CKTRIM (rw) register accessor: UTMI Clock Trimming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cktrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cktrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cktrim`] module"]
#[doc(alias = "CKTRIM")]
pub type Cktrim = crate::Reg<cktrim::CktrimSpec>;
#[doc = "UTMI Clock Trimming Register"]
pub mod cktrim;
