#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    word0: Word0,
    word1: Word1,
    word2: Word2,
    word3: Word3,
}
impl RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    #[inline(always)]
    pub const fn word0(&self) -> &Word0 {
        &self.word0
    }
    #[doc = "0x04 - Lock Bits Word 1"]
    #[inline(always)]
    pub const fn word1(&self) -> &Word1 {
        &self.word1
    }
    #[doc = "0x08 - Lock Bits Word 2"]
    #[inline(always)]
    pub const fn word2(&self) -> &Word2 {
        &self.word2
    }
    #[doc = "0x0c - Lock Bits Word 3"]
    #[inline(always)]
    pub const fn word3(&self) -> &Word3 {
        &self.word3
    }
}
#[doc = "WORD0 (rw) register accessor: Lock Bits Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word0`] module"]
#[doc(alias = "WORD0")]
pub type Word0 = crate::Reg<word0::Word0Spec>;
#[doc = "Lock Bits Word 0"]
pub mod word0;
#[doc = "WORD1 (rw) register accessor: Lock Bits Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word1`] module"]
#[doc(alias = "WORD1")]
pub type Word1 = crate::Reg<word1::Word1Spec>;
#[doc = "Lock Bits Word 1"]
pub mod word1;
#[doc = "WORD2 (rw) register accessor: Lock Bits Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word2`] module"]
#[doc(alias = "WORD2")]
pub type Word2 = crate::Reg<word2::Word2Spec>;
#[doc = "Lock Bits Word 2"]
pub mod word2;
#[doc = "WORD3 (rw) register accessor: Lock Bits Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word3`] module"]
#[doc(alias = "WORD3")]
pub type Word3 = crate::Reg<word3::Word3Spec>;
#[doc = "Lock Bits Word 3"]
pub mod word3;
