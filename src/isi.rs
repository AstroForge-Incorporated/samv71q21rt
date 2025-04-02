#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg1: Cfg1,
    cfg2: Cfg2,
    psize: Psize,
    pdecf: Pdecf,
    y2r_set0: Y2rSet0,
    y2r_set1: Y2rSet1,
    r2y_set0: R2ySet0,
    r2y_set1: R2ySet1,
    r2y_set2: R2ySet2,
    cr: Cr,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    dma_cher: DmaCher,
    dma_chdr: DmaChdr,
    dma_chsr: DmaChsr,
    dma_p_addr: DmaPAddr,
    dma_p_ctrl: DmaPCtrl,
    dma_p_dscr: DmaPDscr,
    dma_c_addr: DmaCAddr,
    dma_c_ctrl: DmaCCtrl,
    dma_c_dscr: DmaCDscr,
    _reserved23: [u8; 0x88],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - ISI Configuration 1 Register"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x04 - ISI Configuration 2 Register"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x08 - ISI Preview Size Register"]
    #[inline(always)]
    pub const fn psize(&self) -> &Psize {
        &self.psize
    }
    #[doc = "0x0c - ISI Preview Decimation Factor Register"]
    #[inline(always)]
    pub const fn pdecf(&self) -> &Pdecf {
        &self.pdecf
    }
    #[doc = "0x10 - ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
    #[inline(always)]
    pub const fn y2r_set0(&self) -> &Y2rSet0 {
        &self.y2r_set0
    }
    #[doc = "0x14 - ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
    #[inline(always)]
    pub const fn y2r_set1(&self) -> &Y2rSet1 {
        &self.y2r_set1
    }
    #[doc = "0x18 - ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
    #[inline(always)]
    pub const fn r2y_set0(&self) -> &R2ySet0 {
        &self.r2y_set0
    }
    #[doc = "0x1c - ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
    #[inline(always)]
    pub const fn r2y_set1(&self) -> &R2ySet1 {
        &self.r2y_set1
    }
    #[doc = "0x20 - ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
    #[inline(always)]
    pub const fn r2y_set2(&self) -> &R2ySet2 {
        &self.r2y_set2
    }
    #[doc = "0x24 - ISI Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x28 - ISI Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x2c - ISI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x30 - ISI Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x34 - ISI Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x38 - DMA Channel Enable Register"]
    #[inline(always)]
    pub const fn dma_cher(&self) -> &DmaCher {
        &self.dma_cher
    }
    #[doc = "0x3c - DMA Channel Disable Register"]
    #[inline(always)]
    pub const fn dma_chdr(&self) -> &DmaChdr {
        &self.dma_chdr
    }
    #[doc = "0x40 - DMA Channel Status Register"]
    #[inline(always)]
    pub const fn dma_chsr(&self) -> &DmaChsr {
        &self.dma_chsr
    }
    #[doc = "0x44 - DMA Preview Base Address Register"]
    #[inline(always)]
    pub const fn dma_p_addr(&self) -> &DmaPAddr {
        &self.dma_p_addr
    }
    #[doc = "0x48 - DMA Preview Control Register"]
    #[inline(always)]
    pub const fn dma_p_ctrl(&self) -> &DmaPCtrl {
        &self.dma_p_ctrl
    }
    #[doc = "0x4c - DMA Preview Descriptor Address Register"]
    #[inline(always)]
    pub const fn dma_p_dscr(&self) -> &DmaPDscr {
        &self.dma_p_dscr
    }
    #[doc = "0x50 - DMA Codec Base Address Register"]
    #[inline(always)]
    pub const fn dma_c_addr(&self) -> &DmaCAddr {
        &self.dma_c_addr
    }
    #[doc = "0x54 - DMA Codec Control Register"]
    #[inline(always)]
    pub const fn dma_c_ctrl(&self) -> &DmaCCtrl {
        &self.dma_c_ctrl
    }
    #[doc = "0x58 - DMA Codec Descriptor Address Register"]
    #[inline(always)]
    pub const fn dma_c_dscr(&self) -> &DmaCDscr {
        &self.dma_c_dscr
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
}
#[doc = "CFG1 (rw) register accessor: ISI Configuration 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "ISI Configuration 1 Register"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: ISI Configuration 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`] module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "ISI Configuration 2 Register"]
pub mod cfg2;
#[doc = "PSIZE (rw) register accessor: ISI Preview Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psize`] module"]
#[doc(alias = "PSIZE")]
pub type Psize = crate::Reg<psize::PsizeSpec>;
#[doc = "ISI Preview Size Register"]
pub mod psize;
#[doc = "PDECF (rw) register accessor: ISI Preview Decimation Factor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdecf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdecf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdecf`] module"]
#[doc(alias = "PDECF")]
pub type Pdecf = crate::Reg<pdecf::PdecfSpec>;
#[doc = "ISI Preview Decimation Factor Register"]
pub mod pdecf;
#[doc = "Y2R_SET0 (rw) register accessor: ISI Color Space Conversion YCrCb To RGB Set 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`y2r_set0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y2r_set0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@y2r_set0`] module"]
#[doc(alias = "Y2R_SET0")]
pub type Y2rSet0 = crate::Reg<y2r_set0::Y2rSet0Spec>;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub mod y2r_set0;
#[doc = "Y2R_SET1 (rw) register accessor: ISI Color Space Conversion YCrCb To RGB Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`y2r_set1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y2r_set1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@y2r_set1`] module"]
#[doc(alias = "Y2R_SET1")]
pub type Y2rSet1 = crate::Reg<y2r_set1::Y2rSet1Spec>;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub mod y2r_set1;
#[doc = "R2Y_SET0 (rw) register accessor: ISI Color Space Conversion RGB To YCrCb Set 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r2y_set0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2y_set0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2y_set0`] module"]
#[doc(alias = "R2Y_SET0")]
pub type R2ySet0 = crate::Reg<r2y_set0::R2ySet0Spec>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub mod r2y_set0;
#[doc = "R2Y_SET1 (rw) register accessor: ISI Color Space Conversion RGB To YCrCb Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r2y_set1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2y_set1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2y_set1`] module"]
#[doc(alias = "R2Y_SET1")]
pub type R2ySet1 = crate::Reg<r2y_set1::R2ySet1Spec>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub mod r2y_set1;
#[doc = "R2Y_SET2 (rw) register accessor: ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r2y_set2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2y_set2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2y_set2`] module"]
#[doc(alias = "R2Y_SET2")]
pub type R2ySet2 = crate::Reg<r2y_set2::R2ySet2Spec>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub mod r2y_set2;
#[doc = "CR (w) register accessor: ISI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "ISI Control Register"]
pub mod cr;
#[doc = "SR (r) register accessor: ISI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "ISI Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: ISI Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "ISI Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: ISI Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "ISI Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: ISI Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "ISI Interrupt Mask Register"]
pub mod imr;
#[doc = "DMA_CHER (w) register accessor: DMA Channel Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cher`] module"]
#[doc(alias = "DMA_CHER")]
pub type DmaCher = crate::Reg<dma_cher::DmaCherSpec>;
#[doc = "DMA Channel Enable Register"]
pub mod dma_cher;
#[doc = "DMA_CHDR (w) register accessor: DMA Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_chdr`] module"]
#[doc(alias = "DMA_CHDR")]
pub type DmaChdr = crate::Reg<dma_chdr::DmaChdrSpec>;
#[doc = "DMA Channel Disable Register"]
pub mod dma_chdr;
#[doc = "DMA_CHSR (r) register accessor: DMA Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_chsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_chsr`] module"]
#[doc(alias = "DMA_CHSR")]
pub type DmaChsr = crate::Reg<dma_chsr::DmaChsrSpec>;
#[doc = "DMA Channel Status Register"]
pub mod dma_chsr;
#[doc = "DMA_P_ADDR (rw) register accessor: DMA Preview Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_p_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_p_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_p_addr`] module"]
#[doc(alias = "DMA_P_ADDR")]
pub type DmaPAddr = crate::Reg<dma_p_addr::DmaPAddrSpec>;
#[doc = "DMA Preview Base Address Register"]
pub mod dma_p_addr;
#[doc = "DMA_P_CTRL (rw) register accessor: DMA Preview Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_p_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_p_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_p_ctrl`] module"]
#[doc(alias = "DMA_P_CTRL")]
pub type DmaPCtrl = crate::Reg<dma_p_ctrl::DmaPCtrlSpec>;
#[doc = "DMA Preview Control Register"]
pub mod dma_p_ctrl;
#[doc = "DMA_P_DSCR (rw) register accessor: DMA Preview Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_p_dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_p_dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_p_dscr`] module"]
#[doc(alias = "DMA_P_DSCR")]
pub type DmaPDscr = crate::Reg<dma_p_dscr::DmaPDscrSpec>;
#[doc = "DMA Preview Descriptor Address Register"]
pub mod dma_p_dscr;
#[doc = "DMA_C_ADDR (rw) register accessor: DMA Codec Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c_addr`] module"]
#[doc(alias = "DMA_C_ADDR")]
pub type DmaCAddr = crate::Reg<dma_c_addr::DmaCAddrSpec>;
#[doc = "DMA Codec Base Address Register"]
pub mod dma_c_addr;
#[doc = "DMA_C_CTRL (rw) register accessor: DMA Codec Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c_ctrl`] module"]
#[doc(alias = "DMA_C_CTRL")]
pub type DmaCCtrl = crate::Reg<dma_c_ctrl::DmaCCtrlSpec>;
#[doc = "DMA Codec Control Register"]
pub mod dma_c_ctrl;
#[doc = "DMA_C_DSCR (rw) register accessor: DMA Codec Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c_dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c_dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c_dscr`] module"]
#[doc(alias = "DMA_C_DSCR")]
pub type DmaCDscr = crate::Reg<dma_c_dscr::DmaCDscrSpec>;
#[doc = "DMA Codec Descriptor Address Register"]
pub mod dma_c_dscr;
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
