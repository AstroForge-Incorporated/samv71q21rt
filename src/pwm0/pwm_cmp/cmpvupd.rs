#[doc = "Register `CMPVUPD` writer"]
pub type W = crate::W<CmpvupdSpec>;
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CvupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CvmupdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    pub fn cvupd(&mut self) -> CvupdW<CmpvupdSpec> {
        CvupdW::new(self, 0)
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    pub fn cvmupd(&mut self) -> CvmupdW<CmpvupdSpec> {
        CvmupdW::new(self, 24)
    }
}
#[doc = "PWM Comparison 0 Value Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpvupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpvupdSpec;
impl crate::RegisterSpec for CmpvupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmpvupd::W`](W) writer structure"]
impl crate::Writable for CmpvupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPVUPD to value 0"]
impl crate::Resettable for CmpvupdSpec {}
