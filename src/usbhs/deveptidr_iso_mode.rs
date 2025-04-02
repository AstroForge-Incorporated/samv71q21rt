#[doc = "Register `DEVEPTIDR_ISO_MODE[%s]` writer"]
pub type W = crate::W<DeveptidrIsoModeSpec>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFEC` writer - Underflow Interrupt Clear"]
pub type UnderfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERREC` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
pub type HbisoinerrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHEC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HbisoflushecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OverfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type ShortpacketecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATAEC` writer - MData Interrupt Clear"]
pub type MdataecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAXEC` writer - DataX Interrupt Clear"]
pub type DataxecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTRANSEC` writer - Transaction Error Interrupt Clear"]
pub type ErrortransecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EpdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    pub fn txinec(&mut self) -> TxinecW<DeveptidrIsoModeSpec> {
        TxinecW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutec(&mut self) -> RxoutecW<DeveptidrIsoModeSpec> {
        RxoutecW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfec(&mut self) -> UnderfecW<DeveptidrIsoModeSpec> {
        UnderfecW::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoinerrec(&mut self) -> HbisoinerrecW<DeveptidrIsoModeSpec> {
        HbisoinerrecW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoflushec(&mut self) -> HbisoflushecW<DeveptidrIsoModeSpec> {
        HbisoflushecW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfec(&mut self) -> OverfecW<DeveptidrIsoModeSpec> {
        OverfecW::new(self, 5)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketec(&mut self) -> ShortpacketecW<DeveptidrIsoModeSpec> {
        ShortpacketecW::new(self, 7)
    }
    #[doc = "Bit 8 - MData Interrupt Clear"]
    #[inline(always)]
    pub fn mdataec(&mut self) -> MdataecW<DeveptidrIsoModeSpec> {
        MdataecW::new(self, 8)
    }
    #[doc = "Bit 9 - DataX Interrupt Clear"]
    #[inline(always)]
    pub fn dataxec(&mut self) -> DataxecW<DeveptidrIsoModeSpec> {
        DataxecW::new(self, 9)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Clear"]
    #[inline(always)]
    pub fn errortransec(&mut self) -> ErrortransecW<DeveptidrIsoModeSpec> {
        ErrortransecW::new(self, 10)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    pub fn nbusybkec(&mut self) -> NbusybkecW<DeveptidrIsoModeSpec> {
        NbusybkecW::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    pub fn fifoconc(&mut self) -> FifoconcW<DeveptidrIsoModeSpec> {
        FifoconcW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    pub fn epdishdmac(&mut self) -> EpdishdmacW<DeveptidrIsoModeSpec> {
        EpdishdmacW::new(self, 16)
    }
}
#[doc = "Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptidrIsoModeSpec;
impl crate::RegisterSpec for DeveptidrIsoModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr_iso_mode::W`](W) writer structure"]
impl crate::Writable for DeveptidrIsoModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTIDR_ISO_MODE[%s] to value 0"]
impl crate::Resettable for DeveptidrIsoModeSpec {}
