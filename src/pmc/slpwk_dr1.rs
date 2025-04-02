#[doc = "Register `SLPWK_DR1` writer"]
pub type W = crate::W<SlpwkDr1Spec>;
#[doc = "Field `PID32` writer - Peripheral 32 SleepWalking Disable"]
pub type Pid32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral 33 SleepWalking Disable"]
pub type Pid33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral 34 SleepWalking Disable"]
pub type Pid34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID35` writer - Peripheral 35 SleepWalking Disable"]
pub type Pid35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID37` writer - Peripheral 37 SleepWalking Disable"]
pub type Pid37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID39` writer - Peripheral 39 SleepWalking Disable"]
pub type Pid39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID40` writer - Peripheral 40 SleepWalking Disable"]
pub type Pid40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID41` writer - Peripheral 41 SleepWalking Disable"]
pub type Pid41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID42` writer - Peripheral 42 SleepWalking Disable"]
pub type Pid42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID43` writer - Peripheral 43 SleepWalking Disable"]
pub type Pid43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID44` writer - Peripheral 44 SleepWalking Disable"]
pub type Pid44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID45` writer - Peripheral 45 SleepWalking Disable"]
pub type Pid45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID46` writer - Peripheral 46 SleepWalking Disable"]
pub type Pid46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID47` writer - Peripheral 47 SleepWalking Disable"]
pub type Pid47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID48` writer - Peripheral 48 SleepWalking Disable"]
pub type Pid48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID49` writer - Peripheral 49 SleepWalking Disable"]
pub type Pid49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID50` writer - Peripheral 50 SleepWalking Disable"]
pub type Pid50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID51` writer - Peripheral 51 SleepWalking Disable"]
pub type Pid51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID52` writer - Peripheral 52 SleepWalking Disable"]
pub type Pid52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID53` writer - Peripheral 53 SleepWalking Disable"]
pub type Pid53W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID56` writer - Peripheral 56 SleepWalking Disable"]
pub type Pid56W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID57` writer - Peripheral 57 SleepWalking Disable"]
pub type Pid57W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID58` writer - Peripheral 58 SleepWalking Disable"]
pub type Pid58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID59` writer - Peripheral 59 SleepWalking Disable"]
pub type Pid59W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID60` writer - Peripheral 60 SleepWalking Disable"]
pub type Pid60W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Peripheral 32 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> Pid32W<SlpwkDr1Spec> {
        Pid32W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral 33 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> Pid33W<SlpwkDr1Spec> {
        Pid33W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral 34 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> Pid34W<SlpwkDr1Spec> {
        Pid34W::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral 35 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid35(&mut self) -> Pid35W<SlpwkDr1Spec> {
        Pid35W::new(self, 3)
    }
    #[doc = "Bit 5 - Peripheral 37 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid37(&mut self) -> Pid37W<SlpwkDr1Spec> {
        Pid37W::new(self, 5)
    }
    #[doc = "Bit 7 - Peripheral 39 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid39(&mut self) -> Pid39W<SlpwkDr1Spec> {
        Pid39W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral 40 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid40(&mut self) -> Pid40W<SlpwkDr1Spec> {
        Pid40W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral 41 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid41(&mut self) -> Pid41W<SlpwkDr1Spec> {
        Pid41W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral 42 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid42(&mut self) -> Pid42W<SlpwkDr1Spec> {
        Pid42W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral 43 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid43(&mut self) -> Pid43W<SlpwkDr1Spec> {
        Pid43W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral 44 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid44(&mut self) -> Pid44W<SlpwkDr1Spec> {
        Pid44W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral 45 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid45(&mut self) -> Pid45W<SlpwkDr1Spec> {
        Pid45W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral 46 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid46(&mut self) -> Pid46W<SlpwkDr1Spec> {
        Pid46W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral 47 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> Pid47W<SlpwkDr1Spec> {
        Pid47W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral 48 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid48(&mut self) -> Pid48W<SlpwkDr1Spec> {
        Pid48W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral 49 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid49(&mut self) -> Pid49W<SlpwkDr1Spec> {
        Pid49W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral 50 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid50(&mut self) -> Pid50W<SlpwkDr1Spec> {
        Pid50W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral 51 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid51(&mut self) -> Pid51W<SlpwkDr1Spec> {
        Pid51W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral 52 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid52(&mut self) -> Pid52W<SlpwkDr1Spec> {
        Pid52W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral 53 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid53(&mut self) -> Pid53W<SlpwkDr1Spec> {
        Pid53W::new(self, 21)
    }
    #[doc = "Bit 24 - Peripheral 56 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid56(&mut self) -> Pid56W<SlpwkDr1Spec> {
        Pid56W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral 57 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid57(&mut self) -> Pid57W<SlpwkDr1Spec> {
        Pid57W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral 58 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid58(&mut self) -> Pid58W<SlpwkDr1Spec> {
        Pid58W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral 59 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid59(&mut self) -> Pid59W<SlpwkDr1Spec> {
        Pid59W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral 60 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid60(&mut self) -> Pid60W<SlpwkDr1Spec> {
        Pid60W::new(self, 28)
    }
}
#[doc = "SleepWalking Disable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slpwk_dr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpwkDr1Spec;
impl crate::RegisterSpec for SlpwkDr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slpwk_dr1::W`](W) writer structure"]
impl crate::Writable for SlpwkDr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLPWK_DR1 to value 0"]
impl crate::Resettable for SlpwkDr1Spec {}
