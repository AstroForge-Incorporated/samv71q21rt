#[doc = "Register `CDTYUPD` writer"]
pub type W = crate::W<CdtyupdSpec>;
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub type CdtyupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    pub fn cdtyupd(&mut self) -> CdtyupdW<CdtyupdSpec> {
        CdtyupdW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdtyupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdtyupdSpec;
impl crate::RegisterSpec for CdtyupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdtyupd::W`](W) writer structure"]
impl crate::Writable for CdtyupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDTYUPD to value 0"]
impl crate::Resettable for CdtyupdSpec {}
