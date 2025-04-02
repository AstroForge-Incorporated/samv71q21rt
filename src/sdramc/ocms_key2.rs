#[doc = "Register `OCMS_KEY2` writer"]
pub type W = crate::W<OcmsKey2Spec>;
#[doc = "Field `KEY2` writer - Off-chip Memory Scrambling (OCMS) Key Part 2"]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Off-chip Memory Scrambling (OCMS) Key Part 2"]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<OcmsKey2Spec> {
        Key2W::new(self, 0)
    }
}
#[doc = "SDRAMC OCMS KEY2 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms_key2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmsKey2Spec;
impl crate::RegisterSpec for OcmsKey2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ocms_key2::W`](W) writer structure"]
impl crate::Writable for OcmsKey2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCMS_KEY2 to value 0"]
impl crate::Resettable for OcmsKey2Spec {}
