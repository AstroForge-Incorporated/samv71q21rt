#[doc = "Register `PCDR0` writer"]
pub type W = crate::W<Pcdr0Spec>;
#[doc = "Field `PID7` writer - Peripheral Clock 7 Disable"]
pub type Pid7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID8` writer - Peripheral Clock 8 Disable"]
pub type Pid8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID9` writer - Peripheral Clock 9 Disable"]
pub type Pid9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID10` writer - Peripheral Clock 10 Disable"]
pub type Pid10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID11` writer - Peripheral Clock 11 Disable"]
pub type Pid11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID12` writer - Peripheral Clock 12 Disable"]
pub type Pid12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID13` writer - Peripheral Clock 13 Disable"]
pub type Pid13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID14` writer - Peripheral Clock 14 Disable"]
pub type Pid14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID15` writer - Peripheral Clock 15 Disable"]
pub type Pid15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID16` writer - Peripheral Clock 16 Disable"]
pub type Pid16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID17` writer - Peripheral Clock 17 Disable"]
pub type Pid17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID18` writer - Peripheral Clock 18 Disable"]
pub type Pid18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID19` writer - Peripheral Clock 19 Disable"]
pub type Pid19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID20` writer - Peripheral Clock 20 Disable"]
pub type Pid20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID21` writer - Peripheral Clock 21 Disable"]
pub type Pid21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID22` writer - Peripheral Clock 22 Disable"]
pub type Pid22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID23` writer - Peripheral Clock 23 Disable"]
pub type Pid23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID24` writer - Peripheral Clock 24 Disable"]
pub type Pid24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID25` writer - Peripheral Clock 25 Disable"]
pub type Pid25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID26` writer - Peripheral Clock 26 Disable"]
pub type Pid26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID27` writer - Peripheral Clock 27 Disable"]
pub type Pid27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID28` writer - Peripheral Clock 28 Disable"]
pub type Pid28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID29` writer - Peripheral Clock 29 Disable"]
pub type Pid29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID30` writer - Peripheral Clock 30 Disable"]
pub type Pid30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID31` writer - Peripheral Clock 31 Disable"]
pub type Pid31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 7 - Peripheral Clock 7 Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> Pid7W<Pcdr0Spec> {
        Pid7W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Clock 8 Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> Pid8W<Pcdr0Spec> {
        Pid8W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 9 Disable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> Pid9W<Pcdr0Spec> {
        Pid9W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> Pid10W<Pcdr0Spec> {
        Pid10W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> Pid11W<Pcdr0Spec> {
        Pid11W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 12 Disable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> Pid12W<Pcdr0Spec> {
        Pid12W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> Pid13W<Pcdr0Spec> {
        Pid13W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> Pid14W<Pcdr0Spec> {
        Pid14W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> Pid15W<Pcdr0Spec> {
        Pid15W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> Pid16W<Pcdr0Spec> {
        Pid16W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral Clock 17 Disable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> Pid17W<Pcdr0Spec> {
        Pid17W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> Pid18W<Pcdr0Spec> {
        Pid18W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> Pid19W<Pcdr0Spec> {
        Pid19W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> Pid20W<Pcdr0Spec> {
        Pid20W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> Pid21W<Pcdr0Spec> {
        Pid21W::new(self, 21)
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> Pid22W<Pcdr0Spec> {
        Pid22W::new(self, 22)
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> Pid23W<Pcdr0Spec> {
        Pid23W::new(self, 23)
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> Pid24W<Pcdr0Spec> {
        Pid24W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> Pid25W<Pcdr0Spec> {
        Pid25W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> Pid26W<Pcdr0Spec> {
        Pid26W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> Pid27W<Pcdr0Spec> {
        Pid27W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> Pid28W<Pcdr0Spec> {
        Pid28W::new(self, 28)
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> Pid29W<Pcdr0Spec> {
        Pid29W::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> Pid30W<Pcdr0Spec> {
        Pid30W::new(self, 30)
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> Pid31W<Pcdr0Spec> {
        Pid31W::new(self, 31)
    }
}
#[doc = "Peripheral Clock Disable Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdr0Spec;
impl crate::RegisterSpec for Pcdr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcdr0::W`](W) writer structure"]
impl crate::Writable for Pcdr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCDR0 to value 0"]
impl crate::Resettable for Pcdr0Spec {}
