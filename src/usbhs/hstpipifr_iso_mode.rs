#[doc = "Register `HSTPIPIFR_ISO_MODE[%s]` writer"]
pub type W = crate::W<HstpipifrIsoModeSpec>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UnderfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NakedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CrcerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type ShortpacketisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    pub fn rxinis(&mut self) -> RxinisW<HstpipifrIsoModeSpec> {
        RxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    pub fn txoutis(&mut self) -> TxoutisW<HstpipifrIsoModeSpec> {
        TxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    pub fn underfis(&mut self) -> UnderfisW<HstpipifrIsoModeSpec> {
        UnderfisW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    pub fn perris(&mut self) -> PerrisW<HstpipifrIsoModeSpec> {
        PerrisW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    pub fn nakedis(&mut self) -> NakedisW<HstpipifrIsoModeSpec> {
        NakedisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn overfis(&mut self) -> OverfisW<HstpipifrIsoModeSpec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    pub fn crcerris(&mut self) -> CrcerrisW<HstpipifrIsoModeSpec> {
        CrcerrisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    pub fn shortpacketis(&mut self) -> ShortpacketisW<HstpipifrIsoModeSpec> {
        ShortpacketisW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NbusybksW<HstpipifrIsoModeSpec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Host Pipe Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipifr_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipifrIsoModeSpec;
impl crate::RegisterSpec for HstpipifrIsoModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipifr_iso_mode::W`](W) writer structure"]
impl crate::Writable for HstpipifrIsoModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPIPIFR_ISO_MODE[%s] to value 0"]
impl crate::Resettable for HstpipifrIsoModeSpec {}
