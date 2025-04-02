#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_us_cr: [u8; 0x04],
    _reserved_1_us_mr: [u8; 0x04],
    _reserved_2_us_ier: [u8; 0x04],
    _reserved_3_us_idr: [u8; 0x04],
    _reserved_4_us_imr: [u8; 0x04],
    _reserved_5_us_csr: [u8; 0x04],
    us_rhr: UsRhr,
    us_thr: UsThr,
    us_brgr: UsBrgr,
    us_rtor: UsRtor,
    _reserved_10_us_ttgr: [u8; 0x04],
    _reserved11: [u8; 0x14],
    _reserved_11_us_fidi: [u8; 0x04],
    us_ner: UsNer,
    _reserved13: [u8; 0x04],
    us_if: UsIf,
    us_man: UsMan,
    us_linmr: UsLinmr,
    us_linir: UsLinir,
    us_linbrr: UsLinbrr,
    us_lonmr: UsLonmr,
    us_lonpr: UsLonpr,
    us_londl: UsLondl,
    us_lonl2hdr: UsLonl2hdr,
    us_lonbl: UsLonbl,
    us_lonb1tx: UsLonb1tx,
    us_lonb1rx: UsLonb1rx,
    us_lonprio: UsLonprio,
    us_idttx: UsIdttx,
    us_idtrx: UsIdtrx,
    us_icdiff: UsIcdiff,
    _reserved29: [u8; 0x58],
    us_wpmr: UsWpmr,
    us_wpsr: UsWpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn us_cr_lin_mode(&self) -> &UsCrLinMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn us_cr_spi_mode(&self) -> &UsCrSpiMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn us_cr_usart_mode(&self) -> &UsCrUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn us_mr_spi_mode(&self) -> &UsMrSpiMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn us_mr_usart_mode(&self) -> &UsMrUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn us_ier_lon_mode(&self) -> &UsIerLonMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn us_ier_lin_mode(&self) -> &UsIerLinMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn us_ier_spi_mode(&self) -> &UsIerSpiMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn us_ier_usart_mode(&self) -> &UsIerUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn us_idr_lon_mode(&self) -> &UsIdrLonMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn us_idr_lin_mode(&self) -> &UsIdrLinMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn us_idr_spi_mode(&self) -> &UsIdrSpiMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn us_idr_usart_mode(&self) -> &UsIdrUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn us_imr_lon_mode(&self) -> &UsImrLonMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn us_imr_lin_mode(&self) -> &UsImrLinMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn us_imr_spi_mode(&self) -> &UsImrSpiMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn us_imr_usart_mode(&self) -> &UsImrUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn us_csr_lon_mode(&self) -> &UsCsrLonMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn us_csr_lin_mode(&self) -> &UsCsrLinMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn us_csr_spi_mode(&self) -> &UsCsrSpiMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn us_csr_usart_mode(&self) -> &UsCsrUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Receive Holding Register"]
    #[inline(always)]
    pub const fn us_rhr(&self) -> &UsRhr {
        &self.us_rhr
    }
    #[doc = "0x1c - Transmit Holding Register"]
    #[inline(always)]
    pub const fn us_thr(&self) -> &UsThr {
        &self.us_thr
    }
    #[doc = "0x20 - Baud Rate Generator Register"]
    #[inline(always)]
    pub const fn us_brgr(&self) -> &UsBrgr {
        &self.us_brgr
    }
    #[doc = "0x24 - Receiver Timeout Register"]
    #[inline(always)]
    pub const fn us_rtor(&self) -> &UsRtor {
        &self.us_rtor
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub const fn us_ttgr_lon_mode(&self) -> &UsTtgrLonMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub const fn us_ttgr_usart_mode(&self) -> &UsTtgrUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x40 - FI DI Ratio Register"]
    #[inline(always)]
    pub const fn us_fidi_lon_mode(&self) -> &UsFidiLonMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - FI DI Ratio Register"]
    #[inline(always)]
    pub const fn us_fidi_usart_mode(&self) -> &UsFidiUsartMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - Number of Errors Register"]
    #[inline(always)]
    pub const fn us_ner(&self) -> &UsNer {
        &self.us_ner
    }
    #[doc = "0x4c - IrDA Filter Register"]
    #[inline(always)]
    pub const fn us_if(&self) -> &UsIf {
        &self.us_if
    }
    #[doc = "0x50 - Manchester Configuration Register"]
    #[inline(always)]
    pub const fn us_man(&self) -> &UsMan {
        &self.us_man
    }
    #[doc = "0x54 - LIN Mode Register"]
    #[inline(always)]
    pub const fn us_linmr(&self) -> &UsLinmr {
        &self.us_linmr
    }
    #[doc = "0x58 - LIN Identifier Register"]
    #[inline(always)]
    pub const fn us_linir(&self) -> &UsLinir {
        &self.us_linir
    }
    #[doc = "0x5c - LIN Baud Rate Register"]
    #[inline(always)]
    pub const fn us_linbrr(&self) -> &UsLinbrr {
        &self.us_linbrr
    }
    #[doc = "0x60 - LON Mode Register"]
    #[inline(always)]
    pub const fn us_lonmr(&self) -> &UsLonmr {
        &self.us_lonmr
    }
    #[doc = "0x64 - LON Preamble Register"]
    #[inline(always)]
    pub const fn us_lonpr(&self) -> &UsLonpr {
        &self.us_lonpr
    }
    #[doc = "0x68 - LON Data Length Register"]
    #[inline(always)]
    pub const fn us_londl(&self) -> &UsLondl {
        &self.us_londl
    }
    #[doc = "0x6c - LON L2HDR Register"]
    #[inline(always)]
    pub const fn us_lonl2hdr(&self) -> &UsLonl2hdr {
        &self.us_lonl2hdr
    }
    #[doc = "0x70 - LON Backlog Register"]
    #[inline(always)]
    pub const fn us_lonbl(&self) -> &UsLonbl {
        &self.us_lonbl
    }
    #[doc = "0x74 - LON Beta1 Tx Register"]
    #[inline(always)]
    pub const fn us_lonb1tx(&self) -> &UsLonb1tx {
        &self.us_lonb1tx
    }
    #[doc = "0x78 - LON Beta1 Rx Register"]
    #[inline(always)]
    pub const fn us_lonb1rx(&self) -> &UsLonb1rx {
        &self.us_lonb1rx
    }
    #[doc = "0x7c - LON Priority Register"]
    #[inline(always)]
    pub const fn us_lonprio(&self) -> &UsLonprio {
        &self.us_lonprio
    }
    #[doc = "0x80 - LON IDT Tx Register"]
    #[inline(always)]
    pub const fn us_idttx(&self) -> &UsIdttx {
        &self.us_idttx
    }
    #[doc = "0x84 - LON IDT Rx Register"]
    #[inline(always)]
    pub const fn us_idtrx(&self) -> &UsIdtrx {
        &self.us_idtrx
    }
    #[doc = "0x88 - IC DIFF Register"]
    #[inline(always)]
    pub const fn us_icdiff(&self) -> &UsIcdiff {
        &self.us_icdiff
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn us_wpmr(&self) -> &UsWpmr {
        &self.us_wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn us_wpsr(&self) -> &UsWpsr {
        &self.us_wpsr
    }
}
#[doc = "US_CR_USART_MODE (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_cr_usart_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_cr_usart_mode`] module"]
#[doc(alias = "US_CR_USART_MODE")]
pub type UsCrUsartMode = crate::Reg<us_cr_usart_mode::UsCrUsartModeSpec>;
#[doc = "Control Register"]
pub mod us_cr_usart_mode;
#[doc = "US_CR_SPI_MODE (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_cr_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_cr_spi_mode`] module"]
#[doc(alias = "US_CR_SPI_MODE")]
pub type UsCrSpiMode = crate::Reg<us_cr_spi_mode::UsCrSpiModeSpec>;
#[doc = "Control Register"]
pub mod us_cr_spi_mode;
#[doc = "US_CR_LIN_MODE (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_cr_lin_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_cr_lin_mode`] module"]
#[doc(alias = "US_CR_LIN_MODE")]
pub type UsCrLinMode = crate::Reg<us_cr_lin_mode::UsCrLinModeSpec>;
#[doc = "Control Register"]
pub mod us_cr_lin_mode;
#[doc = "US_MR_USART_MODE (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_mr_usart_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_mr_usart_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_mr_usart_mode`] module"]
#[doc(alias = "US_MR_USART_MODE")]
pub type UsMrUsartMode = crate::Reg<us_mr_usart_mode::UsMrUsartModeSpec>;
#[doc = "Mode Register"]
pub mod us_mr_usart_mode;
#[doc = "US_MR_SPI_MODE (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_mr_spi_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_mr_spi_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_mr_spi_mode`] module"]
#[doc(alias = "US_MR_SPI_MODE")]
pub type UsMrSpiMode = crate::Reg<us_mr_spi_mode::UsMrSpiModeSpec>;
#[doc = "Mode Register"]
pub mod us_mr_spi_mode;
#[doc = "US_IER_USART_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_usart_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ier_usart_mode`] module"]
#[doc(alias = "US_IER_USART_MODE")]
pub type UsIerUsartMode = crate::Reg<us_ier_usart_mode::UsIerUsartModeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod us_ier_usart_mode;
#[doc = "US_IER_SPI_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ier_spi_mode`] module"]
#[doc(alias = "US_IER_SPI_MODE")]
pub type UsIerSpiMode = crate::Reg<us_ier_spi_mode::UsIerSpiModeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod us_ier_spi_mode;
#[doc = "US_IER_LIN_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_lin_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ier_lin_mode`] module"]
#[doc(alias = "US_IER_LIN_MODE")]
pub type UsIerLinMode = crate::Reg<us_ier_lin_mode::UsIerLinModeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod us_ier_lin_mode;
#[doc = "US_IER_LON_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_lon_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ier_lon_mode`] module"]
#[doc(alias = "US_IER_LON_MODE")]
pub type UsIerLonMode = crate::Reg<us_ier_lon_mode::UsIerLonModeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod us_ier_lon_mode;
#[doc = "US_IDR_USART_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idr_usart_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_idr_usart_mode`] module"]
#[doc(alias = "US_IDR_USART_MODE")]
pub type UsIdrUsartMode = crate::Reg<us_idr_usart_mode::UsIdrUsartModeSpec>;
#[doc = "Interrupt Disable Register"]
pub mod us_idr_usart_mode;
#[doc = "US_IDR_SPI_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idr_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_idr_spi_mode`] module"]
#[doc(alias = "US_IDR_SPI_MODE")]
pub type UsIdrSpiMode = crate::Reg<us_idr_spi_mode::UsIdrSpiModeSpec>;
#[doc = "Interrupt Disable Register"]
pub mod us_idr_spi_mode;
#[doc = "US_IDR_LIN_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idr_lin_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_idr_lin_mode`] module"]
#[doc(alias = "US_IDR_LIN_MODE")]
pub type UsIdrLinMode = crate::Reg<us_idr_lin_mode::UsIdrLinModeSpec>;
#[doc = "Interrupt Disable Register"]
pub mod us_idr_lin_mode;
#[doc = "US_IDR_LON_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idr_lon_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_idr_lon_mode`] module"]
#[doc(alias = "US_IDR_LON_MODE")]
pub type UsIdrLonMode = crate::Reg<us_idr_lon_mode::UsIdrLonModeSpec>;
#[doc = "Interrupt Disable Register"]
pub mod us_idr_lon_mode;
#[doc = "US_IMR_USART_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_imr_usart_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_imr_usart_mode`] module"]
#[doc(alias = "US_IMR_USART_MODE")]
pub type UsImrUsartMode = crate::Reg<us_imr_usart_mode::UsImrUsartModeSpec>;
#[doc = "Interrupt Mask Register"]
pub mod us_imr_usart_mode;
#[doc = "US_IMR_SPI_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_imr_spi_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_imr_spi_mode`] module"]
#[doc(alias = "US_IMR_SPI_MODE")]
pub type UsImrSpiMode = crate::Reg<us_imr_spi_mode::UsImrSpiModeSpec>;
#[doc = "Interrupt Mask Register"]
pub mod us_imr_spi_mode;
#[doc = "US_IMR_LIN_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_imr_lin_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_imr_lin_mode`] module"]
#[doc(alias = "US_IMR_LIN_MODE")]
pub type UsImrLinMode = crate::Reg<us_imr_lin_mode::UsImrLinModeSpec>;
#[doc = "Interrupt Mask Register"]
pub mod us_imr_lin_mode;
#[doc = "US_IMR_LON_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_imr_lon_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_imr_lon_mode`] module"]
#[doc(alias = "US_IMR_LON_MODE")]
pub type UsImrLonMode = crate::Reg<us_imr_lon_mode::UsImrLonModeSpec>;
#[doc = "Interrupt Mask Register"]
pub mod us_imr_lon_mode;
#[doc = "US_CSR_USART_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_usart_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_csr_usart_mode`] module"]
#[doc(alias = "US_CSR_USART_MODE")]
pub type UsCsrUsartMode = crate::Reg<us_csr_usart_mode::UsCsrUsartModeSpec>;
#[doc = "Channel Status Register"]
pub mod us_csr_usart_mode;
#[doc = "US_CSR_SPI_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_spi_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_csr_spi_mode`] module"]
#[doc(alias = "US_CSR_SPI_MODE")]
pub type UsCsrSpiMode = crate::Reg<us_csr_spi_mode::UsCsrSpiModeSpec>;
#[doc = "Channel Status Register"]
pub mod us_csr_spi_mode;
#[doc = "US_CSR_LIN_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_lin_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_csr_lin_mode`] module"]
#[doc(alias = "US_CSR_LIN_MODE")]
pub type UsCsrLinMode = crate::Reg<us_csr_lin_mode::UsCsrLinModeSpec>;
#[doc = "Channel Status Register"]
pub mod us_csr_lin_mode;
#[doc = "US_CSR_LON_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_lon_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_csr_lon_mode`] module"]
#[doc(alias = "US_CSR_LON_MODE")]
pub type UsCsrLonMode = crate::Reg<us_csr_lon_mode::UsCsrLonModeSpec>;
#[doc = "Channel Status Register"]
pub mod us_csr_lon_mode;
#[doc = "US_RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_rhr`] module"]
#[doc(alias = "US_RHR")]
pub type UsRhr = crate::Reg<us_rhr::UsRhrSpec>;
#[doc = "Receive Holding Register"]
pub mod us_rhr;
#[doc = "US_THR (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_thr`] module"]
#[doc(alias = "US_THR")]
pub type UsThr = crate::Reg<us_thr::UsThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod us_thr;
#[doc = "US_BRGR (rw) register accessor: Baud Rate Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_brgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_brgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_brgr`] module"]
#[doc(alias = "US_BRGR")]
pub type UsBrgr = crate::Reg<us_brgr::UsBrgrSpec>;
#[doc = "Baud Rate Generator Register"]
pub mod us_brgr;
#[doc = "US_RTOR (rw) register accessor: Receiver Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_rtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_rtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_rtor`] module"]
#[doc(alias = "US_RTOR")]
pub type UsRtor = crate::Reg<us_rtor::UsRtorSpec>;
#[doc = "Receiver Timeout Register"]
pub mod us_rtor;
#[doc = "US_TTGR_USART_MODE (rw) register accessor: Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_ttgr_usart_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ttgr_usart_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ttgr_usart_mode`] module"]
#[doc(alias = "US_TTGR_USART_MODE")]
pub type UsTtgrUsartMode = crate::Reg<us_ttgr_usart_mode::UsTtgrUsartModeSpec>;
#[doc = "Transmitter Timeguard Register"]
pub mod us_ttgr_usart_mode;
#[doc = "US_TTGR_LON_MODE (rw) register accessor: Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_ttgr_lon_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ttgr_lon_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ttgr_lon_mode`] module"]
#[doc(alias = "US_TTGR_LON_MODE")]
pub type UsTtgrLonMode = crate::Reg<us_ttgr_lon_mode::UsTtgrLonModeSpec>;
#[doc = "Transmitter Timeguard Register"]
pub mod us_ttgr_lon_mode;
#[doc = "US_FIDI_USART_MODE (rw) register accessor: FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_fidi_usart_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_fidi_usart_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_fidi_usart_mode`] module"]
#[doc(alias = "US_FIDI_USART_MODE")]
pub type UsFidiUsartMode = crate::Reg<us_fidi_usart_mode::UsFidiUsartModeSpec>;
#[doc = "FI DI Ratio Register"]
pub mod us_fidi_usart_mode;
#[doc = "US_FIDI_LON_MODE (rw) register accessor: FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_fidi_lon_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_fidi_lon_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_fidi_lon_mode`] module"]
#[doc(alias = "US_FIDI_LON_MODE")]
pub type UsFidiLonMode = crate::Reg<us_fidi_lon_mode::UsFidiLonModeSpec>;
#[doc = "FI DI Ratio Register"]
pub mod us_fidi_lon_mode;
#[doc = "US_NER (r) register accessor: Number of Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_ner::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_ner`] module"]
#[doc(alias = "US_NER")]
pub type UsNer = crate::Reg<us_ner::UsNerSpec>;
#[doc = "Number of Errors Register"]
pub mod us_ner;
#[doc = "US_IF (rw) register accessor: IrDA Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_if`] module"]
#[doc(alias = "US_IF")]
pub type UsIf = crate::Reg<us_if::UsIfSpec>;
#[doc = "IrDA Filter Register"]
pub mod us_if;
#[doc = "US_MAN (rw) register accessor: Manchester Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_man::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_man::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_man`] module"]
#[doc(alias = "US_MAN")]
pub type UsMan = crate::Reg<us_man::UsManSpec>;
#[doc = "Manchester Configuration Register"]
pub mod us_man;
#[doc = "US_LINMR (rw) register accessor: LIN Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_linmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_linmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_linmr`] module"]
#[doc(alias = "US_LINMR")]
pub type UsLinmr = crate::Reg<us_linmr::UsLinmrSpec>;
#[doc = "LIN Mode Register"]
pub mod us_linmr;
#[doc = "US_LINIR (rw) register accessor: LIN Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_linir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_linir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_linir`] module"]
#[doc(alias = "US_LINIR")]
pub type UsLinir = crate::Reg<us_linir::UsLinirSpec>;
#[doc = "LIN Identifier Register"]
pub mod us_linir;
#[doc = "US_LINBRR (r) register accessor: LIN Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_linbrr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_linbrr`] module"]
#[doc(alias = "US_LINBRR")]
pub type UsLinbrr = crate::Reg<us_linbrr::UsLinbrrSpec>;
#[doc = "LIN Baud Rate Register"]
pub mod us_linbrr;
#[doc = "US_LONMR (rw) register accessor: LON Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonmr`] module"]
#[doc(alias = "US_LONMR")]
pub type UsLonmr = crate::Reg<us_lonmr::UsLonmrSpec>;
#[doc = "LON Mode Register"]
pub mod us_lonmr;
#[doc = "US_LONPR (rw) register accessor: LON Preamble Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonpr`] module"]
#[doc(alias = "US_LONPR")]
pub type UsLonpr = crate::Reg<us_lonpr::UsLonprSpec>;
#[doc = "LON Preamble Register"]
pub mod us_lonpr;
#[doc = "US_LONDL (rw) register accessor: LON Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_londl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_londl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_londl`] module"]
#[doc(alias = "US_LONDL")]
pub type UsLondl = crate::Reg<us_londl::UsLondlSpec>;
#[doc = "LON Data Length Register"]
pub mod us_londl;
#[doc = "US_LONL2HDR (rw) register accessor: LON L2HDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonl2hdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonl2hdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonl2hdr`] module"]
#[doc(alias = "US_LONL2HDR")]
pub type UsLonl2hdr = crate::Reg<us_lonl2hdr::UsLonl2hdrSpec>;
#[doc = "LON L2HDR Register"]
pub mod us_lonl2hdr;
#[doc = "US_LONBL (r) register accessor: LON Backlog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonbl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonbl`] module"]
#[doc(alias = "US_LONBL")]
pub type UsLonbl = crate::Reg<us_lonbl::UsLonblSpec>;
#[doc = "LON Backlog Register"]
pub mod us_lonbl;
#[doc = "US_LONB1TX (rw) register accessor: LON Beta1 Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonb1tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonb1tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonb1tx`] module"]
#[doc(alias = "US_LONB1TX")]
pub type UsLonb1tx = crate::Reg<us_lonb1tx::UsLonb1txSpec>;
#[doc = "LON Beta1 Tx Register"]
pub mod us_lonb1tx;
#[doc = "US_LONB1RX (rw) register accessor: LON Beta1 Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonb1rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonb1rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonb1rx`] module"]
#[doc(alias = "US_LONB1RX")]
pub type UsLonb1rx = crate::Reg<us_lonb1rx::UsLonb1rxSpec>;
#[doc = "LON Beta1 Rx Register"]
pub mod us_lonb1rx;
#[doc = "US_LONPRIO (rw) register accessor: LON Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_lonprio`] module"]
#[doc(alias = "US_LONPRIO")]
pub type UsLonprio = crate::Reg<us_lonprio::UsLonprioSpec>;
#[doc = "LON Priority Register"]
pub mod us_lonprio;
#[doc = "US_IDTTX (rw) register accessor: LON IDT Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_idttx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idttx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_idttx`] module"]
#[doc(alias = "US_IDTTX")]
pub type UsIdttx = crate::Reg<us_idttx::UsIdttxSpec>;
#[doc = "LON IDT Tx Register"]
pub mod us_idttx;
#[doc = "US_IDTRX (rw) register accessor: LON IDT Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_idtrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idtrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_idtrx`] module"]
#[doc(alias = "US_IDTRX")]
pub type UsIdtrx = crate::Reg<us_idtrx::UsIdtrxSpec>;
#[doc = "LON IDT Rx Register"]
pub mod us_idtrx;
#[doc = "US_ICDIFF (rw) register accessor: IC DIFF Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_icdiff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_icdiff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_icdiff`] module"]
#[doc(alias = "US_ICDIFF")]
pub type UsIcdiff = crate::Reg<us_icdiff::UsIcdiffSpec>;
#[doc = "IC DIFF Register"]
pub mod us_icdiff;
#[doc = "US_WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_wpmr`] module"]
#[doc(alias = "US_WPMR")]
pub type UsWpmr = crate::Reg<us_wpmr::UsWpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod us_wpmr;
#[doc = "US_WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@us_wpsr`] module"]
#[doc(alias = "US_WPSR")]
pub type UsWpsr = crate::Reg<us_wpsr::UsWpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod us_wpsr;
