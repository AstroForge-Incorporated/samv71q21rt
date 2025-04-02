#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    emr: Emr,
    seq1r: Seq1r,
    seq2r: Seq2r,
    cher: Cher,
    chdr: Chdr,
    chsr: Chsr,
    lcdr: Lcdr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    _reserved13: [u8; 0x18],
    over: Over,
    cwr: Cwr,
    cgr: Cgr,
    _reserved16: [u8; 0x08],
    diffr: Diffr,
    cselr: Cselr,
    cdr: Cdr,
    cocr: Cocr,
    tempmr: Tempmr,
    tempcwr: Tempcwr,
    _reserved22: [u8; 0x1c],
    acr: Acr,
    _reserved23: [u8; 0x08],
    shmr: Shmr,
    _reserved24: [u8; 0x2c],
    cosr: Cosr,
    cvr: Cvr,
    cecr: Cecr,
    _reserved27: [u8; 0x08],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - AFEC Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - AFEC Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x08 - AFEC Extended Mode Register"]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
    #[doc = "0x0c - AFEC Channel Sequence 1 Register"]
    #[inline(always)]
    pub const fn seq1r(&self) -> &Seq1r {
        &self.seq1r
    }
    #[doc = "0x10 - AFEC Channel Sequence 2 Register"]
    #[inline(always)]
    pub const fn seq2r(&self) -> &Seq2r {
        &self.seq2r
    }
    #[doc = "0x14 - AFEC Channel Enable Register"]
    #[inline(always)]
    pub const fn cher(&self) -> &Cher {
        &self.cher
    }
    #[doc = "0x18 - AFEC Channel Disable Register"]
    #[inline(always)]
    pub const fn chdr(&self) -> &Chdr {
        &self.chdr
    }
    #[doc = "0x1c - AFEC Channel Status Register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &Chsr {
        &self.chsr
    }
    #[doc = "0x20 - AFEC Last Converted Data Register"]
    #[inline(always)]
    pub const fn lcdr(&self) -> &Lcdr {
        &self.lcdr
    }
    #[doc = "0x24 - AFEC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x28 - AFEC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x2c - AFEC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - AFEC Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x4c - AFEC Overrun Status Register"]
    #[inline(always)]
    pub const fn over(&self) -> &Over {
        &self.over
    }
    #[doc = "0x50 - AFEC Compare Window Register"]
    #[inline(always)]
    pub const fn cwr(&self) -> &Cwr {
        &self.cwr
    }
    #[doc = "0x54 - AFEC Channel Gain Register"]
    #[inline(always)]
    pub const fn cgr(&self) -> &Cgr {
        &self.cgr
    }
    #[doc = "0x60 - AFEC Channel Differential Register"]
    #[inline(always)]
    pub const fn diffr(&self) -> &Diffr {
        &self.diffr
    }
    #[doc = "0x64 - AFEC Channel Selection Register"]
    #[inline(always)]
    pub const fn cselr(&self) -> &Cselr {
        &self.cselr
    }
    #[doc = "0x68 - AFEC Channel Data Register"]
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
    #[doc = "0x6c - AFEC Channel Offset Compensation Register"]
    #[inline(always)]
    pub const fn cocr(&self) -> &Cocr {
        &self.cocr
    }
    #[doc = "0x70 - AFEC Temperature Sensor Mode Register"]
    #[inline(always)]
    pub const fn tempmr(&self) -> &Tempmr {
        &self.tempmr
    }
    #[doc = "0x74 - AFEC Temperature Compare Window Register"]
    #[inline(always)]
    pub const fn tempcwr(&self) -> &Tempcwr {
        &self.tempcwr
    }
    #[doc = "0x94 - AFEC Analog Control Register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0xa0 - AFEC Sample & Hold Mode Register"]
    #[inline(always)]
    pub const fn shmr(&self) -> &Shmr {
        &self.shmr
    }
    #[doc = "0xd0 - AFEC Correction Select Register"]
    #[inline(always)]
    pub const fn cosr(&self) -> &Cosr {
        &self.cosr
    }
    #[doc = "0xd4 - AFEC Correction Values Register"]
    #[inline(always)]
    pub const fn cvr(&self) -> &Cvr {
        &self.cvr
    }
    #[doc = "0xd8 - AFEC Channel Error Correction Register"]
    #[inline(always)]
    pub const fn cecr(&self) -> &Cecr {
        &self.cecr
    }
    #[doc = "0xe4 - AFEC Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - AFEC Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "CR (w) register accessor: AFEC Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "AFEC Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: AFEC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "AFEC Mode Register"]
pub mod mr;
#[doc = "EMR (rw) register accessor: AFEC Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`] module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "AFEC Extended Mode Register"]
pub mod emr;
#[doc = "SEQ1R (rw) register accessor: AFEC Channel Sequence 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq1r`] module"]
#[doc(alias = "SEQ1R")]
pub type Seq1r = crate::Reg<seq1r::Seq1rSpec>;
#[doc = "AFEC Channel Sequence 1 Register"]
pub mod seq1r;
#[doc = "SEQ2R (rw) register accessor: AFEC Channel Sequence 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`seq2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq2r`] module"]
#[doc(alias = "SEQ2R")]
pub type Seq2r = crate::Reg<seq2r::Seq2rSpec>;
#[doc = "AFEC Channel Sequence 2 Register"]
pub mod seq2r;
#[doc = "CHER (w) register accessor: AFEC Channel Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cher`] module"]
#[doc(alias = "CHER")]
pub type Cher = crate::Reg<cher::CherSpec>;
#[doc = "AFEC Channel Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: AFEC Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdr`] module"]
#[doc(alias = "CHDR")]
pub type Chdr = crate::Reg<chdr::ChdrSpec>;
#[doc = "AFEC Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: AFEC Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`] module"]
#[doc(alias = "CHSR")]
pub type Chsr = crate::Reg<chsr::ChsrSpec>;
#[doc = "AFEC Channel Status Register"]
pub mod chsr;
#[doc = "LCDR (r) register accessor: AFEC Last Converted Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdr`] module"]
#[doc(alias = "LCDR")]
pub type Lcdr = crate::Reg<lcdr::LcdrSpec>;
#[doc = "AFEC Last Converted Data Register"]
pub mod lcdr;
#[doc = "IER (w) register accessor: AFEC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "AFEC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: AFEC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "AFEC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: AFEC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "AFEC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: AFEC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "AFEC Interrupt Status Register"]
pub mod isr;
#[doc = "OVER (r) register accessor: AFEC Overrun Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@over`] module"]
#[doc(alias = "OVER")]
pub type Over = crate::Reg<over::OverSpec>;
#[doc = "AFEC Overrun Status Register"]
pub mod over;
#[doc = "CWR (rw) register accessor: AFEC Compare Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwr`] module"]
#[doc(alias = "CWR")]
pub type Cwr = crate::Reg<cwr::CwrSpec>;
#[doc = "AFEC Compare Window Register"]
pub mod cwr;
#[doc = "CGR (rw) register accessor: AFEC Channel Gain Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgr`] module"]
#[doc(alias = "CGR")]
pub type Cgr = crate::Reg<cgr::CgrSpec>;
#[doc = "AFEC Channel Gain Register"]
pub mod cgr;
#[doc = "DIFFR (rw) register accessor: AFEC Channel Differential Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diffr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diffr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diffr`] module"]
#[doc(alias = "DIFFR")]
pub type Diffr = crate::Reg<diffr::DiffrSpec>;
#[doc = "AFEC Channel Differential Register"]
pub mod diffr;
#[doc = "CSELR (rw) register accessor: AFEC Channel Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cselr`] module"]
#[doc(alias = "CSELR")]
pub type Cselr = crate::Reg<cselr::CselrSpec>;
#[doc = "AFEC Channel Selection Register"]
pub mod cselr;
#[doc = "CDR (r) register accessor: AFEC Channel Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`] module"]
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
#[doc = "AFEC Channel Data Register"]
pub mod cdr;
#[doc = "COCR (rw) register accessor: AFEC Channel Offset Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cocr`] module"]
#[doc(alias = "COCR")]
pub type Cocr = crate::Reg<cocr::CocrSpec>;
#[doc = "AFEC Channel Offset Compensation Register"]
pub mod cocr;
#[doc = "TEMPMR (rw) register accessor: AFEC Temperature Sensor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tempmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tempmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempmr`] module"]
#[doc(alias = "TEMPMR")]
pub type Tempmr = crate::Reg<tempmr::TempmrSpec>;
#[doc = "AFEC Temperature Sensor Mode Register"]
pub mod tempmr;
#[doc = "TEMPCWR (rw) register accessor: AFEC Temperature Compare Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tempcwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tempcwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempcwr`] module"]
#[doc(alias = "TEMPCWR")]
pub type Tempcwr = crate::Reg<tempcwr::TempcwrSpec>;
#[doc = "AFEC Temperature Compare Window Register"]
pub mod tempcwr;
#[doc = "ACR (rw) register accessor: AFEC Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "AFEC Analog Control Register"]
pub mod acr;
#[doc = "SHMR (rw) register accessor: AFEC Sample & Hold Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shmr`] module"]
#[doc(alias = "SHMR")]
pub type Shmr = crate::Reg<shmr::ShmrSpec>;
#[doc = "AFEC Sample & Hold Mode Register"]
pub mod shmr;
#[doc = "COSR (rw) register accessor: AFEC Correction Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cosr`] module"]
#[doc(alias = "COSR")]
pub type Cosr = crate::Reg<cosr::CosrSpec>;
#[doc = "AFEC Correction Select Register"]
pub mod cosr;
#[doc = "CVR (rw) register accessor: AFEC Correction Values Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvr`] module"]
#[doc(alias = "CVR")]
pub type Cvr = crate::Reg<cvr::CvrSpec>;
#[doc = "AFEC Correction Values Register"]
pub mod cvr;
#[doc = "CECR (rw) register accessor: AFEC Channel Error Correction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cecr`] module"]
#[doc(alias = "CECR")]
pub type Cecr = crate::Reg<cecr::CecrSpec>;
#[doc = "AFEC Channel Error Correction Register"]
pub mod cecr;
#[doc = "WPMR (rw) register accessor: AFEC Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "AFEC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: AFEC Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "AFEC Write Protection Status Register"]
pub mod wpsr;
