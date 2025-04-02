#[doc = "Register `CMPV` reader"]
pub type R = crate::R<CmpvSpec>;
#[doc = "Register `CMPV` writer"]
pub type W = crate::W<CmpvSpec>;
#[doc = "Field `CV` reader - Comparison x Value"]
pub type CvR = crate::FieldReader<u32>;
#[doc = "Field `CV` writer - Comparison x Value"]
pub type CvW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Comparison x Value Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cvmselect {
    #[doc = "0: Compare when counter is incrementing"]
    CompareAtIncrement = 0,
    #[doc = "1: Compare when counter is decrementing"]
    CompareAtDecrement = 1,
}
impl From<Cvmselect> for bool {
    #[inline(always)]
    fn from(variant: Cvmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub type CvmR = crate::BitReader<Cvmselect>;
impl CvmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cvmselect {
        match self.bits {
            false => Cvmselect::CompareAtIncrement,
            true => Cvmselect::CompareAtDecrement,
        }
    }
    #[doc = "Compare when counter is incrementing"]
    #[inline(always)]
    pub fn is_compare_at_increment(&self) -> bool {
        *self == Cvmselect::CompareAtIncrement
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline(always)]
    pub fn is_compare_at_decrement(&self) -> bool {
        *self == Cvmselect::CompareAtDecrement
    }
}
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub type CvmW<'a, REG> = crate::BitWriter<'a, REG, Cvmselect>;
impl<'a, REG> CvmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare when counter is incrementing"]
    #[inline(always)]
    pub fn compare_at_increment(self) -> &'a mut crate::W<REG> {
        self.variant(Cvmselect::CompareAtIncrement)
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline(always)]
    pub fn compare_at_decrement(self) -> &'a mut crate::W<REG> {
        self.variant(Cvmselect::CompareAtDecrement)
    }
}
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CvmR {
        CvmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&mut self) -> CvW<CmpvSpec> {
        CvW::new(self, 0)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&mut self) -> CvmW<CmpvSpec> {
        CvmW::new(self, 24)
    }
}
#[doc = "PWM Comparison 0 Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpvSpec;
impl crate::RegisterSpec for CmpvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpv::R`](R) reader structure"]
impl crate::Readable for CmpvSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpv::W`](W) writer structure"]
impl crate::Writable for CmpvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPV to value 0"]
impl crate::Resettable for CmpvSpec {}
