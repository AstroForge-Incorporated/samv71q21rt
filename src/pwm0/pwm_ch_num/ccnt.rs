#[doc = "Register `CCNT` reader"]
pub type R = crate::R<CcntSpec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PWM Channel Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcntSpec;
impl crate::RegisterSpec for CcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt::R`](R) reader structure"]
impl crate::Readable for CcntSpec {}
#[doc = "`reset()` method sets CCNT to value 0"]
impl crate::Resettable for CcntSpec {}
