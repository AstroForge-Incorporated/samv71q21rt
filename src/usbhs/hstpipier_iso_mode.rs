#[doc = "Register `HSTPIPIER_ISO_MODE[%s]` writer"]
pub type W = crate::W<HstpipierIsoModeSpec>;
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub type RxinesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub type TxoutesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIES` writer - Underflow Interrupt Enable"]
pub type UnderfiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub type PerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub type NakedesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub type OverfiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub type CrcerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIES` writer - Short Packet Interrupt Enable"]
pub type ShortpacketiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Enable"]
pub type NbusybkesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDISHDMAS` writer - Pipe Interrupts Disable HDMA Request Enable"]
pub type PdishdmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZES` writer - Pipe Freeze Enable"]
pub type PfreezesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RstdtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxines(&mut self) -> RxinesW<HstpipierIsoModeSpec> {
        RxinesW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoutes(&mut self) -> TxoutesW<HstpipierIsoModeSpec> {
        TxoutesW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn underfies(&mut self) -> UnderfiesW<HstpipierIsoModeSpec> {
        UnderfiesW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perres(&mut self) -> PerresW<HstpipierIsoModeSpec> {
        PerresW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakedes(&mut self) -> NakedesW<HstpipierIsoModeSpec> {
        NakedesW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfies(&mut self) -> OverfiesW<HstpipierIsoModeSpec> {
        OverfiesW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crcerres(&mut self) -> CrcerresW<HstpipierIsoModeSpec> {
        CrcerresW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketies(&mut self) -> ShortpacketiesW<HstpipierIsoModeSpec> {
        ShortpacketiesW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NbusybkesW<HstpipierIsoModeSpec> {
        NbusybkesW::new(self, 12)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdmas(&mut self) -> PdishdmasW<HstpipierIsoModeSpec> {
        PdishdmasW::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    pub fn pfreezes(&mut self) -> PfreezesW<HstpipierIsoModeSpec> {
        PfreezesW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    pub fn rstdts(&mut self) -> RstdtsW<HstpipierIsoModeSpec> {
        RstdtsW::new(self, 18)
    }
}
#[doc = "Host Pipe Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipier_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipierIsoModeSpec;
impl crate::RegisterSpec for HstpipierIsoModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipier_iso_mode::W`](W) writer structure"]
impl crate::Writable for HstpipierIsoModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPIPIER_ISO_MODE[%s] to value 0"]
impl crate::Resettable for HstpipierIsoModeSpec {}
