#[doc = "Register `DMA_CHDR` writer"]
pub type W = crate::W<DmaChdrSpec>;
#[doc = "Field `P_CH_DIS` writer - Preview Channel Disable Request"]
pub type PChDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CH_DIS` writer - Codec Channel Disable Request"]
pub type CChDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Preview Channel Disable Request"]
    #[inline(always)]
    pub fn p_ch_dis(&mut self) -> PChDisW<DmaChdrSpec> {
        PChDisW::new(self, 0)
    }
    #[doc = "Bit 1 - Codec Channel Disable Request"]
    #[inline(always)]
    pub fn c_ch_dis(&mut self) -> CChDisW<DmaChdrSpec> {
        CChDisW::new(self, 1)
    }
}
#[doc = "DMA Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaChdrSpec;
impl crate::RegisterSpec for DmaChdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_chdr::W`](W) writer structure"]
impl crate::Writable for DmaChdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CHDR to value 0"]
impl crate::Resettable for DmaChdrSpec {}
