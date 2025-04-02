#[doc = "Register `CPRDUPD` writer"]
pub type W = crate::W<CprdupdSpec>;
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CprdupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    pub fn cprdupd(&mut self) -> CprdupdW<CprdupdSpec> {
        CprdupdW::new(self, 0)
    }
}
#[doc = "PWM Channel Period Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cprdupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprdupdSpec;
impl crate::RegisterSpec for CprdupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cprdupd::W`](W) writer structure"]
impl crate::Writable for CprdupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPRDUPD to value 0"]
impl crate::Resettable for CprdupdSpec {}
