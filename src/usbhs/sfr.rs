#[doc = "Register `SFR` writer"]
pub type W = crate::W<SfrSpec>;
#[doc = "Field `RDERRIS` writer - Remote Device Connection Error Interrupt Set"]
pub type RderrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSRQS` writer - VBUS Request Set"]
pub type VbusrqsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Set"]
    #[inline(always)]
    pub fn rderris(&mut self) -> RderrisW<SfrSpec> {
        RderrisW::new(self, 4)
    }
    #[doc = "Bit 9 - VBUS Request Set"]
    #[inline(always)]
    pub fn vbusrqs(&mut self) -> VbusrqsW<SfrSpec> {
        VbusrqsW::new(self, 9)
    }
}
#[doc = "General Status Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSpec;
impl crate::RegisterSpec for SfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR to value 0"]
impl crate::Resettable for SfrSpec {}
