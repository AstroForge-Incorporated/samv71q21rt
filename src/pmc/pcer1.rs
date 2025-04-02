#[doc = "Register `PCER1` writer"]
pub type W = crate::W<Pcer1Spec>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Enable"]
pub type Pid32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Enable"]
pub type Pid33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Enable"]
pub type Pid34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Enable"]
pub type Pid35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Enable"]
pub type Pid37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Enable"]
pub type Pid39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Enable"]
pub type Pid40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Enable"]
pub type Pid41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Enable"]
pub type Pid42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Enable"]
pub type Pid43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Enable"]
pub type Pid44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID45` writer - Peripheral Clock 45 Enable"]
pub type Pid45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID46` writer - Peripheral Clock 46 Enable"]
pub type Pid46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID47` writer - Peripheral Clock 47 Enable"]
pub type Pid47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID48` writer - Peripheral Clock 48 Enable"]
pub type Pid48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID49` writer - Peripheral Clock 49 Enable"]
pub type Pid49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID50` writer - Peripheral Clock 50 Enable"]
pub type Pid50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID51` writer - Peripheral Clock 51 Enable"]
pub type Pid51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID52` writer - Peripheral Clock 52 Enable"]
pub type Pid52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID53` writer - Peripheral Clock 53 Enable"]
pub type Pid53W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID56` writer - Peripheral Clock 56 Enable"]
pub type Pid56W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID57` writer - Peripheral Clock 57 Enable"]
pub type Pid57W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID58` writer - Peripheral Clock 58 Enable"]
pub type Pid58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID59` writer - Peripheral Clock 59 Enable"]
pub type Pid59W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID60` writer - Peripheral Clock 60 Enable"]
pub type Pid60W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Enable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> Pid32W<Pcer1Spec> {
        Pid32W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Enable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> Pid33W<Pcer1Spec> {
        Pid33W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Enable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> Pid34W<Pcer1Spec> {
        Pid34W::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Enable"]
    #[inline(always)]
    pub fn pid35(&mut self) -> Pid35W<Pcer1Spec> {
        Pid35W::new(self, 3)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Enable"]
    #[inline(always)]
    pub fn pid37(&mut self) -> Pid37W<Pcer1Spec> {
        Pid37W::new(self, 5)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Enable"]
    #[inline(always)]
    pub fn pid39(&mut self) -> Pid39W<Pcer1Spec> {
        Pid39W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Enable"]
    #[inline(always)]
    pub fn pid40(&mut self) -> Pid40W<Pcer1Spec> {
        Pid40W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Enable"]
    #[inline(always)]
    pub fn pid41(&mut self) -> Pid41W<Pcer1Spec> {
        Pid41W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Enable"]
    #[inline(always)]
    pub fn pid42(&mut self) -> Pid42W<Pcer1Spec> {
        Pid42W::new(self, 10)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Enable"]
    #[inline(always)]
    pub fn pid43(&mut self) -> Pid43W<Pcer1Spec> {
        Pid43W::new(self, 11)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Enable"]
    #[inline(always)]
    pub fn pid44(&mut self) -> Pid44W<Pcer1Spec> {
        Pid44W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Enable"]
    #[inline(always)]
    pub fn pid45(&mut self) -> Pid45W<Pcer1Spec> {
        Pid45W::new(self, 13)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Enable"]
    #[inline(always)]
    pub fn pid46(&mut self) -> Pid46W<Pcer1Spec> {
        Pid46W::new(self, 14)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Enable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> Pid47W<Pcer1Spec> {
        Pid47W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Enable"]
    #[inline(always)]
    pub fn pid48(&mut self) -> Pid48W<Pcer1Spec> {
        Pid48W::new(self, 16)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Enable"]
    #[inline(always)]
    pub fn pid49(&mut self) -> Pid49W<Pcer1Spec> {
        Pid49W::new(self, 17)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Enable"]
    #[inline(always)]
    pub fn pid50(&mut self) -> Pid50W<Pcer1Spec> {
        Pid50W::new(self, 18)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Enable"]
    #[inline(always)]
    pub fn pid51(&mut self) -> Pid51W<Pcer1Spec> {
        Pid51W::new(self, 19)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Enable"]
    #[inline(always)]
    pub fn pid52(&mut self) -> Pid52W<Pcer1Spec> {
        Pid52W::new(self, 20)
    }
    #[doc = "Bit 21 - Peripheral Clock 53 Enable"]
    #[inline(always)]
    pub fn pid53(&mut self) -> Pid53W<Pcer1Spec> {
        Pid53W::new(self, 21)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Enable"]
    #[inline(always)]
    pub fn pid56(&mut self) -> Pid56W<Pcer1Spec> {
        Pid56W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Enable"]
    #[inline(always)]
    pub fn pid57(&mut self) -> Pid57W<Pcer1Spec> {
        Pid57W::new(self, 25)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Enable"]
    #[inline(always)]
    pub fn pid58(&mut self) -> Pid58W<Pcer1Spec> {
        Pid58W::new(self, 26)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Enable"]
    #[inline(always)]
    pub fn pid59(&mut self) -> Pid59W<Pcer1Spec> {
        Pid59W::new(self, 27)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Enable"]
    #[inline(always)]
    pub fn pid60(&mut self) -> Pid60W<Pcer1Spec> {
        Pid60W::new(self, 28)
    }
}
#[doc = "Peripheral Clock Enable Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcer1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcer1Spec;
impl crate::RegisterSpec for Pcer1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcer1::W`](W) writer structure"]
impl crate::Writable for Pcer1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCER1 to value 0"]
impl crate::Resettable for Pcer1Spec {}
