#[doc = "Register `US_CR_USART_MODE` writer"]
pub type W = crate::W<UsCrUsartModeSpec>;
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
#[doc = "Field `STTBRK` writer - Start Break"]
pub type SttbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPBRK` writer - Stop Break"]
pub type StpbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTTO` writer - Clear TIMEOUT Flag and Start Timeout After Next Character Received"]
pub type StttoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDA` writer - Send Address"]
pub type SendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub type RstitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub type RstnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETTO` writer - Start Timeout Immediately"]
pub type RettoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTREN` writer - Data Terminal Ready Enable"]
pub type DtrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRDIS` writer - Data Terminal Ready Disable"]
pub type DtrdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub type RtsdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RstrxW<UsCrUsartModeSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RsttxW<UsCrUsartModeSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<UsCrUsartModeSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<UsCrUsartModeSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<UsCrUsartModeSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TxdisW<UsCrUsartModeSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RststaW<UsCrUsartModeSpec> {
        RststaW::new(self, 8)
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    pub fn sttbrk(&mut self) -> SttbrkW<UsCrUsartModeSpec> {
        SttbrkW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    pub fn stpbrk(&mut self) -> StpbrkW<UsCrUsartModeSpec> {
        StpbrkW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear TIMEOUT Flag and Start Timeout After Next Character Received"]
    #[inline(always)]
    pub fn sttto(&mut self) -> StttoW<UsCrUsartModeSpec> {
        StttoW::new(self, 11)
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    pub fn senda(&mut self) -> SendaW<UsCrUsartModeSpec> {
        SendaW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    pub fn rstit(&mut self) -> RstitW<UsCrUsartModeSpec> {
        RstitW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    pub fn rstnack(&mut self) -> RstnackW<UsCrUsartModeSpec> {
        RstnackW::new(self, 14)
    }
    #[doc = "Bit 15 - Start Timeout Immediately"]
    #[inline(always)]
    pub fn retto(&mut self) -> RettoW<UsCrUsartModeSpec> {
        RettoW::new(self, 15)
    }
    #[doc = "Bit 16 - Data Terminal Ready Enable"]
    #[inline(always)]
    pub fn dtren(&mut self) -> DtrenW<UsCrUsartModeSpec> {
        DtrenW::new(self, 16)
    }
    #[doc = "Bit 17 - Data Terminal Ready Disable"]
    #[inline(always)]
    pub fn dtrdis(&mut self) -> DtrdisW<UsCrUsartModeSpec> {
        DtrdisW::new(self, 17)
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<UsCrUsartModeSpec> {
        RtsenW::new(self, 18)
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    pub fn rtsdis(&mut self) -> RtsdisW<UsCrUsartModeSpec> {
        RtsdisW::new(self, 19)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_cr_usart_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsCrUsartModeSpec;
impl crate::RegisterSpec for UsCrUsartModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_cr_usart_mode::W`](W) writer structure"]
impl crate::Writable for UsCrUsartModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_CR_USART_MODE to value 0"]
impl crate::Resettable for UsCrUsartModeSpec {}
