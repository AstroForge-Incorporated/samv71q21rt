#[doc = "Register `DEVEPTICR_CTRL_MODE[%s]` writer"]
pub type W = crate::W<DevepticrCtrlModeSpec>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPIC` writer - Received SETUP Interrupt Clear"]
pub type RxstpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTIC` writer - NAKed OUT Interrupt Clear"]
pub type NakouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINIC` writer - NAKed IN Interrupt Clear"]
pub type NakinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDIC` writer - STALLed Interrupt Clear"]
pub type StalledicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn txinic(&mut self) -> TxinicW<DevepticrCtrlModeSpec> {
        TxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutic(&mut self) -> RxouticW<DevepticrCtrlModeSpec> {
        RxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    pub fn rxstpic(&mut self) -> RxstpicW<DevepticrCtrlModeSpec> {
        RxstpicW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    pub fn nakoutic(&mut self) -> NakouticW<DevepticrCtrlModeSpec> {
        NakouticW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    pub fn nakinic(&mut self) -> NakinicW<DevepticrCtrlModeSpec> {
        NakinicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OverficW<DevepticrCtrlModeSpec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn stalledic(&mut self) -> StalledicW<DevepticrCtrlModeSpec> {
        StalledicW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketc(&mut self) -> ShortpacketcW<DevepticrCtrlModeSpec> {
        ShortpacketcW::new(self, 7)
    }
}
#[doc = "Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr_ctrl_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevepticrCtrlModeSpec;
impl crate::RegisterSpec for DevepticrCtrlModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devepticr_ctrl_mode::W`](W) writer structure"]
impl crate::Writable for DevepticrCtrlModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTICR_CTRL_MODE[%s] to value 0"]
impl crate::Resettable for DevepticrCtrlModeSpec {}
