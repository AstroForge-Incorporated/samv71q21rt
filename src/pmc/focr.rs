#[doc = "Register `FOCR` writer"]
pub type W = crate::W<FocrSpec>;
#[doc = "Field `FOCLR` writer - Fault Output Clear"]
pub type FoclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline(always)]
    pub fn foclr(&mut self) -> FoclrW<FocrSpec> {
        FoclrW::new(self, 0)
    }
}
#[doc = "Fault Output Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`focr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FocrSpec;
impl crate::RegisterSpec for FocrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`focr::W`](W) writer structure"]
impl crate::Writable for FocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FOCR to value 0"]
impl crate::Resettable for FocrSpec {}
