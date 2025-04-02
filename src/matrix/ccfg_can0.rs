#[doc = "Register `CCFG_CAN0` reader"]
pub type R = crate::R<CcfgCan0Spec>;
#[doc = "Register `CCFG_CAN0` writer"]
pub type W = crate::W<CcfgCan0Spec>;
#[doc = "Field `CAN0DMABA` reader - CAN0 DMA Base Address"]
pub type Can0dmabaR = crate::FieldReader<u16>;
#[doc = "Field `CAN0DMABA` writer - CAN0 DMA Base Address"]
pub type Can0dmabaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&self) -> Can0dmabaR {
        Can0dmabaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&mut self) -> Can0dmabaW<CcfgCan0Spec> {
        Can0dmabaW::new(self, 16)
    }
}
#[doc = "CAN0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_can0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_can0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgCan0Spec;
impl crate::RegisterSpec for CcfgCan0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_can0::R`](R) reader structure"]
impl crate::Readable for CcfgCan0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_can0::W`](W) writer structure"]
impl crate::Writable for CcfgCan0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCFG_CAN0 to value 0"]
impl crate::Resettable for CcfgCan0Spec {}
