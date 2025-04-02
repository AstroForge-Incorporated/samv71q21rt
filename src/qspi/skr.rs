#[doc = "Register `SKR` writer"]
pub type W = crate::W<SkrSpec>;
#[doc = "Field `USRK` writer - User Scrambling Key"]
pub type UsrkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - User Scrambling Key"]
    #[inline(always)]
    pub fn usrk(&mut self) -> UsrkW<SkrSpec> {
        UsrkW::new(self, 0)
    }
}
#[doc = "Scrambling Key Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SkrSpec;
impl crate::RegisterSpec for SkrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`skr::W`](W) writer structure"]
impl crate::Writable for SkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SkrSpec {}
