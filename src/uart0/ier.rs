#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RXRDY` writer - Enable RXRDY Interrupt"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Enable TXRDY Interrupt"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Enable Overrun Error Interrupt"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Enable Framing Error Interrupt"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Enable Parity Error Interrupt"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Enable TXEMPTY Interrupt"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` writer - Enable Comparison Interrupt"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable RXRDY Interrupt"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<IerSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable TXRDY Interrupt"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<IerSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Enable Overrun Error Interrupt"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<IerSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Framing Error Interrupt"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<IerSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Parity Error Interrupt"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<IerSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 9 - Enable TXEMPTY Interrupt"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<IerSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 15 - Enable Comparison Interrupt"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<IerSpec> {
        CmpW::new(self, 15)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
