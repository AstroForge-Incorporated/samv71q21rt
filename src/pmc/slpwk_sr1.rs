#[doc = "Register `SLPWK_SR1` reader"]
pub type R = crate::R<SlpwkSr1Spec>;
#[doc = "Field `PID32` reader - Peripheral 32 SleepWalking Status"]
pub type Pid32R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral 33 SleepWalking Status"]
pub type Pid33R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral 34 SleepWalking Status"]
pub type Pid34R = crate::BitReader;
#[doc = "Field `PID35` reader - Peripheral 35 SleepWalking Status"]
pub type Pid35R = crate::BitReader;
#[doc = "Field `PID37` reader - Peripheral 37 SleepWalking Status"]
pub type Pid37R = crate::BitReader;
#[doc = "Field `PID39` reader - Peripheral 39 SleepWalking Status"]
pub type Pid39R = crate::BitReader;
#[doc = "Field `PID40` reader - Peripheral 40 SleepWalking Status"]
pub type Pid40R = crate::BitReader;
#[doc = "Field `PID41` reader - Peripheral 41 SleepWalking Status"]
pub type Pid41R = crate::BitReader;
#[doc = "Field `PID42` reader - Peripheral 42 SleepWalking Status"]
pub type Pid42R = crate::BitReader;
#[doc = "Field `PID43` reader - Peripheral 43 SleepWalking Status"]
pub type Pid43R = crate::BitReader;
#[doc = "Field `PID44` reader - Peripheral 44 SleepWalking Status"]
pub type Pid44R = crate::BitReader;
#[doc = "Field `PID45` reader - Peripheral 45 SleepWalking Status"]
pub type Pid45R = crate::BitReader;
#[doc = "Field `PID46` reader - Peripheral 46 SleepWalking Status"]
pub type Pid46R = crate::BitReader;
#[doc = "Field `PID47` reader - Peripheral 47 SleepWalking Status"]
pub type Pid47R = crate::BitReader;
#[doc = "Field `PID48` reader - Peripheral 48 SleepWalking Status"]
pub type Pid48R = crate::BitReader;
#[doc = "Field `PID49` reader - Peripheral 49 SleepWalking Status"]
pub type Pid49R = crate::BitReader;
#[doc = "Field `PID50` reader - Peripheral 50 SleepWalking Status"]
pub type Pid50R = crate::BitReader;
#[doc = "Field `PID51` reader - Peripheral 51 SleepWalking Status"]
pub type Pid51R = crate::BitReader;
#[doc = "Field `PID52` reader - Peripheral 52 SleepWalking Status"]
pub type Pid52R = crate::BitReader;
#[doc = "Field `PID53` reader - Peripheral 53 SleepWalking Status"]
pub type Pid53R = crate::BitReader;
#[doc = "Field `PID56` reader - Peripheral 56 SleepWalking Status"]
pub type Pid56R = crate::BitReader;
#[doc = "Field `PID57` reader - Peripheral 57 SleepWalking Status"]
pub type Pid57R = crate::BitReader;
#[doc = "Field `PID58` reader - Peripheral 58 SleepWalking Status"]
pub type Pid58R = crate::BitReader;
#[doc = "Field `PID59` reader - Peripheral 59 SleepWalking Status"]
pub type Pid59R = crate::BitReader;
#[doc = "Field `PID60` reader - Peripheral 60 SleepWalking Status"]
pub type Pid60R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral 32 SleepWalking Status"]
    #[inline(always)]
    pub fn pid32(&self) -> Pid32R {
        Pid32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral 33 SleepWalking Status"]
    #[inline(always)]
    pub fn pid33(&self) -> Pid33R {
        Pid33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral 34 SleepWalking Status"]
    #[inline(always)]
    pub fn pid34(&self) -> Pid34R {
        Pid34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral 35 SleepWalking Status"]
    #[inline(always)]
    pub fn pid35(&self) -> Pid35R {
        Pid35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral 37 SleepWalking Status"]
    #[inline(always)]
    pub fn pid37(&self) -> Pid37R {
        Pid37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral 39 SleepWalking Status"]
    #[inline(always)]
    pub fn pid39(&self) -> Pid39R {
        Pid39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral 40 SleepWalking Status"]
    #[inline(always)]
    pub fn pid40(&self) -> Pid40R {
        Pid40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral 41 SleepWalking Status"]
    #[inline(always)]
    pub fn pid41(&self) -> Pid41R {
        Pid41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral 42 SleepWalking Status"]
    #[inline(always)]
    pub fn pid42(&self) -> Pid42R {
        Pid42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral 43 SleepWalking Status"]
    #[inline(always)]
    pub fn pid43(&self) -> Pid43R {
        Pid43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral 44 SleepWalking Status"]
    #[inline(always)]
    pub fn pid44(&self) -> Pid44R {
        Pid44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral 45 SleepWalking Status"]
    #[inline(always)]
    pub fn pid45(&self) -> Pid45R {
        Pid45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral 46 SleepWalking Status"]
    #[inline(always)]
    pub fn pid46(&self) -> Pid46R {
        Pid46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral 47 SleepWalking Status"]
    #[inline(always)]
    pub fn pid47(&self) -> Pid47R {
        Pid47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral 48 SleepWalking Status"]
    #[inline(always)]
    pub fn pid48(&self) -> Pid48R {
        Pid48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral 49 SleepWalking Status"]
    #[inline(always)]
    pub fn pid49(&self) -> Pid49R {
        Pid49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral 50 SleepWalking Status"]
    #[inline(always)]
    pub fn pid50(&self) -> Pid50R {
        Pid50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral 51 SleepWalking Status"]
    #[inline(always)]
    pub fn pid51(&self) -> Pid51R {
        Pid51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral 52 SleepWalking Status"]
    #[inline(always)]
    pub fn pid52(&self) -> Pid52R {
        Pid52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral 53 SleepWalking Status"]
    #[inline(always)]
    pub fn pid53(&self) -> Pid53R {
        Pid53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral 56 SleepWalking Status"]
    #[inline(always)]
    pub fn pid56(&self) -> Pid56R {
        Pid56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral 57 SleepWalking Status"]
    #[inline(always)]
    pub fn pid57(&self) -> Pid57R {
        Pid57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral 58 SleepWalking Status"]
    #[inline(always)]
    pub fn pid58(&self) -> Pid58R {
        Pid58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral 59 SleepWalking Status"]
    #[inline(always)]
    pub fn pid59(&self) -> Pid59R {
        Pid59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral 60 SleepWalking Status"]
    #[inline(always)]
    pub fn pid60(&self) -> Pid60R {
        Pid60R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "SleepWalking Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpwkSr1Spec;
impl crate::RegisterSpec for SlpwkSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slpwk_sr1::R`](R) reader structure"]
impl crate::Readable for SlpwkSr1Spec {}
#[doc = "`reset()` method sets SLPWK_SR1 to value 0"]
impl crate::Resettable for SlpwkSr1Spec {}
