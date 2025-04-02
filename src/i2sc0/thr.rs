#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `THR` writer - Transmitter Holding Register"]
pub type ThrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmitter Holding Register"]
    #[inline(always)]
    pub fn thr(&mut self) -> ThrW<ThrSpec> {
        ThrW::new(self, 0)
    }
}
#[doc = "Transmitter Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {}
