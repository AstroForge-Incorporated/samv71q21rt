#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmit Empty Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Ready Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRUN` writer - Receive Overrun Interrupt Enable"]
pub type OvrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP0` writer - Compare 0 Interrupt Enable"]
pub type Cp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1` writer - Compare 1 Interrupt Enable"]
pub type Cp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSYN` writer - Tx Sync Interrupt Enable"]
pub type TxsynW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSYN` writer - Rx Sync Interrupt Enable"]
pub type RxsynW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<IerSpec> {
        TxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<IerSpec> {
        TxemptyW::new(self, 1)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<IerSpec> {
        RxrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovrun(&mut self) -> OvrunW<IerSpec> {
        OvrunW::new(self, 5)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cp0(&mut self) -> Cp0W<IerSpec> {
        Cp0W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cp1(&mut self) -> Cp1W<IerSpec> {
        Cp1W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Enable"]
    #[inline(always)]
    pub fn txsyn(&mut self) -> TxsynW<IerSpec> {
        TxsynW::new(self, 10)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Enable"]
    #[inline(always)]
    pub fn rxsyn(&mut self) -> RxsynW<IerSpec> {
        RxsynW::new(self, 11)
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
