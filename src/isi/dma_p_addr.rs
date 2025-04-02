#[doc = "Register `DMA_P_ADDR` reader"]
pub type R = crate::R<DmaPAddrSpec>;
#[doc = "Register `DMA_P_ADDR` writer"]
pub type W = crate::W<DmaPAddrSpec>;
#[doc = "Field `P_ADDR` reader - Preview Image Base Address"]
pub type PAddrR = crate::FieldReader<u32>;
#[doc = "Field `P_ADDR` writer - Preview Image Base Address"]
pub type PAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Preview Image Base Address"]
    #[inline(always)]
    pub fn p_addr(&self) -> PAddrR {
        PAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Preview Image Base Address"]
    #[inline(always)]
    pub fn p_addr(&mut self) -> PAddrW<DmaPAddrSpec> {
        PAddrW::new(self, 2)
    }
}
#[doc = "DMA Preview Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_p_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_p_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaPAddrSpec;
impl crate::RegisterSpec for DmaPAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_p_addr::R`](R) reader structure"]
impl crate::Readable for DmaPAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_p_addr::W`](W) writer structure"]
impl crate::Writable for DmaPAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_P_ADDR to value 0"]
impl crate::Resettable for DmaPAddrSpec {}
