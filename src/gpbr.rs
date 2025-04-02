#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_gpbr: [SysGpbr; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - General Purpose Backup Register 0"]
    #[inline(always)]
    pub const fn sys_gpbr(&self, n: usize) -> &SysGpbr {
        &self.sys_gpbr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - General Purpose Backup Register 0"]
    #[inline(always)]
    pub fn sys_gpbr_iter(&self) -> impl Iterator<Item = &SysGpbr> {
        self.sys_gpbr.iter()
    }
}
#[doc = "SYS_GPBR (rw) register accessor: General Purpose Backup Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_gpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_gpbr`] module"]
#[doc(alias = "SYS_GPBR")]
pub type SysGpbr = crate::Reg<sys_gpbr::SysGpbrSpec>;
#[doc = "General Purpose Backup Register 0"]
pub mod sys_gpbr;
