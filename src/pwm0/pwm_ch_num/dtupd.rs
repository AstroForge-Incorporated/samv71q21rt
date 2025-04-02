#[doc = "Register `DTUPD` writer"]
pub type W = crate::W<DtupdSpec>;
#[doc = "Field `DTHUPD` writer - Dead-Time Value Update for PWMHx Output"]
pub type DthupdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DTLUPD` writer - Dead-Time Value Update for PWMLx Output"]
pub type DtlupdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value Update for PWMHx Output"]
    #[inline(always)]
    pub fn dthupd(&mut self) -> DthupdW<DtupdSpec> {
        DthupdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Dead-Time Value Update for PWMLx Output"]
    #[inline(always)]
    pub fn dtlupd(&mut self) -> DtlupdW<DtupdSpec> {
        DtlupdW::new(self, 16)
    }
}
#[doc = "PWM Channel Dead Time Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtupdSpec;
impl crate::RegisterSpec for DtupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtupd::W`](W) writer structure"]
impl crate::Writable for DtupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTUPD to value 0"]
impl crate::Resettable for DtupdSpec {}
