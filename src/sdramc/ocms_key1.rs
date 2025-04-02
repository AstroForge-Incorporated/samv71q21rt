#[doc = "Register `OCMS_KEY1` writer"]
pub type W = crate::W<OcmsKey1Spec>;
#[doc = "Field `KEY1` writer - Off-chip Memory Scrambling (OCMS) Key Part 1"]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Off-chip Memory Scrambling (OCMS) Key Part 1"]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<OcmsKey1Spec> {
        Key1W::new(self, 0)
    }
}
#[doc = "SDRAMC OCMS KEY1 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms_key1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmsKey1Spec;
impl crate::RegisterSpec for OcmsKey1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ocms_key1::W`](W) writer structure"]
impl crate::Writable for OcmsKey1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCMS_KEY1 to value 0"]
impl crate::Resettable for OcmsKey1Spec {}
