#[repr(C)]
#[doc = "Specific Address 1 Bottom Register"]
#[doc(alias = "GMAC_SA")]
pub struct GmacSa {
    sab: Sab,
    sat: Sat,
}
impl GmacSa {
    #[doc = "0x00 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub const fn sab(&self) -> &Sab {
        &self.sab
    }
    #[doc = "0x04 - Specific Address 1 Top Register"]
    #[inline(always)]
    pub const fn sat(&self) -> &Sat {
        &self.sat
    }
}
#[doc = "SAB (rw) register accessor: Specific Address 1 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sab`] module"]
#[doc(alias = "SAB")]
pub type Sab = crate::Reg<sab::SabSpec>;
#[doc = "Specific Address 1 Bottom Register"]
pub mod sab;
#[doc = "SAT (rw) register accessor: Specific Address 1 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sat`] module"]
#[doc(alias = "SAT")]
pub type Sat = crate::Reg<sat::SatSpec>;
#[doc = "Specific Address 1 Top Register"]
pub mod sat;
