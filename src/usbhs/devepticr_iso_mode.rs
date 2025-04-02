#[doc = "Register `DEVEPTICR_ISO_MODE[%s]` writer"]
pub type W = crate::W<DevepticrIsoModeSpec>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UnderficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRIC` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
pub type HbisoinerricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHIC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HbisoflushicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub type CrcerricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn txinic(&mut self) -> TxinicW<DevepticrIsoModeSpec> {
        TxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutic(&mut self) -> RxouticW<DevepticrIsoModeSpec> {
        RxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfic(&mut self) -> UnderficW<DevepticrIsoModeSpec> {
        UnderficW::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoinerric(&mut self) -> HbisoinerricW<DevepticrIsoModeSpec> {
        HbisoinerricW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoflushic(&mut self) -> HbisoflushicW<DevepticrIsoModeSpec> {
        HbisoflushicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OverficW<DevepticrIsoModeSpec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    pub fn crcerric(&mut self) -> CrcerricW<DevepticrIsoModeSpec> {
        CrcerricW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketc(&mut self) -> ShortpacketcW<DevepticrIsoModeSpec> {
        ShortpacketcW::new(self, 7)
    }
}
#[doc = "Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevepticrIsoModeSpec;
impl crate::RegisterSpec for DevepticrIsoModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devepticr_iso_mode::W`](W) writer structure"]
impl crate::Writable for DevepticrIsoModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTICR_ISO_MODE[%s] to value 0"]
impl crate::Resettable for DevepticrIsoModeSpec {}
