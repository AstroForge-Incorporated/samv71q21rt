#[doc = "Register `DMA_CHER` writer"]
pub type W = crate::W<DmaCherSpec>;
#[doc = "Field `P_CH_EN` writer - Preview Channel Enable"]
pub type PChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CH_EN` writer - Codec Channel Enable"]
pub type CChEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Preview Channel Enable"]
    #[inline(always)]
    pub fn p_ch_en(&mut self) -> PChEnW<DmaCherSpec> {
        PChEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Codec Channel Enable"]
    #[inline(always)]
    pub fn c_ch_en(&mut self) -> CChEnW<DmaCherSpec> {
        CChEnW::new(self, 1)
    }
}
#[doc = "DMA Channel Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCherSpec;
impl crate::RegisterSpec for DmaCherSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_cher::W`](W) writer structure"]
impl crate::Writable for DmaCherSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CHER to value 0"]
impl crate::Resettable for DmaCherSpec {}
