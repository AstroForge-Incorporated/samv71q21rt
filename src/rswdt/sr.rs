#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `WDUNF` reader - Watchdog Underflow"]
pub type WdunfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog Underflow"]
    #[inline(always)]
    pub fn wdunf(&self) -> WdunfR {
        WdunfR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
