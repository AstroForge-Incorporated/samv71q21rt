#[doc = "Register `DEVEPTIFR_ISO_MODE[%s]` writer"]
pub type W = crate::W<DeveptifrIsoModeSpec>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UnderfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRIS` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Set"]
pub type HbisoinerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHIS` writer - High Bandwidth Isochronous IN Flush Interrupt Set"]
pub type HbisoflushisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CrcerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type ShortpacketsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    pub fn txinis(&mut self) -> TxinisW<DeveptifrIsoModeSpec> {
        TxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    pub fn rxoutis(&mut self) -> RxoutisW<DeveptifrIsoModeSpec> {
        RxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    pub fn underfis(&mut self) -> UnderfisW<DeveptifrIsoModeSpec> {
        UnderfisW::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Set"]
    #[inline(always)]
    pub fn hbisoinerris(&mut self) -> HbisoinerrisW<DeveptifrIsoModeSpec> {
        HbisoinerrisW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Set"]
    #[inline(always)]
    pub fn hbisoflushis(&mut self) -> HbisoflushisW<DeveptifrIsoModeSpec> {
        HbisoflushisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn overfis(&mut self) -> OverfisW<DeveptifrIsoModeSpec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    pub fn crcerris(&mut self) -> CrcerrisW<DeveptifrIsoModeSpec> {
        CrcerrisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    pub fn shortpackets(&mut self) -> ShortpacketsW<DeveptifrIsoModeSpec> {
        ShortpacketsW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NbusybksW<DeveptifrIsoModeSpec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptifrIsoModeSpec;
impl crate::RegisterSpec for DeveptifrIsoModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptifr_iso_mode::W`](W) writer structure"]
impl crate::Writable for DeveptifrIsoModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTIFR_ISO_MODE[%s] to value 0"]
impl crate::Resettable for DeveptifrIsoModeSpec {}
