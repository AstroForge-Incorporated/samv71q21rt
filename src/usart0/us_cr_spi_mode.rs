#[doc = "Register `US_CR_SPI_MODE` writer"]
pub type W = crate::W<UsCrSpiModeSpec>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RstrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RsttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RststaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCS` writer - Force SPI Chip Select"]
pub type FcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCS` writer - Release SPI Chip Select"]
pub type RcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RstrxW<UsCrSpiModeSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RsttxW<UsCrSpiModeSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<UsCrSpiModeSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<UsCrSpiModeSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<UsCrSpiModeSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TxdisW<UsCrSpiModeSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RststaW<UsCrSpiModeSpec> {
        RststaW::new(self, 8)
    }
    #[doc = "Bit 18 - Force SPI Chip Select"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FcsW<UsCrSpiModeSpec> {
        FcsW::new(self, 18)
    }
    #[doc = "Bit 19 - Release SPI Chip Select"]
    #[inline(always)]
    pub fn rcs(&mut self) -> RcsW<UsCrSpiModeSpec> {
        RcsW::new(self, 19)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_cr_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsCrSpiModeSpec;
impl crate::RegisterSpec for UsCrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_cr_spi_mode::W`](W) writer structure"]
impl crate::Writable for UsCrSpiModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_CR_SPI_MODE to value 0"]
impl crate::Resettable for UsCrSpiModeSpec {}
