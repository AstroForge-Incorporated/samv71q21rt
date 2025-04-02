#[doc = "Register `SLPWK_SR0` reader"]
pub type R = crate::R<SlpwkSr0Spec>;
#[doc = "Field `PID7` reader - Peripheral 7 SleepWalking Status"]
pub type Pid7R = crate::BitReader;
#[doc = "Field `PID8` reader - Peripheral 8 SleepWalking Status"]
pub type Pid8R = crate::BitReader;
#[doc = "Field `PID9` reader - Peripheral 9 SleepWalking Status"]
pub type Pid9R = crate::BitReader;
#[doc = "Field `PID10` reader - Peripheral 10 SleepWalking Status"]
pub type Pid10R = crate::BitReader;
#[doc = "Field `PID11` reader - Peripheral 11 SleepWalking Status"]
pub type Pid11R = crate::BitReader;
#[doc = "Field `PID12` reader - Peripheral 12 SleepWalking Status"]
pub type Pid12R = crate::BitReader;
#[doc = "Field `PID13` reader - Peripheral 13 SleepWalking Status"]
pub type Pid13R = crate::BitReader;
#[doc = "Field `PID14` reader - Peripheral 14 SleepWalking Status"]
pub type Pid14R = crate::BitReader;
#[doc = "Field `PID15` reader - Peripheral 15 SleepWalking Status"]
pub type Pid15R = crate::BitReader;
#[doc = "Field `PID16` reader - Peripheral 16 SleepWalking Status"]
pub type Pid16R = crate::BitReader;
#[doc = "Field `PID17` reader - Peripheral 17 SleepWalking Status"]
pub type Pid17R = crate::BitReader;
#[doc = "Field `PID18` reader - Peripheral 18 SleepWalking Status"]
pub type Pid18R = crate::BitReader;
#[doc = "Field `PID19` reader - Peripheral 19 SleepWalking Status"]
pub type Pid19R = crate::BitReader;
#[doc = "Field `PID20` reader - Peripheral 20 SleepWalking Status"]
pub type Pid20R = crate::BitReader;
#[doc = "Field `PID21` reader - Peripheral 21 SleepWalking Status"]
pub type Pid21R = crate::BitReader;
#[doc = "Field `PID22` reader - Peripheral 22 SleepWalking Status"]
pub type Pid22R = crate::BitReader;
#[doc = "Field `PID23` reader - Peripheral 23 SleepWalking Status"]
pub type Pid23R = crate::BitReader;
#[doc = "Field `PID24` reader - Peripheral 24 SleepWalking Status"]
pub type Pid24R = crate::BitReader;
#[doc = "Field `PID25` reader - Peripheral 25 SleepWalking Status"]
pub type Pid25R = crate::BitReader;
#[doc = "Field `PID26` reader - Peripheral 26 SleepWalking Status"]
pub type Pid26R = crate::BitReader;
#[doc = "Field `PID27` reader - Peripheral 27 SleepWalking Status"]
pub type Pid27R = crate::BitReader;
#[doc = "Field `PID28` reader - Peripheral 28 SleepWalking Status"]
pub type Pid28R = crate::BitReader;
#[doc = "Field `PID29` reader - Peripheral 29 SleepWalking Status"]
pub type Pid29R = crate::BitReader;
#[doc = "Field `PID30` reader - Peripheral 30 SleepWalking Status"]
pub type Pid30R = crate::BitReader;
#[doc = "Field `PID31` reader - Peripheral 31 SleepWalking Status"]
pub type Pid31R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Status"]
    #[inline(always)]
    pub fn pid7(&self) -> Pid7R {
        Pid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Status"]
    #[inline(always)]
    pub fn pid8(&self) -> Pid8R {
        Pid8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral 9 SleepWalking Status"]
    #[inline(always)]
    pub fn pid9(&self) -> Pid9R {
        Pid9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Status"]
    #[inline(always)]
    pub fn pid10(&self) -> Pid10R {
        Pid10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Status"]
    #[inline(always)]
    pub fn pid11(&self) -> Pid11R {
        Pid11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral 12 SleepWalking Status"]
    #[inline(always)]
    pub fn pid12(&self) -> Pid12R {
        Pid12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Status"]
    #[inline(always)]
    pub fn pid13(&self) -> Pid13R {
        Pid13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Status"]
    #[inline(always)]
    pub fn pid14(&self) -> Pid14R {
        Pid14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral 15 SleepWalking Status"]
    #[inline(always)]
    pub fn pid15(&self) -> Pid15R {
        Pid15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Status"]
    #[inline(always)]
    pub fn pid16(&self) -> Pid16R {
        Pid16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral 17 SleepWalking Status"]
    #[inline(always)]
    pub fn pid17(&self) -> Pid17R {
        Pid17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral 18 SleepWalking Status"]
    #[inline(always)]
    pub fn pid18(&self) -> Pid18R {
        Pid18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Status"]
    #[inline(always)]
    pub fn pid19(&self) -> Pid19R {
        Pid19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Status"]
    #[inline(always)]
    pub fn pid20(&self) -> Pid20R {
        Pid20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral 21 SleepWalking Status"]
    #[inline(always)]
    pub fn pid21(&self) -> Pid21R {
        Pid21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Status"]
    #[inline(always)]
    pub fn pid22(&self) -> Pid22R {
        Pid22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Status"]
    #[inline(always)]
    pub fn pid23(&self) -> Pid23R {
        Pid23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Status"]
    #[inline(always)]
    pub fn pid24(&self) -> Pid24R {
        Pid24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Status"]
    #[inline(always)]
    pub fn pid25(&self) -> Pid25R {
        Pid25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Status"]
    #[inline(always)]
    pub fn pid26(&self) -> Pid26R {
        Pid26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Status"]
    #[inline(always)]
    pub fn pid27(&self) -> Pid27R {
        Pid27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Status"]
    #[inline(always)]
    pub fn pid28(&self) -> Pid28R {
        Pid28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Status"]
    #[inline(always)]
    pub fn pid29(&self) -> Pid29R {
        Pid29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Status"]
    #[inline(always)]
    pub fn pid30(&self) -> Pid30R {
        Pid30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Status"]
    #[inline(always)]
    pub fn pid31(&self) -> Pid31R {
        Pid31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SleepWalking Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_sr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpwkSr0Spec;
impl crate::RegisterSpec for SlpwkSr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slpwk_sr0::R`](R) reader structure"]
impl crate::Readable for SlpwkSr0Spec {}
#[doc = "`reset()` method sets SLPWK_SR0 to value 0"]
impl crate::Resettable for SlpwkSr0Spec {}
