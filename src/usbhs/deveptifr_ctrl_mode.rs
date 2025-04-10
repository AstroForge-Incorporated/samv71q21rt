#[doc = "Register `DEVEPTIFR_CTRL_MODE[%s]` writer"]
pub type W = crate::W<DeveptifrCtrlModeSpec>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPIS` writer - Received SETUP Interrupt Set"]
pub type RxstpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTIS` writer - NAKed OUT Interrupt Set"]
pub type NakoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINIS` writer - NAKed IN Interrupt Set"]
pub type NakinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDIS` writer - STALLed Interrupt Set"]
pub type StalledisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type ShortpacketsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    pub fn txinis(&mut self) -> TxinisW<DeveptifrCtrlModeSpec> {
        TxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    pub fn rxoutis(&mut self) -> RxoutisW<DeveptifrCtrlModeSpec> {
        RxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline(always)]
    pub fn rxstpis(&mut self) -> RxstpisW<DeveptifrCtrlModeSpec> {
        RxstpisW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline(always)]
    pub fn nakoutis(&mut self) -> NakoutisW<DeveptifrCtrlModeSpec> {
        NakoutisW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline(always)]
    pub fn nakinis(&mut self) -> NakinisW<DeveptifrCtrlModeSpec> {
        NakinisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn overfis(&mut self) -> OverfisW<DeveptifrCtrlModeSpec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline(always)]
    pub fn stalledis(&mut self) -> StalledisW<DeveptifrCtrlModeSpec> {
        StalledisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    pub fn shortpackets(&mut self) -> ShortpacketsW<DeveptifrCtrlModeSpec> {
        ShortpacketsW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NbusybksW<DeveptifrCtrlModeSpec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr_ctrl_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptifrCtrlModeSpec;
impl crate::RegisterSpec for DeveptifrCtrlModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptifr_ctrl_mode::W`](W) writer structure"]
impl crate::Writable for DeveptifrCtrlModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTIFR_CTRL_MODE[%s] to value 0"]
impl crate::Resettable for DeveptifrCtrlModeSpec {}
