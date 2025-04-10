#[doc = "Register `CHDR` writer"]
pub type W = crate::W<ChdrSpec>;
#[doc = "Field `CH0` writer - Channel 0 Disable"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Disable"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<ChdrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<ChdrSpec> {
        Ch1W::new(self, 1)
    }
}
#[doc = "Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdrSpec;
impl crate::RegisterSpec for ChdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for ChdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDR to value 0"]
impl crate::Resettable for ChdrSpec {}
