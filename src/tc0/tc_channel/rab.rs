#[doc = "Register `RAB` reader"]
pub type R = crate::R<RabSpec>;
#[doc = "Field `RAB` reader - Register A or Register B"]
pub type RabR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RabR {
        RabR::new(self.bits)
    }
}
#[doc = "Register AB (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rab::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RabSpec;
impl crate::RegisterSpec for RabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rab::R`](R) reader structure"]
impl crate::Readable for RabSpec {}
#[doc = "`reset()` method sets RAB to value 0"]
impl crate::Resettable for RabSpec {}
