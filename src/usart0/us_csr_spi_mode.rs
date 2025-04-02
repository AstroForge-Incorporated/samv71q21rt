#[doc = "Register `US_CSR_SPI_MODE` reader"]
pub type R = crate::R<UsCsrSpiModeSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNRE` reader - SPI Underrun Error"]
pub type UnreR = crate::BitReader;
#[doc = "Field `NSSE` reader - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
pub type NsseR = crate::BitReader;
#[doc = "Field `NSS` reader - Image of NSS Line"]
pub type NssR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
    #[inline(always)]
    pub fn nsse(&self) -> NsseR {
        NsseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Image of NSS Line"]
    #[inline(always)]
    pub fn nss(&self) -> NssR {
        NssR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_spi_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsCsrSpiModeSpec;
impl crate::RegisterSpec for UsCsrSpiModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_csr_spi_mode::R`](R) reader structure"]
impl crate::Readable for UsCsrSpiModeSpec {}
#[doc = "`reset()` method sets US_CSR_SPI_MODE to value 0"]
impl crate::Resettable for UsCsrSpiModeSpec {}
