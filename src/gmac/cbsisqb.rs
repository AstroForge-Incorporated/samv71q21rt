#[doc = "Register `CBSISQB` reader"]
pub type R = crate::R<CbsisqbSpec>;
#[doc = "Register `CBSISQB` writer"]
pub type W = crate::W<CbsisqbSpec>;
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
    pub fn is(&mut self) -> IsW<CbsisqbSpec> {
        IsW::new(self, 0)
    }
}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B\n\nYou can [`read`](crate::Reg::read) this register and get [`cbsisqb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsisqb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbsisqbSpec;
impl crate::RegisterSpec for CbsisqbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbsisqb::R`](R) reader structure"]
impl crate::Readable for CbsisqbSpec {}
#[doc = "`write(|w| ..)` method takes [`cbsisqb::W`](W) writer structure"]
impl crate::Writable for CbsisqbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CBSISQB to value 0"]
impl crate::Resettable for CbsisqbSpec {}
