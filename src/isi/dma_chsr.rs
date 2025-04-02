#[doc = "Register `DMA_CHSR` reader"]
pub type R = crate::R<DmaChsrSpec>;
#[doc = "Field `P_CH_S` reader - Preview DMA Channel Status"]
pub type PChSR = crate::BitReader;
#[doc = "Field `C_CH_S` reader - Code DMA Channel Status"]
pub type CChSR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Preview DMA Channel Status"]
    #[inline(always)]
    pub fn p_ch_s(&self) -> PChSR {
        PChSR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Code DMA Channel Status"]
    #[inline(always)]
    pub fn c_ch_s(&self) -> CChSR {
        CChSR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DMA Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_chsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaChsrSpec;
impl crate::RegisterSpec for DmaChsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_chsr::R`](R) reader structure"]
impl crate::Readable for DmaChsrSpec {}
#[doc = "`reset()` method sets DMA_CHSR to value 0"]
impl crate::Resettable for DmaChsrSpec {}
