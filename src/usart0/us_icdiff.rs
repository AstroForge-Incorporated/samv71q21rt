#[doc = "Register `US_ICDIFF` reader"]
pub type R = crate::R<UsIcdiffSpec>;
#[doc = "Register `US_ICDIFF` writer"]
pub type W = crate::W<UsIcdiffSpec>;
#[doc = "Field `ICDIFF` reader - IC Differentiator Number"]
pub type IcdiffR = crate::FieldReader;
#[doc = "Field `ICDIFF` writer - IC Differentiator Number"]
pub type IcdiffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&self) -> IcdiffR {
        IcdiffR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&mut self) -> IcdiffW<UsIcdiffSpec> {
        IcdiffW::new(self, 0)
    }
}
#[doc = "IC DIFF Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_icdiff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_icdiff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsIcdiffSpec;
impl crate::RegisterSpec for UsIcdiffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_icdiff::R`](R) reader structure"]
impl crate::Readable for UsIcdiffSpec {}
#[doc = "`write(|w| ..)` method takes [`us_icdiff::W`](W) writer structure"]
impl crate::Writable for UsIcdiffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_ICDIFF to value 0"]
impl crate::Resettable for UsIcdiffSpec {}
