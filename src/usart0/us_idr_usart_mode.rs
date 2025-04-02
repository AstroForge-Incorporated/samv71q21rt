#[doc = "Register `US_IDR_USART_MODE` writer"]
pub type W = crate::W<UsIdrUsartModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Disable"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Disable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` writer - Max Number of Repetitions Reached Interrupt Disable"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Disable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Disable"]
pub type RiicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Disable"]
pub type DsricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Disable"]
pub type DcdicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Disable"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Disable"]
pub type ManeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<UsIdrUsartModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<UsIdrUsartModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RxbrkW<UsIdrUsartModeSpec> {
        RxbrkW::new(self, 2)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<UsIdrUsartModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<UsIdrUsartModeSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<UsIdrUsartModeSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Timeout Interrupt Disable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<UsIdrUsartModeSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<UsIdrUsartModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached Interrupt Disable"]
    #[inline(always)]
    pub fn iter(&mut self) -> IterW<UsIdrUsartModeSpec> {
        IterW::new(self, 10)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<UsIdrUsartModeSpec> {
        NackW::new(self, 13)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline(always)]
    pub fn riic(&mut self) -> RiicW<UsIdrUsartModeSpec> {
        RiicW::new(self, 16)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline(always)]
    pub fn dsric(&mut self) -> DsricW<UsIdrUsartModeSpec> {
        DsricW::new(self, 17)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn dcdic(&mut self) -> DcdicW<UsIdrUsartModeSpec> {
        DcdicW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CtsicW<UsIdrUsartModeSpec> {
        CtsicW::new(self, 19)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Disable"]
    #[inline(always)]
    pub fn mane(&mut self) -> ManeW<UsIdrUsartModeSpec> {
        ManeW::new(self, 24)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_idr_usart_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIdrUsartModeSpec;
impl crate::RegisterSpec for UsIdrUsartModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_idr_usart_mode::W`](W) writer structure"]
impl crate::Writable for UsIdrUsartModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IDR_USART_MODE to value 0"]
impl crate::Resettable for UsIdrUsartModeSpec {}
