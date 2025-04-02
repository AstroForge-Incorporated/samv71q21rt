#[doc = "Register `PCDR1` writer"]
pub type W = crate::W<Pcdr1Spec>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Disable"]
pub type Pid32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Disable"]
pub type Pid33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Disable"]
pub type Pid34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Disable"]
pub type Pid35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Disable"]
pub type Pid37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Disable"]
pub type Pid39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Disable"]
pub type Pid40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Disable"]
pub type Pid41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Disable"]
pub type Pid42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Disable"]
pub type Pid43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Disable"]
pub type Pid44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID45` writer - Peripheral Clock 45 Disable"]
pub type Pid45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID46` writer - Peripheral Clock 46 Disable"]
pub type Pid46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID47` writer - Peripheral Clock 47 Disable"]
pub type Pid47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID48` writer - Peripheral Clock 48 Disable"]
pub type Pid48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID49` writer - Peripheral Clock 49 Disable"]
pub type Pid49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID50` writer - Peripheral Clock 50 Disable"]
pub type Pid50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID51` writer - Peripheral Clock 51 Disable"]
pub type Pid51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID52` writer - Peripheral Clock 52 Disable"]
pub type Pid52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID53` writer - Peripheral Clock 53 Disable"]
pub type Pid53W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID56` writer - Peripheral Clock 56 Disable"]
pub type Pid56W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID57` writer - Peripheral Clock 57 Disable"]
pub type Pid57W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID58` writer - Peripheral Clock 58 Disable"]
pub type Pid58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID59` writer - Peripheral Clock 59 Disable"]
pub type Pid59W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID60` writer - Peripheral Clock 60 Disable"]
pub type Pid60W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Disable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> Pid32W<Pcdr1Spec> {
        Pid32W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Disable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> Pid33W<Pcdr1Spec> {
        Pid33W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Disable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> Pid34W<Pcdr1Spec> {
        Pid34W::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Disable"]
    #[inline(always)]
    pub fn pid35(&mut self) -> Pid35W<Pcdr1Spec> {
        Pid35W::new(self, 3)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Disable"]
    #[inline(always)]
    pub fn pid37(&mut self) -> Pid37W<Pcdr1Spec> {
        Pid37W::new(self, 5)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Disable"]
    #[inline(always)]
    pub fn pid39(&mut self) -> Pid39W<Pcdr1Spec> {
        Pid39W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Disable"]
    #[inline(always)]
    pub fn pid40(&mut self) -> Pid40W<Pcdr1Spec> {
        Pid40W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Disable"]
    #[inline(always)]
    pub fn pid41(&mut self) -> Pid41W<Pcdr1Spec> {
        Pid41W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Disable"]
    #[inline(always)]
    pub fn pid42(&mut self) -> Pid42W<Pcdr1Spec> {
        Pid42W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Disable"]
    #[inline(always)]
    pub fn pid43(&mut self) -> Pid43W<Pcdr1Spec> {
        Pid43W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Disable"]
    #[inline(always)]
    pub fn pid44(&mut self) -> Pid44W<Pcdr1Spec> {
        Pid44W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Disable"]
    #[inline(always)]
    pub fn pid45(&mut self) -> Pid45W<Pcdr1Spec> {
        Pid45W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Disable"]
    #[inline(always)]
    pub fn pid46(&mut self) -> Pid46W<Pcdr1Spec> {
        Pid46W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Disable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> Pid47W<Pcdr1Spec> {
        Pid47W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Disable"]
    #[inline(always)]
    pub fn pid48(&mut self) -> Pid48W<Pcdr1Spec> {
        Pid48W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Disable"]
    #[inline(always)]
    pub fn pid49(&mut self) -> Pid49W<Pcdr1Spec> {
        Pid49W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Disable"]
    #[inline(always)]
    pub fn pid50(&mut self) -> Pid50W<Pcdr1Spec> {
        Pid50W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Disable"]
    #[inline(always)]
    pub fn pid51(&mut self) -> Pid51W<Pcdr1Spec> {
        Pid51W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Disable"]
    #[inline(always)]
    pub fn pid52(&mut self) -> Pid52W<Pcdr1Spec> {
        Pid52W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Clock 53 Disable"]
    #[inline(always)]
    pub fn pid53(&mut self) -> Pid53W<Pcdr1Spec> {
        Pid53W::new(self, 21)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Disable"]
    #[inline(always)]
    pub fn pid56(&mut self) -> Pid56W<Pcdr1Spec> {
        Pid56W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Disable"]
    #[inline(always)]
    pub fn pid57(&mut self) -> Pid57W<Pcdr1Spec> {
        Pid57W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Disable"]
    #[inline(always)]
    pub fn pid58(&mut self) -> Pid58W<Pcdr1Spec> {
        Pid58W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Disable"]
    #[inline(always)]
    pub fn pid59(&mut self) -> Pid59W<Pcdr1Spec> {
        Pid59W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Disable"]
    #[inline(always)]
    pub fn pid60(&mut self) -> Pid60W<Pcdr1Spec> {
        Pid60W::new(self, 28)
    }
}
#[doc = "Peripheral Clock Disable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcdr1Spec;
impl crate::RegisterSpec for Pcdr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcdr1::W`](W) writer structure"]
impl crate::Writable for Pcdr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCDR1 to value 0"]
impl crate::Resettable for Pcdr1Spec {}
