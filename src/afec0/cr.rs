#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` writer - Start Conversion"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CrSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Conversion"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 1)
    }
}
#[doc = "AFEC Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
