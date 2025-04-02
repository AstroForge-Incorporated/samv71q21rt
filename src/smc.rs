#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    smc_cs_number: [SmcCsNumber; 4],
    _reserved1: [u8; 0x40],
    ocms: Ocms,
    key1: Key1,
    key2: Key2,
    _reserved4: [u8; 0x58],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - SMC Setup Register"]
    #[inline(always)]
    pub const fn smc_cs_number(&self, n: usize) -> &SmcCsNumber {
        &self.smc_cs_number[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - SMC Setup Register"]
    #[inline(always)]
    pub fn smc_cs_number_iter(&self) -> impl Iterator<Item = &SmcCsNumber> {
        self.smc_cs_number.iter()
    }
    #[doc = "0x80 - SMC Off-Chip Memory Scrambling Register"]
    #[inline(always)]
    pub const fn ocms(&self) -> &Ocms {
        &self.ocms
    }
    #[doc = "0x84 - SMC Off-Chip Memory Scrambling KEY1 Register"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x88 - SMC Off-Chip Memory Scrambling KEY2 Register"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "SMC Setup Register"]
pub use self::smc_cs_number::SmcCsNumber;
#[doc = r"Cluster"]
#[doc = "SMC Setup Register"]
pub mod smc_cs_number;
#[doc = "OCMS (rw) register accessor: SMC Off-Chip Memory Scrambling Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`] module"]
#[doc(alias = "OCMS")]
pub type Ocms = crate::Reg<ocms::OcmsSpec>;
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: SMC Off-Chip Memory Scrambling KEY1 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`] module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: SMC Off-Chip Memory Scrambling KEY2 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`] module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub mod key2;
#[doc = "WPMR (rw) register accessor: SMC Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "SMC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: SMC Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "SMC Write Protection Status Register"]
pub mod wpsr;
