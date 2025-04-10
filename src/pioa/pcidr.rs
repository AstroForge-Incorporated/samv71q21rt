#[doc = "Register `PCIDR` writer"]
pub type W = crate::W<PcidrSpec>;
#[doc = "Field `DRDY` writer - Parallel Capture Mode Data Ready Interrupt Disable"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Parallel Capture Mode Overrun Error Interrupt Disable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Reception Transfer Interrupt Disable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Reception Buffer Full Interrupt Disable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Disable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DrdyW<PcidrSpec> {
        DrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<PcidrSpec> {
        OvreW::new(self, 1)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Disable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> EndrxW<PcidrSpec> {
        EndrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RxbuffW<PcidrSpec> {
        RxbuffW::new(self, 3)
    }
}
#[doc = "Parallel Capture Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcidrSpec;
impl crate::RegisterSpec for PcidrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcidr::W`](W) writer structure"]
impl crate::Writable for PcidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCIDR to value 0"]
impl crate::Resettable for PcidrSpec {}
