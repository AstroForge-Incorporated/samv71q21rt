#[doc = "Register `RHR` reader"]
pub type R = crate::R<RhrSpec>;
#[doc = "Field `RDAT` reader - Receive Data"]
pub type RdatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data"]
    #[inline(always)]
    pub fn rdat(&self) -> RdatR {
        RdatR::new(self.bits)
    }
}
#[doc = "Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RhrSpec;
impl crate::RegisterSpec for RhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rhr::R`](R) reader structure"]
impl crate::Readable for RhrSpec {}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RhrSpec {}
