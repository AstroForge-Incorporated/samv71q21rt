#[doc = "Register `DMA_C_ADDR` reader"]
pub type R = crate::R<DmaCAddrSpec>;
#[doc = "Register `DMA_C_ADDR` writer"]
pub type W = crate::W<DmaCAddrSpec>;
#[doc = "Field `C_ADDR` reader - Codec Image Base Address"]
pub type CAddrR = crate::FieldReader<u32>;
#[doc = "Field `C_ADDR` writer - Codec Image Base Address"]
pub type CAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Codec Image Base Address"]
    #[inline(always)]
    pub fn c_addr(&self) -> CAddrR {
        CAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Codec Image Base Address"]
    #[inline(always)]
    pub fn c_addr(&mut self) -> CAddrW<DmaCAddrSpec> {
        CAddrW::new(self, 2)
    }
}
#[doc = "DMA Codec Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCAddrSpec;
impl crate::RegisterSpec for DmaCAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c_addr::R`](R) reader structure"]
impl crate::Readable for DmaCAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c_addr::W`](W) writer structure"]
impl crate::Writable for DmaCAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C_ADDR to value 0"]
impl crate::Resettable for DmaCAddrSpec {}
