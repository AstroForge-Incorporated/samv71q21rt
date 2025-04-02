#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    _reserved2: [u8; 0x08],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    keywr: [Keywr; 8],
    idatar: [Idatar; 4],
    odatar: [Odatar; 4],
    ivr: [Ivr; 4],
    aadlenr: Aadlenr,
    clenr: Clenr,
    ghashr: [Ghashr; 4],
    tagr: [Tagr; 4],
    ctrr: Ctrr,
    gcmhr: [Gcmhr; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
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
    #[doc = "0x20..0x40 - Key Word Register"]
    #[inline(always)]
    pub const fn keywr(&self, n: usize) -> &Keywr {
        &self.keywr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - Key Word Register"]
    #[inline(always)]
    pub fn keywr_iter(&self) -> impl Iterator<Item = &Keywr> {
        self.keywr.iter()
    }
    #[doc = "0x40..0x50 - Input Data Register"]
    #[inline(always)]
    pub const fn idatar(&self, n: usize) -> &Idatar {
        &self.idatar[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Input Data Register"]
    #[inline(always)]
    pub fn idatar_iter(&self) -> impl Iterator<Item = &Idatar> {
        self.idatar.iter()
    }
    #[doc = "0x50..0x60 - Output Data Register"]
    #[inline(always)]
    pub const fn odatar(&self, n: usize) -> &Odatar {
        &self.odatar[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - Output Data Register"]
    #[inline(always)]
    pub fn odatar_iter(&self) -> impl Iterator<Item = &Odatar> {
        self.odatar.iter()
    }
    #[doc = "0x60..0x70 - Initialization Vector Register"]
    #[inline(always)]
    pub const fn ivr(&self, n: usize) -> &Ivr {
        &self.ivr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Initialization Vector Register"]
    #[inline(always)]
    pub fn ivr_iter(&self) -> impl Iterator<Item = &Ivr> {
        self.ivr.iter()
    }
    #[doc = "0x70 - Additional Authenticated Data Length Register"]
    #[inline(always)]
    pub const fn aadlenr(&self) -> &Aadlenr {
        &self.aadlenr
    }
    #[doc = "0x74 - Plaintext/Ciphertext Length Register"]
    #[inline(always)]
    pub const fn clenr(&self) -> &Clenr {
        &self.clenr
    }
    #[doc = "0x78..0x88 - GCM Intermediate Hash Word Register"]
    #[inline(always)]
    pub const fn ghashr(&self, n: usize) -> &Ghashr {
        &self.ghashr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x78..0x88 - GCM Intermediate Hash Word Register"]
    #[inline(always)]
    pub fn ghashr_iter(&self) -> impl Iterator<Item = &Ghashr> {
        self.ghashr.iter()
    }
    #[doc = "0x88..0x98 - GCM Authentication Tag Word Register"]
    #[inline(always)]
    pub const fn tagr(&self, n: usize) -> &Tagr {
        &self.tagr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x98 - GCM Authentication Tag Word Register"]
    #[inline(always)]
    pub fn tagr_iter(&self) -> impl Iterator<Item = &Tagr> {
        self.tagr.iter()
    }
    #[doc = "0x98 - GCM Encryption Counter Value Register"]
    #[inline(always)]
    pub const fn ctrr(&self) -> &Ctrr {
        &self.ctrr
    }
    #[doc = "0x9c..0xac - GCM H Word Register"]
    #[inline(always)]
    pub const fn gcmhr(&self, n: usize) -> &Gcmhr {
        &self.gcmhr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0xac - GCM H Word Register"]
    #[inline(always)]
    pub fn gcmhr_iter(&self) -> impl Iterator<Item = &Gcmhr> {
        self.gcmhr.iter()
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
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
#[doc = "KEYWR (w) register accessor: Key Word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keywr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keywr`] module"]
#[doc(alias = "KEYWR")]
pub type Keywr = crate::Reg<keywr::KeywrSpec>;
#[doc = "Key Word Register"]
pub mod keywr;
#[doc = "IDATAR (w) register accessor: Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar`] module"]
#[doc(alias = "IDATAR")]
pub type Idatar = crate::Reg<idatar::IdatarSpec>;
#[doc = "Input Data Register"]
pub mod idatar;
#[doc = "ODATAR (r) register accessor: Output Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`odatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odatar`] module"]
#[doc(alias = "ODATAR")]
pub type Odatar = crate::Reg<odatar::OdatarSpec>;
#[doc = "Output Data Register"]
pub mod odatar;
#[doc = "IVR (w) register accessor: Initialization Vector Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr`] module"]
#[doc(alias = "IVR")]
pub type Ivr = crate::Reg<ivr::IvrSpec>;
#[doc = "Initialization Vector Register"]
pub mod ivr;
#[doc = "AADLENR (rw) register accessor: Additional Authenticated Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aadlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aadlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aadlenr`] module"]
#[doc(alias = "AADLENR")]
pub type Aadlenr = crate::Reg<aadlenr::AadlenrSpec>;
#[doc = "Additional Authenticated Data Length Register"]
pub mod aadlenr;
#[doc = "CLENR (rw) register accessor: Plaintext/Ciphertext Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clenr`] module"]
#[doc(alias = "CLENR")]
pub type Clenr = crate::Reg<clenr::ClenrSpec>;
#[doc = "Plaintext/Ciphertext Length Register"]
pub mod clenr;
#[doc = "GHASHR (rw) register accessor: GCM Intermediate Hash Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashr`] module"]
#[doc(alias = "GHASHR")]
pub type Ghashr = crate::Reg<ghashr::GhashrSpec>;
#[doc = "GCM Intermediate Hash Word Register"]
pub mod ghashr;
#[doc = "TAGR (r) register accessor: GCM Authentication Tag Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tagr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagr`] module"]
#[doc(alias = "TAGR")]
pub type Tagr = crate::Reg<tagr::TagrSpec>;
#[doc = "GCM Authentication Tag Word Register"]
pub mod tagr;
#[doc = "CTRR (r) register accessor: GCM Encryption Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrr`] module"]
#[doc(alias = "CTRR")]
pub type Ctrr = crate::Reg<ctrr::CtrrSpec>;
#[doc = "GCM Encryption Counter Value Register"]
pub mod ctrr;
#[doc = "GCMHR (rw) register accessor: GCM H Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmhr`] module"]
#[doc(alias = "GCMHR")]
pub type Gcmhr = crate::Reg<gcmhr::GcmhrSpec>;
#[doc = "GCM H Word Register"]
pub mod gcmhr;
