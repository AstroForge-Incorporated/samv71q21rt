#[doc = "Register `CHSR` reader"]
pub type R = crate::R<ChsrSpec>;
#[doc = "Field `CH0` reader - Channel 0 Status"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Channel 1 Status"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `DACRDY0` reader - DAC Ready Flag"]
pub type Dacrdy0R = crate::BitReader;
#[doc = "Field `DACRDY1` reader - DAC Ready Flag"]
pub type Dacrdy1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy0(&self) -> Dacrdy0R {
        Dacrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy1(&self) -> Dacrdy1R {
        Dacrdy1R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsrSpec;
impl crate::RegisterSpec for ChsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for ChsrSpec {}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for ChsrSpec {}
