#[doc = "Register `CV` reader"]
pub type R = crate::R<CvSpec>;
#[doc = "Field `CV` reader - Counter Value"]
pub type CvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvSpec;
impl crate::RegisterSpec for CvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv::R`](R) reader structure"]
impl crate::Readable for CvSpec {}
#[doc = "`reset()` method sets CV to value 0"]
impl crate::Resettable for CvSpec {}
