#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcfg: [Mcfg; 13],
    _reserved1: [u8; 0x0c],
    scfg: [Scfg; 9],
    _reserved2: [u8; 0x1c],
    matrix_pr: [MatrixPr; 9],
    _reserved3: [u8; 0x38],
    mrcr: Mrcr,
    _reserved4: [u8; 0x0c],
    ccfg_can0: CcfgCan0,
    ccfg_sysio: CcfgSysio,
    ccfg_pccr: CcfgPccr,
    ccfg_dynckg: CcfgDynckg,
    _reserved8: [u8; 0x04],
    ccfg_smcnfcs: CcfgSmcnfcs,
    _reserved9: [u8; 0xbc],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00..0x34 - Master Configuration Register 0"]
    #[inline(always)]
    pub const fn mcfg(&self, n: usize) -> &Mcfg {
        &self.mcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x34 - Master Configuration Register 0"]
    #[inline(always)]
    pub fn mcfg_iter(&self) -> impl Iterator<Item = &Mcfg> {
        self.mcfg.iter()
    }
    #[doc = "0x40..0x64 - Slave Configuration Register 0"]
    #[inline(always)]
    pub const fn scfg(&self, n: usize) -> &Scfg {
        &self.scfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x64 - Slave Configuration Register 0"]
    #[inline(always)]
    pub fn scfg_iter(&self) -> impl Iterator<Item = &Scfg> {
        self.scfg.iter()
    }
    #[doc = "0x80..0xc8 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn matrix_pr(&self, n: usize) -> &MatrixPr {
        &self.matrix_pr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc8 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub fn matrix_pr_iter(&self) -> impl Iterator<Item = &MatrixPr> {
        self.matrix_pr.iter()
    }
    #[doc = "0x100 - Master Remap Control Register"]
    #[inline(always)]
    pub const fn mrcr(&self) -> &Mrcr {
        &self.mrcr
    }
    #[doc = "0x110 - CAN0 Configuration Register"]
    #[inline(always)]
    pub const fn ccfg_can0(&self) -> &CcfgCan0 {
        &self.ccfg_can0
    }
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    #[inline(always)]
    pub const fn ccfg_sysio(&self) -> &CcfgSysio {
        &self.ccfg_sysio
    }
    #[doc = "0x118 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn ccfg_pccr(&self) -> &CcfgPccr {
        &self.ccfg_pccr
    }
    #[doc = "0x11c - Dynamic Clock Gating Register"]
    #[inline(always)]
    pub const fn ccfg_dynckg(&self) -> &CcfgDynckg {
        &self.ccfg_dynckg
    }
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    #[inline(always)]
    pub const fn ccfg_smcnfcs(&self) -> &CcfgSmcnfcs {
        &self.ccfg_smcnfcs
    }
    #[doc = "0x1e4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x1e8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "MCFG (rw) register accessor: Master Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`] module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "Master Configuration Register 0"]
pub mod mcfg;
#[doc = "SCFG (rw) register accessor: Slave Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg`] module"]
#[doc(alias = "SCFG")]
pub type Scfg = crate::Reg<scfg::ScfgSpec>;
#[doc = "Slave Configuration Register 0"]
pub mod scfg;
#[doc = "Priority Register A for Slave 0"]
pub use self::matrix_pr::MatrixPr;
#[doc = r"Cluster"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "MRCR (rw) register accessor: Master Remap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcr`] module"]
#[doc(alias = "MRCR")]
pub type Mrcr = crate::Reg<mrcr::MrcrSpec>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "CCFG_CAN0 (rw) register accessor: CAN0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_can0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_can0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_can0`] module"]
#[doc(alias = "CCFG_CAN0")]
pub type CcfgCan0 = crate::Reg<ccfg_can0::CcfgCan0Spec>;
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "CCFG_SYSIO (rw) register accessor: System I/O and CAN1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_sysio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_sysio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_sysio`] module"]
#[doc(alias = "CCFG_SYSIO")]
pub type CcfgSysio = crate::Reg<ccfg_sysio::CcfgSysioSpec>;
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_PCCR (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_pccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_pccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_pccr`] module"]
#[doc(alias = "CCFG_PCCR")]
pub type CcfgPccr = crate::Reg<ccfg_pccr::CcfgPccrSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod ccfg_pccr;
#[doc = "CCFG_DYNCKG (rw) register accessor: Dynamic Clock Gating Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_dynckg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_dynckg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_dynckg`] module"]
#[doc(alias = "CCFG_DYNCKG")]
pub type CcfgDynckg = crate::Reg<ccfg_dynckg::CcfgDynckgSpec>;
#[doc = "Dynamic Clock Gating Register"]
pub mod ccfg_dynckg;
#[doc = "CCFG_SMCNFCS (rw) register accessor: SMC NAND Flash Chip Select Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_smcnfcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_smcnfcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_smcnfcs`] module"]
#[doc(alias = "CCFG_SMCNFCS")]
pub type CcfgSmcnfcs = crate::Reg<ccfg_smcnfcs::CcfgSmcnfcsSpec>;
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
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
