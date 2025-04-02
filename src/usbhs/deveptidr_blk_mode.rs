#[doc = "Register `DEVEPTIDR_BLK_MODE[%s]` writer"]
pub type W = crate::W<DeveptidrBlkModeSpec>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub type RxstpecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub type NakoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub type NakinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OverfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub type StalledecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type ShortpacketecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EpdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub type NyetdiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub type StallrqcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    pub fn txinec(&mut self) -> TxinecW<DeveptidrBlkModeSpec> {
        TxinecW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutec(&mut self) -> RxoutecW<DeveptidrBlkModeSpec> {
        RxoutecW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    pub fn rxstpec(&mut self) -> RxstpecW<DeveptidrBlkModeSpec> {
        RxstpecW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    pub fn nakoutec(&mut self) -> NakoutecW<DeveptidrBlkModeSpec> {
        NakoutecW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    pub fn nakinec(&mut self) -> NakinecW<DeveptidrBlkModeSpec> {
        NakinecW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfec(&mut self) -> OverfecW<DeveptidrBlkModeSpec> {
        OverfecW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn stalledec(&mut self) -> StalledecW<DeveptidrBlkModeSpec> {
        StalledecW::new(self, 6)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketec(&mut self) -> ShortpacketecW<DeveptidrBlkModeSpec> {
        ShortpacketecW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    pub fn nbusybkec(&mut self) -> NbusybkecW<DeveptidrBlkModeSpec> {
        NbusybkecW::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    pub fn fifoconc(&mut self) -> FifoconcW<DeveptidrBlkModeSpec> {
        FifoconcW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    pub fn epdishdmac(&mut self) -> EpdishdmacW<DeveptidrBlkModeSpec> {
        EpdishdmacW::new(self, 16)
    }
    #[doc = "Bit 17 - NYET Token Disable Clear"]
    #[inline(always)]
    pub fn nyetdisc(&mut self) -> NyetdiscW<DeveptidrBlkModeSpec> {
        NyetdiscW::new(self, 17)
    }
    #[doc = "Bit 19 - STALL Request Clear"]
    #[inline(always)]
    pub fn stallrqc(&mut self) -> StallrqcW<DeveptidrBlkModeSpec> {
        StallrqcW::new(self, 19)
    }
}
#[doc = "Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptidr_blk_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptidrBlkModeSpec;
impl crate::RegisterSpec for DeveptidrBlkModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr_blk_mode::W`](W) writer structure"]
impl crate::Writable for DeveptidrBlkModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTIDR_BLK_MODE[%s] to value 0"]
impl crate::Resettable for DeveptidrBlkModeSpec {}
