#[repr(C)]
#[doc = "Channel Interrupt Enable Register"]
#[doc(alias = "XDMAC_CHID")]
pub struct XdmacChid {
    cie: Cie,
    cid: Cid,
    cim: Cim,
    cis: Cis,
    csa: Csa,
    cda: Cda,
    cnda: Cnda,
    cndc: Cndc,
    cubc: Cubc,
    cbc: Cbc,
    cc: Cc,
    cds_msp: CdsMsp,
    csus: Csus,
    cdus: Cdus,
}
impl XdmacChid {
    #[doc = "0x00 - Channel Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cie(&self) -> &Cie {
        &self.cie
    }
    #[doc = "0x04 - Channel Interrupt Disable Register"]
    #[inline(always)]
    pub const fn cid(&self) -> &Cid {
        &self.cid
    }
    #[doc = "0x08 - Channel Interrupt Mask Register"]
    #[inline(always)]
    pub const fn cim(&self) -> &Cim {
        &self.cim
    }
    #[doc = "0x0c - Channel Interrupt Status Register"]
    #[inline(always)]
    pub const fn cis(&self) -> &Cis {
        &self.cis
    }
    #[doc = "0x10 - Channel Source Address Register"]
    #[inline(always)]
    pub const fn csa(&self) -> &Csa {
        &self.csa
    }
    #[doc = "0x14 - Channel Destination Address Register"]
    #[inline(always)]
    pub const fn cda(&self) -> &Cda {
        &self.cda
    }
    #[doc = "0x18 - Channel Next Descriptor Address Register"]
    #[inline(always)]
    pub const fn cnda(&self) -> &Cnda {
        &self.cnda
    }
    #[doc = "0x1c - Channel Next Descriptor Control Register"]
    #[inline(always)]
    pub const fn cndc(&self) -> &Cndc {
        &self.cndc
    }
    #[doc = "0x20 - Channel Microblock Control Register"]
    #[inline(always)]
    pub const fn cubc(&self) -> &Cubc {
        &self.cubc
    }
    #[doc = "0x24 - Channel Block Control Register"]
    #[inline(always)]
    pub const fn cbc(&self) -> &Cbc {
        &self.cbc
    }
    #[doc = "0x28 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn cc(&self) -> &Cc {
        &self.cc
    }
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern"]
    #[inline(always)]
    pub const fn cds_msp(&self) -> &CdsMsp {
        &self.cds_msp
    }
    #[doc = "0x30 - Channel Source Microblock Stride"]
    #[inline(always)]
    pub const fn csus(&self) -> &Csus {
        &self.csus
    }
    #[doc = "0x34 - Channel Destination Microblock Stride"]
    #[inline(always)]
    pub const fn cdus(&self) -> &Cdus {
        &self.cdus
    }
}
#[doc = "CIE (w) register accessor: Channel Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cie::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cie`] module"]
#[doc(alias = "CIE")]
pub type Cie = crate::Reg<cie::CieSpec>;
#[doc = "Channel Interrupt Enable Register"]
pub mod cie;
#[doc = "CID (w) register accessor: Channel Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`] module"]
#[doc(alias = "CID")]
pub type Cid = crate::Reg<cid::CidSpec>;
#[doc = "Channel Interrupt Disable Register"]
pub mod cid;
#[doc = "CIM (r) register accessor: Channel Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cim`] module"]
#[doc(alias = "CIM")]
pub type Cim = crate::Reg<cim::CimSpec>;
#[doc = "Channel Interrupt Mask Register"]
pub mod cim;
#[doc = "CIS (r) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cis`] module"]
#[doc(alias = "CIS")]
pub type Cis = crate::Reg<cis::CisSpec>;
#[doc = "Channel Interrupt Status Register"]
pub mod cis;
#[doc = "CSA (rw) register accessor: Channel Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csa`] module"]
#[doc(alias = "CSA")]
pub type Csa = crate::Reg<csa::CsaSpec>;
#[doc = "Channel Source Address Register"]
pub mod csa;
#[doc = "CDA (rw) register accessor: Channel Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cda`] module"]
#[doc(alias = "CDA")]
pub type Cda = crate::Reg<cda::CdaSpec>;
#[doc = "Channel Destination Address Register"]
pub mod cda;
#[doc = "CNDA (rw) register accessor: Channel Next Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnda`] module"]
#[doc(alias = "CNDA")]
pub type Cnda = crate::Reg<cnda::CndaSpec>;
#[doc = "Channel Next Descriptor Address Register"]
pub mod cnda;
#[doc = "CNDC (rw) register accessor: Channel Next Descriptor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndc`] module"]
#[doc(alias = "CNDC")]
pub type Cndc = crate::Reg<cndc::CndcSpec>;
#[doc = "Channel Next Descriptor Control Register"]
pub mod cndc;
#[doc = "CUBC (rw) register accessor: Channel Microblock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cubc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cubc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cubc`] module"]
#[doc(alias = "CUBC")]
pub type Cubc = crate::Reg<cubc::CubcSpec>;
#[doc = "Channel Microblock Control Register"]
pub mod cubc;
#[doc = "CBC (rw) register accessor: Channel Block Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbc`] module"]
#[doc(alias = "CBC")]
pub type Cbc = crate::Reg<cbc::CbcSpec>;
#[doc = "Channel Block Control Register"]
pub mod cbc;
#[doc = "CC (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Channel Configuration Register"]
pub mod cc;
#[doc = "CDS_MSP (rw) register accessor: Channel Data Stride Memory Set Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`cds_msp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cds_msp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cds_msp`] module"]
#[doc(alias = "CDS_MSP")]
pub type CdsMsp = crate::Reg<cds_msp::CdsMspSpec>;
#[doc = "Channel Data Stride Memory Set Pattern"]
pub mod cds_msp;
#[doc = "CSUS (rw) register accessor: Channel Source Microblock Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`csus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csus`] module"]
#[doc(alias = "CSUS")]
pub type Csus = crate::Reg<csus::CsusSpec>;
#[doc = "Channel Source Microblock Stride"]
pub mod csus;
#[doc = "CDUS (rw) register accessor: Channel Destination Microblock Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`cdus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdus`] module"]
#[doc(alias = "CDUS")]
pub type Cdus = crate::Reg<cdus::CdusSpec>;
#[doc = "Channel Destination Microblock Stride"]
pub mod cdus;
