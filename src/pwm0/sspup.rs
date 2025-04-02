#[doc = "Register `SSPUP` writer"]
pub type W = crate::W<SspupSpec>;
#[doc = "Field `SPRDUP` writer - Spread Spectrum Limit Value Update"]
pub type SprdupW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value Update"]
    #[inline(always)]
    pub fn sprdup(&mut self) -> SprdupW<SspupSpec> {
        SprdupW::new(self, 0)
    }
}
#[doc = "PWM Spread Spectrum Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspupSpec;
impl crate::RegisterSpec for SspupSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sspup::W`](W) writer structure"]
impl crate::Writable for SspupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSPUP to value 0"]
impl crate::Resettable for SspupSpec {}
