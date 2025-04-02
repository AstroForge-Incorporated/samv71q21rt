#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mlbc0: Mlbc0,
    _reserved1: [u8; 0x08],
    ms0: Ms0,
    _reserved2: [u8; 0x04],
    ms1: Ms1,
    _reserved3: [u8; 0x08],
    mss: Mss,
    msd: Msd,
    _reserved5: [u8; 0x04],
    mien: Mien,
    _reserved6: [u8; 0x0c],
    mlbc1: Mlbc1,
    _reserved7: [u8; 0x40],
    hctl: Hctl,
    _reserved8: [u8; 0x04],
    hcmr: [Hcmr; 2],
    hcer: [Hcer; 2],
    hcbr: [Hcbr; 2],
    _reserved11: [u8; 0x20],
    mdat: [Mdat; 4],
    mdwe: [Mdwe; 4],
    mctl: Mctl,
    madr: Madr,
    _reserved15: [u8; 0x02d8],
    actl: Actl,
    _reserved16: [u8; 0x0c],
    acsr: [Acsr; 2],
    acmr: [Acmr; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - MediaLB Control 0 Register"]
    #[inline(always)]
    pub const fn mlbc0(&self) -> &Mlbc0 {
        &self.mlbc0
    }
    #[doc = "0x0c - MediaLB Channel Status 0 Register"]
    #[inline(always)]
    pub const fn ms0(&self) -> &Ms0 {
        &self.ms0
    }
    #[doc = "0x14 - MediaLB Channel Status1 Register"]
    #[inline(always)]
    pub const fn ms1(&self) -> &Ms1 {
        &self.ms1
    }
    #[doc = "0x20 - MediaLB System Status Register"]
    #[inline(always)]
    pub const fn mss(&self) -> &Mss {
        &self.mss
    }
    #[doc = "0x24 - MediaLB System Data Register"]
    #[inline(always)]
    pub const fn msd(&self) -> &Msd {
        &self.msd
    }
    #[doc = "0x2c - MediaLB Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mien(&self) -> &Mien {
        &self.mien
    }
    #[doc = "0x3c - MediaLB Control 1 Register"]
    #[inline(always)]
    pub const fn mlbc1(&self) -> &Mlbc1 {
        &self.mlbc1
    }
    #[doc = "0x80 - HBI Control Register"]
    #[inline(always)]
    pub const fn hctl(&self) -> &Hctl {
        &self.hctl
    }
    #[doc = "0x88..0x90 - HBI Channel Mask 0 Register 0"]
    #[inline(always)]
    pub const fn hcmr(&self, n: usize) -> &Hcmr {
        &self.hcmr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x90 - HBI Channel Mask 0 Register 0"]
    #[inline(always)]
    pub fn hcmr_iter(&self) -> impl Iterator<Item = &Hcmr> {
        self.hcmr.iter()
    }
    #[doc = "0x90..0x98 - HBI Channel Error 0 Register 0"]
    #[inline(always)]
    pub const fn hcer(&self, n: usize) -> &Hcer {
        &self.hcer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0x98 - HBI Channel Error 0 Register 0"]
    #[inline(always)]
    pub fn hcer_iter(&self) -> impl Iterator<Item = &Hcer> {
        self.hcer.iter()
    }
    #[doc = "0x98..0xa0 - HBI Channel Busy 0 Register 0"]
    #[inline(always)]
    pub const fn hcbr(&self, n: usize) -> &Hcbr {
        &self.hcbr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xa0 - HBI Channel Busy 0 Register 0"]
    #[inline(always)]
    pub fn hcbr_iter(&self) -> impl Iterator<Item = &Hcbr> {
        self.hcbr.iter()
    }
    #[doc = "0xc0..0xd0 - MIF Data 0 Register 0"]
    #[inline(always)]
    pub const fn mdat(&self, n: usize) -> &Mdat {
        &self.mdat[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xd0 - MIF Data 0 Register 0"]
    #[inline(always)]
    pub fn mdat_iter(&self) -> impl Iterator<Item = &Mdat> {
        self.mdat.iter()
    }
    #[doc = "0xd0..0xe0 - MIF Data Write Enable 0 Register 0"]
    #[inline(always)]
    pub const fn mdwe(&self, n: usize) -> &Mdwe {
        &self.mdwe[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd0..0xe0 - MIF Data Write Enable 0 Register 0"]
    #[inline(always)]
    pub fn mdwe_iter(&self) -> impl Iterator<Item = &Mdwe> {
        self.mdwe.iter()
    }
    #[doc = "0xe0 - MIF Control Register"]
    #[inline(always)]
    pub const fn mctl(&self) -> &Mctl {
        &self.mctl
    }
    #[doc = "0xe4 - MIF Address Register"]
    #[inline(always)]
    pub const fn madr(&self) -> &Madr {
        &self.madr
    }
    #[doc = "0x3c0 - AHB Control Register"]
    #[inline(always)]
    pub const fn actl(&self) -> &Actl {
        &self.actl
    }
    #[doc = "0x3d0..0x3d8 - AHB Channel Status 0 Register 0"]
    #[inline(always)]
    pub const fn acsr(&self, n: usize) -> &Acsr {
        &self.acsr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3d0..0x3d8 - AHB Channel Status 0 Register 0"]
    #[inline(always)]
    pub fn acsr_iter(&self) -> impl Iterator<Item = &Acsr> {
        self.acsr.iter()
    }
    #[doc = "0x3d8..0x3e0 - AHB Channel Mask 0 Register 0"]
    #[inline(always)]
    pub const fn acmr(&self, n: usize) -> &Acmr {
        &self.acmr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3d8..0x3e0 - AHB Channel Mask 0 Register 0"]
    #[inline(always)]
    pub fn acmr_iter(&self) -> impl Iterator<Item = &Acmr> {
        self.acmr.iter()
    }
}
#[doc = "MLBC0 (rw) register accessor: MediaLB Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mlbc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlbc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mlbc0`] module"]
#[doc(alias = "MLBC0")]
pub type Mlbc0 = crate::Reg<mlbc0::Mlbc0Spec>;
#[doc = "MediaLB Control 0 Register"]
pub mod mlbc0;
#[doc = "MS0 (rw) register accessor: MediaLB Channel Status 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ms0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms0`] module"]
#[doc(alias = "MS0")]
pub type Ms0 = crate::Reg<ms0::Ms0Spec>;
#[doc = "MediaLB Channel Status 0 Register"]
pub mod ms0;
#[doc = "MS1 (rw) register accessor: MediaLB Channel Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ms1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms1`] module"]
#[doc(alias = "MS1")]
pub type Ms1 = crate::Reg<ms1::Ms1Spec>;
#[doc = "MediaLB Channel Status1 Register"]
pub mod ms1;
#[doc = "MSS (rw) register accessor: MediaLB System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss`] module"]
#[doc(alias = "MSS")]
pub type Mss = crate::Reg<mss::MssSpec>;
#[doc = "MediaLB System Status Register"]
pub mod mss;
#[doc = "MSD (r) register accessor: MediaLB System Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msd`] module"]
#[doc(alias = "MSD")]
pub type Msd = crate::Reg<msd::MsdSpec>;
#[doc = "MediaLB System Data Register"]
pub mod msd;
#[doc = "MIEN (rw) register accessor: MediaLB Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mien`] module"]
#[doc(alias = "MIEN")]
pub type Mien = crate::Reg<mien::MienSpec>;
#[doc = "MediaLB Interrupt Enable Register"]
pub mod mien;
#[doc = "MLBC1 (rw) register accessor: MediaLB Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mlbc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlbc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mlbc1`] module"]
#[doc(alias = "MLBC1")]
pub type Mlbc1 = crate::Reg<mlbc1::Mlbc1Spec>;
#[doc = "MediaLB Control 1 Register"]
pub mod mlbc1;
#[doc = "HCTL (rw) register accessor: HBI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctl`] module"]
#[doc(alias = "HCTL")]
pub type Hctl = crate::Reg<hctl::HctlSpec>;
#[doc = "HBI Control Register"]
pub mod hctl;
#[doc = "HCMR (rw) register accessor: HBI Channel Mask 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcmr`] module"]
#[doc(alias = "HCMR")]
pub type Hcmr = crate::Reg<hcmr::HcmrSpec>;
#[doc = "HBI Channel Mask 0 Register 0"]
pub mod hcmr;
#[doc = "HCER (r) register accessor: HBI Channel Error 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcer`] module"]
#[doc(alias = "HCER")]
pub type Hcer = crate::Reg<hcer::HcerSpec>;
#[doc = "HBI Channel Error 0 Register 0"]
pub mod hcer;
#[doc = "HCBR (r) register accessor: HBI Channel Busy 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbr`] module"]
#[doc(alias = "HCBR")]
pub type Hcbr = crate::Reg<hcbr::HcbrSpec>;
#[doc = "HBI Channel Busy 0 Register 0"]
pub mod hcbr;
#[doc = "MDAT (rw) register accessor: MIF Data 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdat`] module"]
#[doc(alias = "MDAT")]
pub type Mdat = crate::Reg<mdat::MdatSpec>;
#[doc = "MIF Data 0 Register 0"]
pub mod mdat;
#[doc = "MDWE (rw) register accessor: MIF Data Write Enable 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdwe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdwe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdwe`] module"]
#[doc(alias = "MDWE")]
pub type Mdwe = crate::Reg<mdwe::MdweSpec>;
#[doc = "MIF Data Write Enable 0 Register 0"]
pub mod mdwe;
#[doc = "MCTL (rw) register accessor: MIF Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctl`] module"]
#[doc(alias = "MCTL")]
pub type Mctl = crate::Reg<mctl::MctlSpec>;
#[doc = "MIF Control Register"]
pub mod mctl;
#[doc = "MADR (rw) register accessor: MIF Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@madr`] module"]
#[doc(alias = "MADR")]
pub type Madr = crate::Reg<madr::MadrSpec>;
#[doc = "MIF Address Register"]
pub mod madr;
#[doc = "ACTL (rw) register accessor: AHB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actl`] module"]
#[doc(alias = "ACTL")]
pub type Actl = crate::Reg<actl::ActlSpec>;
#[doc = "AHB Control Register"]
pub mod actl;
#[doc = "ACSR (rw) register accessor: AHB Channel Status 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`acsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acsr`] module"]
#[doc(alias = "ACSR")]
pub type Acsr = crate::Reg<acsr::AcsrSpec>;
#[doc = "AHB Channel Status 0 Register 0"]
pub mod acsr;
#[doc = "ACMR (rw) register accessor: AHB Channel Mask 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`acmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmr`] module"]
#[doc(alias = "ACMR")]
pub type Acmr = crate::Reg<acmr::AcmrSpec>;
#[doc = "AHB Channel Mask 0 Register 0"]
pub mod acmr;
