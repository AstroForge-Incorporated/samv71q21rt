#[repr(C)]
#[doc = "Priority Register A for Slave 0"]
#[doc(alias = "MATRIX_PR")]
pub struct MatrixPr {
    pras: Pras,
    prbs: Prbs,
}
impl MatrixPr {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn pras(&self) -> &Pras {
        &self.pras
    }
    #[doc = "0x04 - Priority Register B for Slave 0"]
    #[inline(always)]
    pub const fn prbs(&self) -> &Prbs {
        &self.prbs
    }
}
#[doc = "PRAS (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras`] module"]
#[doc(alias = "PRAS")]
pub type Pras = crate::Reg<pras::PrasSpec>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: Priority Register B for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prbs`] module"]
#[doc(alias = "PRBS")]
pub type Prbs = crate::Reg<prbs::PrbsSpec>;
#[doc = "Priority Register B for Slave 0"]
pub mod prbs;
