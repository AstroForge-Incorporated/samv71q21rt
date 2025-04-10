#[doc = "Register `CTRR` reader"]
pub type R = crate::R<CtrrSpec>;
#[doc = "Field `CTR` reader - GCM Encryption Counter"]
pub type CtrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Encryption Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(self.bits)
    }
}
#[doc = "GCM Encryption Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrrSpec;
impl crate::RegisterSpec for CtrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrr::R`](R) reader structure"]
impl crate::Readable for CtrrSpec {}
#[doc = "`reset()` method sets CTRR to value 0"]
impl crate::Resettable for CtrrSpec {}
