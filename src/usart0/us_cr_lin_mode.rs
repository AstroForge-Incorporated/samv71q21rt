#[doc = "Register `US_CR_LIN_MODE` writer"]
pub type W = crate::W<UsCrLinModeSpec>;
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
#[doc = "Field `LINABT` writer - Abort LIN Transmission"]
pub type LinabtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINWKUP` writer - Send LIN Wakeup Signal"]
pub type LinwkupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RstrxW<UsCrLinModeSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RsttxW<UsCrLinModeSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<UsCrLinModeSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<UsCrLinModeSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<UsCrLinModeSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TxdisW<UsCrLinModeSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RststaW<UsCrLinModeSpec> {
        RststaW::new(self, 8)
    }
    #[doc = "Bit 20 - Abort LIN Transmission"]
    #[inline(always)]
    pub fn linabt(&mut self) -> LinabtW<UsCrLinModeSpec> {
        LinabtW::new(self, 20)
    }
    #[doc = "Bit 21 - Send LIN Wakeup Signal"]
    #[inline(always)]
    pub fn linwkup(&mut self) -> LinwkupW<UsCrLinModeSpec> {
        LinwkupW::new(self, 21)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_cr_lin_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsCrLinModeSpec;
impl crate::RegisterSpec for UsCrLinModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_cr_lin_mode::W`](W) writer structure"]
impl crate::Writable for UsCrLinModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_CR_LIN_MODE to value 0"]
impl crate::Resettable for UsCrLinModeSpec {}
