#[doc = "Register `US_IER_SPI_MODE` writer"]
pub type W = crate::W<UsIerSpiModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Enable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSE` writer - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
pub type NsseW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<UsIerSpiModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<UsIerSpiModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<UsIerSpiModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<UsIerSpiModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<UsIerSpiModeSpec> {
        UnreW::new(self, 10)
    }
    #[doc = "Bit 19 - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
    #[inline(always)]
    pub fn nsse(&mut self) -> NsseW<UsIerSpiModeSpec> {
        NsseW::new(self, 19)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIerSpiModeSpec;
impl crate::RegisterSpec for UsIerSpiModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_ier_spi_mode::W`](W) writer structure"]
impl crate::Writable for UsIerSpiModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IER_SPI_MODE to value 0"]
impl crate::Resettable for UsIerSpiModeSpec {}
