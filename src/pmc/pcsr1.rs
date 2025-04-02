#[doc = "Register `PCSR1` reader"]
pub type R = crate::R<Pcsr1Spec>;
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub type Pid32R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub type Pid33R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub type Pid34R = crate::BitReader;
#[doc = "Field `PID35` reader - Peripheral Clock 35 Status"]
pub type Pid35R = crate::BitReader;
#[doc = "Field `PID37` reader - Peripheral Clock 37 Status"]
pub type Pid37R = crate::BitReader;
#[doc = "Field `PID39` reader - Peripheral Clock 39 Status"]
pub type Pid39R = crate::BitReader;
#[doc = "Field `PID40` reader - Peripheral Clock 40 Status"]
pub type Pid40R = crate::BitReader;
#[doc = "Field `PID41` reader - Peripheral Clock 41 Status"]
pub type Pid41R = crate::BitReader;
#[doc = "Field `PID42` reader - Peripheral Clock 42 Status"]
pub type Pid42R = crate::BitReader;
#[doc = "Field `PID43` reader - Peripheral Clock 43 Status"]
pub type Pid43R = crate::BitReader;
#[doc = "Field `PID44` reader - Peripheral Clock 44 Status"]
pub type Pid44R = crate::BitReader;
#[doc = "Field `PID45` reader - Peripheral Clock 45 Status"]
pub type Pid45R = crate::BitReader;
#[doc = "Field `PID46` reader - Peripheral Clock 46 Status"]
pub type Pid46R = crate::BitReader;
#[doc = "Field `PID47` reader - Peripheral Clock 47 Status"]
pub type Pid47R = crate::BitReader;
#[doc = "Field `PID48` reader - Peripheral Clock 48 Status"]
pub type Pid48R = crate::BitReader;
#[doc = "Field `PID49` reader - Peripheral Clock 49 Status"]
pub type Pid49R = crate::BitReader;
#[doc = "Field `PID50` reader - Peripheral Clock 50 Status"]
pub type Pid50R = crate::BitReader;
#[doc = "Field `PID51` reader - Peripheral Clock 51 Status"]
pub type Pid51R = crate::BitReader;
#[doc = "Field `PID52` reader - Peripheral Clock 52 Status"]
pub type Pid52R = crate::BitReader;
#[doc = "Field `PID53` reader - Peripheral Clock 53 Status"]
pub type Pid53R = crate::BitReader;
#[doc = "Field `PID56` reader - Peripheral Clock 56 Status"]
pub type Pid56R = crate::BitReader;
#[doc = "Field `PID57` reader - Peripheral Clock 57 Status"]
pub type Pid57R = crate::BitReader;
#[doc = "Field `PID58` reader - Peripheral Clock 58 Status"]
pub type Pid58R = crate::BitReader;
#[doc = "Field `PID59` reader - Peripheral Clock 59 Status"]
pub type Pid59R = crate::BitReader;
#[doc = "Field `PID60` reader - Peripheral Clock 60 Status"]
pub type Pid60R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> Pid32R {
        Pid32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> Pid33R {
        Pid33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> Pid34R {
        Pid34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Status"]
    #[inline(always)]
    pub fn pid35(&self) -> Pid35R {
        Pid35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Status"]
    #[inline(always)]
    pub fn pid37(&self) -> Pid37R {
        Pid37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Status"]
    #[inline(always)]
    pub fn pid39(&self) -> Pid39R {
        Pid39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Status"]
    #[inline(always)]
    pub fn pid40(&self) -> Pid40R {
        Pid40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Status"]
    #[inline(always)]
    pub fn pid41(&self) -> Pid41R {
        Pid41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Status"]
    #[inline(always)]
    pub fn pid42(&self) -> Pid42R {
        Pid42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Status"]
    #[inline(always)]
    pub fn pid43(&self) -> Pid43R {
        Pid43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Status"]
    #[inline(always)]
    pub fn pid44(&self) -> Pid44R {
        Pid44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Status"]
    #[inline(always)]
    pub fn pid45(&self) -> Pid45R {
        Pid45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Status"]
    #[inline(always)]
    pub fn pid46(&self) -> Pid46R {
        Pid46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Status"]
    #[inline(always)]
    pub fn pid47(&self) -> Pid47R {
        Pid47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Status"]
    #[inline(always)]
    pub fn pid48(&self) -> Pid48R {
        Pid48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Status"]
    #[inline(always)]
    pub fn pid49(&self) -> Pid49R {
        Pid49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Status"]
    #[inline(always)]
    pub fn pid50(&self) -> Pid50R {
        Pid50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Status"]
    #[inline(always)]
    pub fn pid51(&self) -> Pid51R {
        Pid51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Status"]
    #[inline(always)]
    pub fn pid52(&self) -> Pid52R {
        Pid52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock 53 Status"]
    #[inline(always)]
    pub fn pid53(&self) -> Pid53R {
        Pid53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Status"]
    #[inline(always)]
    pub fn pid56(&self) -> Pid56R {
        Pid56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Status"]
    #[inline(always)]
    pub fn pid57(&self) -> Pid57R {
        Pid57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Status"]
    #[inline(always)]
    pub fn pid58(&self) -> Pid58R {
        Pid58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Status"]
    #[inline(always)]
    pub fn pid59(&self) -> Pid59R {
        Pid59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Status"]
    #[inline(always)]
    pub fn pid60(&self) -> Pid60R {
        Pid60R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcsr1Spec;
impl crate::RegisterSpec for Pcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcsr1::R`](R) reader structure"]
impl crate::Readable for Pcsr1Spec {}
#[doc = "`reset()` method sets PCSR1 to value 0"]
impl crate::Resettable for Pcsr1Spec {}
