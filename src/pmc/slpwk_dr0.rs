#[doc = "Register `SLPWK_DR0` writer"]
pub type W = crate::W<SlpwkDr0Spec>;
#[doc = "Field `PID7` writer - Peripheral 7 SleepWalking Disable"]
pub type Pid7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID8` writer - Peripheral 8 SleepWalking Disable"]
pub type Pid8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID9` writer - Peripheral 9 SleepWalking Disable"]
pub type Pid9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID10` writer - Peripheral 10 SleepWalking Disable"]
pub type Pid10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID11` writer - Peripheral 11 SleepWalking Disable"]
pub type Pid11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID12` writer - Peripheral 12 SleepWalking Disable"]
pub type Pid12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID13` writer - Peripheral 13 SleepWalking Disable"]
pub type Pid13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID14` writer - Peripheral 14 SleepWalking Disable"]
pub type Pid14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID15` writer - Peripheral 15 SleepWalking Disable"]
pub type Pid15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID16` writer - Peripheral 16 SleepWalking Disable"]
pub type Pid16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID17` writer - Peripheral 17 SleepWalking Disable"]
pub type Pid17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID18` writer - Peripheral 18 SleepWalking Disable"]
pub type Pid18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID19` writer - Peripheral 19 SleepWalking Disable"]
pub type Pid19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID20` writer - Peripheral 20 SleepWalking Disable"]
pub type Pid20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID21` writer - Peripheral 21 SleepWalking Disable"]
pub type Pid21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID22` writer - Peripheral 22 SleepWalking Disable"]
pub type Pid22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID23` writer - Peripheral 23 SleepWalking Disable"]
pub type Pid23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID24` writer - Peripheral 24 SleepWalking Disable"]
pub type Pid24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID25` writer - Peripheral 25 SleepWalking Disable"]
pub type Pid25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID26` writer - Peripheral 26 SleepWalking Disable"]
pub type Pid26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID27` writer - Peripheral 27 SleepWalking Disable"]
pub type Pid27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID28` writer - Peripheral 28 SleepWalking Disable"]
pub type Pid28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID29` writer - Peripheral 29 SleepWalking Disable"]
pub type Pid29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID30` writer - Peripheral 30 SleepWalking Disable"]
pub type Pid30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID31` writer - Peripheral 31 SleepWalking Disable"]
pub type Pid31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> Pid7W<SlpwkDr0Spec> {
        Pid7W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> Pid8W<SlpwkDr0Spec> {
        Pid8W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral 9 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> Pid9W<SlpwkDr0Spec> {
        Pid9W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> Pid10W<SlpwkDr0Spec> {
        Pid10W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> Pid11W<SlpwkDr0Spec> {
        Pid11W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral 12 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> Pid12W<SlpwkDr0Spec> {
        Pid12W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> Pid13W<SlpwkDr0Spec> {
        Pid13W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> Pid14W<SlpwkDr0Spec> {
        Pid14W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral 15 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> Pid15W<SlpwkDr0Spec> {
        Pid15W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> Pid16W<SlpwkDr0Spec> {
        Pid16W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral 17 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> Pid17W<SlpwkDr0Spec> {
        Pid17W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral 18 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> Pid18W<SlpwkDr0Spec> {
        Pid18W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> Pid19W<SlpwkDr0Spec> {
        Pid19W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> Pid20W<SlpwkDr0Spec> {
        Pid20W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral 21 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> Pid21W<SlpwkDr0Spec> {
        Pid21W::new(self, 21)
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> Pid22W<SlpwkDr0Spec> {
        Pid22W::new(self, 22)
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> Pid23W<SlpwkDr0Spec> {
        Pid23W::new(self, 23)
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> Pid24W<SlpwkDr0Spec> {
        Pid24W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> Pid25W<SlpwkDr0Spec> {
        Pid25W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> Pid26W<SlpwkDr0Spec> {
        Pid26W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> Pid27W<SlpwkDr0Spec> {
        Pid27W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> Pid28W<SlpwkDr0Spec> {
        Pid28W::new(self, 28)
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> Pid29W<SlpwkDr0Spec> {
        Pid29W::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> Pid30W<SlpwkDr0Spec> {
        Pid30W::new(self, 30)
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> Pid31W<SlpwkDr0Spec> {
        Pid31W::new(self, 31)
    }
}
#[doc = "SleepWalking Disable Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slpwk_dr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpwkDr0Spec;
impl crate::RegisterSpec for SlpwkDr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slpwk_dr0::W`](W) writer structure"]
impl crate::Writable for SlpwkDr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLPWK_DR0 to value 0"]
impl crate::Resettable for SlpwkDr0Spec {}
