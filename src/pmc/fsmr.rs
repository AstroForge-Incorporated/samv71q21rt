#[doc = "Register `FSMR` reader"]
pub type R = crate::R<FsmrSpec>;
#[doc = "Register `FSMR` writer"]
pub type W = crate::W<FsmrSpec>;
#[doc = "Field `FSTT0` reader - Fast Startup Input Enable 0"]
pub type Fstt0R = crate::BitReader;
#[doc = "Field `FSTT0` writer - Fast Startup Input Enable 0"]
pub type Fstt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT1` reader - Fast Startup Input Enable 1"]
pub type Fstt1R = crate::BitReader;
#[doc = "Field `FSTT1` writer - Fast Startup Input Enable 1"]
pub type Fstt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT2` reader - Fast Startup Input Enable 2"]
pub type Fstt2R = crate::BitReader;
#[doc = "Field `FSTT2` writer - Fast Startup Input Enable 2"]
pub type Fstt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT3` reader - Fast Startup Input Enable 3"]
pub type Fstt3R = crate::BitReader;
#[doc = "Field `FSTT3` writer - Fast Startup Input Enable 3"]
pub type Fstt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT4` reader - Fast Startup Input Enable 4"]
pub type Fstt4R = crate::BitReader;
#[doc = "Field `FSTT4` writer - Fast Startup Input Enable 4"]
pub type Fstt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT5` reader - Fast Startup Input Enable 5"]
pub type Fstt5R = crate::BitReader;
#[doc = "Field `FSTT5` writer - Fast Startup Input Enable 5"]
pub type Fstt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT6` reader - Fast Startup Input Enable 6"]
pub type Fstt6R = crate::BitReader;
#[doc = "Field `FSTT6` writer - Fast Startup Input Enable 6"]
pub type Fstt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT7` reader - Fast Startup Input Enable 7"]
pub type Fstt7R = crate::BitReader;
#[doc = "Field `FSTT7` writer - Fast Startup Input Enable 7"]
pub type Fstt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT8` reader - Fast Startup Input Enable 8"]
pub type Fstt8R = crate::BitReader;
#[doc = "Field `FSTT8` writer - Fast Startup Input Enable 8"]
pub type Fstt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT9` reader - Fast Startup Input Enable 9"]
pub type Fstt9R = crate::BitReader;
#[doc = "Field `FSTT9` writer - Fast Startup Input Enable 9"]
pub type Fstt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT10` reader - Fast Startup Input Enable 10"]
pub type Fstt10R = crate::BitReader;
#[doc = "Field `FSTT10` writer - Fast Startup Input Enable 10"]
pub type Fstt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT11` reader - Fast Startup Input Enable 11"]
pub type Fstt11R = crate::BitReader;
#[doc = "Field `FSTT11` writer - Fast Startup Input Enable 11"]
pub type Fstt11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT12` reader - Fast Startup Input Enable 12"]
pub type Fstt12R = crate::BitReader;
#[doc = "Field `FSTT12` writer - Fast Startup Input Enable 12"]
pub type Fstt12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT13` reader - Fast Startup Input Enable 13"]
pub type Fstt13R = crate::BitReader;
#[doc = "Field `FSTT13` writer - Fast Startup Input Enable 13"]
pub type Fstt13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT14` reader - Fast Startup Input Enable 14"]
pub type Fstt14R = crate::BitReader;
#[doc = "Field `FSTT14` writer - Fast Startup Input Enable 14"]
pub type Fstt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTT15` reader - Fast Startup Input Enable 15"]
pub type Fstt15R = crate::BitReader;
#[doc = "Field `FSTT15` writer - Fast Startup Input Enable 15"]
pub type Fstt15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTAL` reader - RTT Alarm Enable"]
pub type RttalR = crate::BitReader;
#[doc = "Field `RTTAL` writer - RTT Alarm Enable"]
pub type RttalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAL` reader - RTC Alarm Enable"]
pub type RtcalR = crate::BitReader;
#[doc = "Field `RTCAL` writer - RTC Alarm Enable"]
pub type RtcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBAL` reader - USB Alarm Enable"]
pub type UsbalR = crate::BitReader;
#[doc = "Field `USBAL` writer - USB Alarm Enable"]
pub type UsbalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM` reader - Low-power Mode"]
pub type LpmR = crate::BitReader;
#[doc = "Field `LPM` writer - Low-power Mode"]
pub type LpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Flash Low-power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flpmselect {
    #[doc = "0: Flash is in Standby Mode when system enters Wait Mode"]
    FlashStandby = 0,
    #[doc = "1: Flash is in Deep-power-down mode when system enters Wait Mode"]
    FlashDeepPowerdown = 1,
    #[doc = "2: Idle mode"]
    FlashIdle = 2,
}
impl From<Flpmselect> for u8 {
    #[inline(always)]
    fn from(variant: Flpmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flpmselect {
    type Ux = u8;
}
impl crate::IsEnum for Flpmselect {}
#[doc = "Field `FLPM` reader - Flash Low-power Mode"]
pub type FlpmR = crate::FieldReader<Flpmselect>;
impl FlpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flpmselect> {
        match self.bits {
            0 => Some(Flpmselect::FlashStandby),
            1 => Some(Flpmselect::FlashDeepPowerdown),
            2 => Some(Flpmselect::FlashIdle),
            _ => None,
        }
    }
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn is_flash_standby(&self) -> bool {
        *self == Flpmselect::FlashStandby
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn is_flash_deep_powerdown(&self) -> bool {
        *self == Flpmselect::FlashDeepPowerdown
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn is_flash_idle(&self) -> bool {
        *self == Flpmselect::FlashIdle
    }
}
#[doc = "Field `FLPM` writer - Flash Low-power Mode"]
pub type FlpmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flpmselect>;
impl<'a, REG> FlpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_standby(self) -> &'a mut crate::W<REG> {
        self.variant(Flpmselect::FlashStandby)
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_deep_powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Flpmselect::FlashDeepPowerdown)
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn flash_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Flpmselect::FlashIdle)
    }
}
#[doc = "Field `FFLPM` reader - Force Flash Low-power Mode"]
pub type FflpmR = crate::BitReader;
#[doc = "Field `FFLPM` writer - Force Flash Low-power Mode"]
pub type FflpmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> Fstt0R {
        Fstt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> Fstt1R {
        Fstt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> Fstt2R {
        Fstt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> Fstt3R {
        Fstt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> Fstt4R {
        Fstt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> Fstt5R {
        Fstt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> Fstt6R {
        Fstt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> Fstt7R {
        Fstt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> Fstt8R {
        Fstt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> Fstt9R {
        Fstt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> Fstt10R {
        Fstt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> Fstt11R {
        Fstt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> Fstt12R {
        Fstt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> Fstt13R {
        Fstt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> Fstt14R {
        Fstt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> Fstt15R {
        Fstt15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RttalR {
        RttalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RtcalR {
        RtcalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> UsbalR {
        UsbalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LpmR {
        LpmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&self) -> FlpmR {
        FlpmR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&self) -> FflpmR {
        FflpmR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&mut self) -> Fstt0W<FsmrSpec> {
        Fstt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&mut self) -> Fstt1W<FsmrSpec> {
        Fstt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&mut self) -> Fstt2W<FsmrSpec> {
        Fstt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&mut self) -> Fstt3W<FsmrSpec> {
        Fstt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&mut self) -> Fstt4W<FsmrSpec> {
        Fstt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&mut self) -> Fstt5W<FsmrSpec> {
        Fstt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&mut self) -> Fstt6W<FsmrSpec> {
        Fstt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&mut self) -> Fstt7W<FsmrSpec> {
        Fstt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&mut self) -> Fstt8W<FsmrSpec> {
        Fstt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&mut self) -> Fstt9W<FsmrSpec> {
        Fstt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&mut self) -> Fstt10W<FsmrSpec> {
        Fstt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&mut self) -> Fstt11W<FsmrSpec> {
        Fstt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&mut self) -> Fstt12W<FsmrSpec> {
        Fstt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&mut self) -> Fstt13W<FsmrSpec> {
        Fstt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&mut self) -> Fstt14W<FsmrSpec> {
        Fstt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&mut self) -> Fstt15W<FsmrSpec> {
        Fstt15W::new(self, 15)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&mut self) -> RttalW<FsmrSpec> {
        RttalW::new(self, 16)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&mut self) -> RtcalW<FsmrSpec> {
        RtcalW::new(self, 17)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&mut self) -> UsbalW<FsmrSpec> {
        UsbalW::new(self, 18)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LpmW<FsmrSpec> {
        LpmW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&mut self) -> FlpmW<FsmrSpec> {
        FlpmW::new(self, 21)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&mut self) -> FflpmW<FsmrSpec> {
        FflpmW::new(self, 23)
    }
}
#[doc = "Fast Startup Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmrSpec;
impl crate::RegisterSpec for FsmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsmr::R`](R) reader structure"]
impl crate::Readable for FsmrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsmr::W`](W) writer structure"]
impl crate::Writable for FsmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSMR to value 0"]
impl crate::Resettable for FsmrSpec {}
