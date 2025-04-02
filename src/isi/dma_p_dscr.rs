#[doc = "Register `DMA_P_DSCR` reader"]
pub type R = crate::R<DmaPDscrSpec>;
#[doc = "Register `DMA_P_DSCR` writer"]
pub type W = crate::W<DmaPDscrSpec>;
#[doc = "Field `P_DSCR` reader - Preview Descriptor Base Address"]
pub type PDscrR = crate::FieldReader<u32>;
#[doc = "Field `P_DSCR` writer - Preview Descriptor Base Address"]
pub type PDscrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Preview Descriptor Base Address"]
    #[inline(always)]
    pub fn p_dscr(&self) -> PDscrR {
        PDscrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Preview Descriptor Base Address"]
    #[inline(always)]
    pub fn p_dscr(&mut self) -> PDscrW<DmaPDscrSpec> {
        PDscrW::new(self, 2)
    }
}
#[doc = "DMA Preview Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_p_dscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_p_dscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaPDscrSpec;
impl crate::RegisterSpec for DmaPDscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_p_dscr::R`](R) reader structure"]
impl crate::Readable for DmaPDscrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_p_dscr::W`](W) writer structure"]
impl crate::Writable for DmaPDscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_P_DSCR to value 0"]
impl crate::Resettable for DmaPDscrSpec {}
