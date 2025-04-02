#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `DATRDY` writer - Data Ready Interrupt Enable"]
pub type DatrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URAD` writer - Unspecified Register Access Detection Interrupt Enable"]
pub type UradW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAGRDY` writer - GCM Tag Ready Interrupt Enable"]
pub type TagrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn datrdy(&mut self) -> DatrdyW<IerSpec> {
        DatrdyW::new(self, 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Enable"]
    #[inline(always)]
    pub fn urad(&mut self) -> UradW<IerSpec> {
        UradW::new(self, 8)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Enable"]
    #[inline(always)]
    pub fn tagrdy(&mut self) -> TagrdyW<IerSpec> {
        TagrdyW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
