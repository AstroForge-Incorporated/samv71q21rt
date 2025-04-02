#[doc = "Register `CREL` reader"]
pub type R = crate::R<CrelSpec>;
#[doc = "Field `DAY` reader - Timestamp Day"]
pub type DayR = crate::FieldReader;
#[doc = "Field `MON` reader - Timestamp Month"]
pub type MonR = crate::FieldReader;
#[doc = "Field `YEAR` reader - Timestamp Year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SubstepR = crate::FieldReader;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type StepR = crate::FieldReader;
#[doc = "Field `REL` reader - Core Release"]
pub type RelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Timestamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timestamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SubstepR {
        SubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> RelR {
        RelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrelSpec;
impl crate::RegisterSpec for CrelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CrelSpec {}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CrelSpec {}
