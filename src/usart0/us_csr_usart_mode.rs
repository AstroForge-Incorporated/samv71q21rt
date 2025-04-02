#[doc = "Register `US_CSR_USART_MODE` reader"]
pub type R = crate::R<UsCsrUsartModeSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `RXBRK` reader - Break Received/End of Break (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type PareR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Receiver Timeout (cleared by writing a one to bit US_CR.STTTO)"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `ITER` reader - Max Number of Repetitions Reached (cleared by writing a one to bit US_CR.RSTIT)"]
pub type IterR = crate::BitReader;
#[doc = "Field `NACK` reader - Non Acknowledge Interrupt (cleared by writing a one to bit US_CR.RSTNACK)"]
pub type NackR = crate::BitReader;
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Flag (cleared on read)"]
pub type RiicR = crate::BitReader;
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Flag (cleared on read)"]
pub type DsricR = crate::BitReader;
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Flag (cleared on read)"]
pub type DcdicR = crate::BitReader;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag (cleared on read)"]
pub type CtsicR = crate::BitReader;
#[doc = "Field `RI` reader - Image of RI Input"]
pub type RiR = crate::BitReader;
#[doc = "Field `DSR` reader - Image of DSR Input"]
pub type DsrR = crate::BitReader;
#[doc = "Field `DCD` reader - Image of DCD Input"]
pub type DcdR = crate::BitReader;
#[doc = "Field `CTS` reader - Image of CTS Input"]
pub type CtsR = crate::BitReader;
#[doc = "Field `MANERR` reader - Manchester Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
pub type ManerrR = crate::BitReader;
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
    #[doc = "Bit 2 - Break Received/End of Break (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Timeout (cleared by writing a one to bit US_CR.STTTO)"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached (cleared by writing a one to bit US_CR.RSTIT)"]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt (cleared by writing a one to bit US_CR.RSTNACK)"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn riic(&self) -> RiicR {
        RiicR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn dsric(&self) -> DsricR {
        DsricR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn dcdic(&self) -> DcdicR {
        DcdicR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn ctsic(&self) -> CtsicR {
        CtsicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Image of RI Input"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Image of DSR Input"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Image of DCD Input"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Manchester Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn manerr(&self) -> ManerrR {
        ManerrR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_csr_usart_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsCsrUsartModeSpec;
impl crate::RegisterSpec for UsCsrUsartModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_csr_usart_mode::R`](R) reader structure"]
impl crate::Readable for UsCsrUsartModeSpec {}
#[doc = "`reset()` method sets US_CSR_USART_MODE to value 0"]
impl crate::Resettable for UsCsrUsartModeSpec {}
