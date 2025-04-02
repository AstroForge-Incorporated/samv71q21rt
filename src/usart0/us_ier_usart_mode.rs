#[doc = "Register `US_IER_USART_MODE` writer"]
pub type W = crate::W<UsIerUsartModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Enable"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Enable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Enable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Enable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` writer - Max number of Repetitions Reached Interrupt Enable"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Enable"]
pub type RiicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Enable"]
pub type DsricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Enable"]
pub type DcdicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Enable"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Enable"]
pub type ManeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<UsIerUsartModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<UsIerUsartModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Enable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RxbrkW<UsIerUsartModeSpec> {
        RxbrkW::new(self, 2)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<UsIerUsartModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<UsIerUsartModeSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<UsIerUsartModeSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<UsIerUsartModeSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<UsIerUsartModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Max number of Repetitions Reached Interrupt Enable"]
    #[inline(always)]
    pub fn iter(&mut self) -> IterW<UsIerUsartModeSpec> {
        IterW::new(self, 10)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<UsIerUsartModeSpec> {
        NackW::new(self, 13)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Enable"]
    #[inline(always)]
    pub fn riic(&mut self) -> RiicW<UsIerUsartModeSpec> {
        RiicW::new(self, 16)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Enable"]
    #[inline(always)]
    pub fn dsric(&mut self) -> DsricW<UsIerUsartModeSpec> {
        DsricW::new(self, 17)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Enable"]
    #[inline(always)]
    pub fn dcdic(&mut self) -> DcdicW<UsIerUsartModeSpec> {
        DcdicW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Enable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CtsicW<UsIerUsartModeSpec> {
        CtsicW::new(self, 19)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Enable"]
    #[inline(always)]
    pub fn mane(&mut self) -> ManeW<UsIerUsartModeSpec> {
        ManeW::new(self, 24)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ier_usart_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIerUsartModeSpec;
impl crate::RegisterSpec for UsIerUsartModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_ier_usart_mode::W`](W) writer structure"]
impl crate::Writable for UsIerUsartModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_IER_USART_MODE to value 0"]
impl crate::Resettable for UsIerUsartModeSpec {}
