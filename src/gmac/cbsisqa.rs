#[doc = "Register `CBSISQA` reader"]
pub type R = crate::R<CbsisqaSpec>;
#[doc = "Register `CBSISQA` writer"]
pub type W = crate::W<CbsisqaSpec>;
#[doc = "Field `IS` reader - IdleSlope"]
pub type IsR = crate::FieldReader<u32>;
#[doc = "Field `IS` writer - IdleSlope"]
pub type IsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&mut self) -> IsW<CbsisqaSpec> {
        IsW::new(self, 0)
    }
}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A\n\nYou can [`read`](crate::Reg::read) this register and get [`cbsisqa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsisqa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbsisqaSpec;
impl crate::RegisterSpec for CbsisqaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbsisqa::R`](R) reader structure"]
impl crate::Readable for CbsisqaSpec {}
#[doc = "`write(|w| ..)` method takes [`cbsisqa::W`](W) writer structure"]
impl crate::Writable for CbsisqaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CBSISQA to value 0"]
impl crate::Resettable for CbsisqaSpec {}
