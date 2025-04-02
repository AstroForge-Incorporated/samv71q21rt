#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RXRDY` writer - Disable RXRDY Interrupt"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Disable TXRDY Interrupt"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Disable Overrun Error Interrupt"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Disable Framing Error Interrupt"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Disable Parity Error Interrupt"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Disable TXEMPTY Interrupt"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` writer - Disable Comparison Interrupt"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable RXRDY Interrupt"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<IdrSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable TXRDY Interrupt"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<IdrSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Disable Overrun Error Interrupt"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<IdrSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Framing Error Interrupt"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<IdrSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable Parity Error Interrupt"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<IdrSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 9 - Disable TXEMPTY Interrupt"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<IdrSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 15 - Disable Comparison Interrupt"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<IdrSpec> {
        CmpW::new(self, 15)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
