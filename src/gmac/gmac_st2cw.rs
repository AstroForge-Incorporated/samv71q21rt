#[repr(C)]
#[doc = "Screening Type 2 Compare Word 0 Register"]
#[doc(alias = "GMAC_ST2CW")]
pub struct GmacSt2cw {
    st2cw0: St2cw0,
    st2cw1: St2cw1,
}
impl GmacSt2cw {
    #[doc = "0x00 - Screening Type 2 Compare Word 0 Register"]
    #[inline(always)]
    pub const fn st2cw0(&self) -> &St2cw0 {
        &self.st2cw0
    }
    #[doc = "0x04 - Screening Type 2 Compare Word 1 Register"]
    #[inline(always)]
    pub const fn st2cw1(&self) -> &St2cw1 {
        &self.st2cw1
    }
}
#[doc = "ST2CW0 (rw) register accessor: Screening Type 2 Compare Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2cw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2cw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2cw0`] module"]
#[doc(alias = "ST2CW0")]
pub type St2cw0 = crate::Reg<st2cw0::St2cw0Spec>;
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod st2cw0;
#[doc = "ST2CW1 (rw) register accessor: Screening Type 2 Compare Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2cw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2cw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2cw1`] module"]
#[doc(alias = "ST2CW1")]
pub type St2cw1 = crate::Reg<st2cw1::St2cw1Spec>;
#[doc = "Screening Type 2 Compare Word 1 Register"]
pub mod st2cw1;
