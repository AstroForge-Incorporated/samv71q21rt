#[doc = "Register `DEVEPTIER_ISO_MODE[%s]` writer"]
pub type W = crate::W<DeveptierIsoModeSpec>;
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TxinesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RxoutesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFES` writer - Underflow Interrupt Enable"]
pub type UnderfesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRES` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Enable"]
pub type HbisoinerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHES` writer - High Bandwidth Isochronous IN Flush Interrupt Enable"]
pub type HbisoflushesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OverfesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub type CrcerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type ShortpacketesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATAES` writer - MData Interrupt Enable"]
pub type MdataesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAXES` writer - DataX Interrupt Enable"]
pub type DataxesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTRANSES` writer - Transaction Error Interrupt Enable"]
pub type ErrortransesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NbusybkesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KillbksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FifoconsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EpdishdmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RstdtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn txines(&mut self) -> TxinesW<DeveptierIsoModeSpec> {
        TxinesW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxoutes(&mut self) -> RxoutesW<DeveptierIsoModeSpec> {
        RxoutesW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn underfes(&mut self) -> UnderfesW<DeveptierIsoModeSpec> {
        UnderfesW::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn hbisoinerres(&mut self) -> HbisoinerresW<DeveptierIsoModeSpec> {
        HbisoinerresW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Enable"]
    #[inline(always)]
    pub fn hbisoflushes(&mut self) -> HbisoflushesW<DeveptierIsoModeSpec> {
        HbisoflushesW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfes(&mut self) -> OverfesW<DeveptierIsoModeSpec> {
        OverfesW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crcerres(&mut self) -> CrcerresW<DeveptierIsoModeSpec> {
        CrcerresW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketes(&mut self) -> ShortpacketesW<DeveptierIsoModeSpec> {
        ShortpacketesW::new(self, 7)
    }
    #[doc = "Bit 8 - MData Interrupt Enable"]
    #[inline(always)]
    pub fn mdataes(&mut self) -> MdataesW<DeveptierIsoModeSpec> {
        MdataesW::new(self, 8)
    }
    #[doc = "Bit 9 - DataX Interrupt Enable"]
    #[inline(always)]
    pub fn dataxes(&mut self) -> DataxesW<DeveptierIsoModeSpec> {
        DataxesW::new(self, 9)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn errortranses(&mut self) -> ErrortransesW<DeveptierIsoModeSpec> {
        ErrortransesW::new(self, 10)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NbusybkesW<DeveptierIsoModeSpec> {
        NbusybkesW::new(self, 12)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbks(&mut self) -> KillbksW<DeveptierIsoModeSpec> {
        KillbksW::new(self, 13)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocons(&mut self) -> FifoconsW<DeveptierIsoModeSpec> {
        FifoconsW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn epdishdmas(&mut self) -> EpdishdmasW<DeveptierIsoModeSpec> {
        EpdishdmasW::new(self, 16)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    pub fn rstdts(&mut self) -> RstdtsW<DeveptierIsoModeSpec> {
        RstdtsW::new(self, 18)
    }
}
#[doc = "Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptier_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptierIsoModeSpec;
impl crate::RegisterSpec for DeveptierIsoModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptier_iso_mode::W`](W) writer structure"]
impl crate::Writable for DeveptierIsoModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTIER_ISO_MODE[%s] to value 0"]
impl crate::Resettable for DeveptierIsoModeSpec {}
