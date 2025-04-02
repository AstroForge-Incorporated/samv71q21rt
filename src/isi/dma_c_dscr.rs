#[doc = "Register `DMA_C_DSCR` reader"]
pub type R = crate::R<DmaCDscrSpec>;
#[doc = "Register `DMA_C_DSCR` writer"]
pub type W = crate::W<DmaCDscrSpec>;
#[doc = "Field `C_DSCR` reader - Codec Descriptor Base Address"]
pub type CDscrR = crate::FieldReader<u32>;
#[doc = "Field `C_DSCR` writer - Codec Descriptor Base Address"]
pub type CDscrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Codec Descriptor Base Address"]
    #[inline(always)]
    pub fn c_dscr(&self) -> CDscrR {
        CDscrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Codec Descriptor Base Address"]
    #[inline(always)]
    pub fn c_dscr(&mut self) -> CDscrW<DmaCDscrSpec> {
        CDscrW::new(self, 2)
    }
}
#[doc = "DMA Codec Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c_dscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c_dscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCDscrSpec;
impl crate::RegisterSpec for DmaCDscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c_dscr::R`](R) reader structure"]
impl crate::Readable for DmaCDscrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c_dscr::W`](W) writer structure"]
impl crate::Writable for DmaCDscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C_DSCR to value 0"]
impl crate::Resettable for DmaCDscrSpec {}
