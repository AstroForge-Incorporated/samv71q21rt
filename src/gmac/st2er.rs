#[doc = "Register `ST2ER[%s]` reader"]
pub type R = crate::R<St2erSpec>;
#[doc = "Register `ST2ER[%s]` writer"]
pub type W = crate::W<St2erSpec>;
#[doc = "Field `COMPVAL` reader - Ethertype Compare Value"]
pub type CompvalR = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - Ethertype Compare Value"]
pub type CompvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> CompvalR {
        CompvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> CompvalW<St2erSpec> {
        CompvalW::new(self, 0)
    }
}
#[doc = "Screening Type 2 Ethertype Register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2erSpec;
impl crate::RegisterSpec for St2erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2er::R`](R) reader structure"]
impl crate::Readable for St2erSpec {}
#[doc = "`write(|w| ..)` method takes [`st2er::W`](W) writer structure"]
impl crate::Writable for St2erSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2ER[%s] to value 0"]
impl crate::Resettable for St2erSpec {}
