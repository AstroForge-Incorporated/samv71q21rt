#[doc = "Register `SCH` reader"]
pub type R = crate::R<SchSpec>;
#[doc = "Register `SCH` writer"]
pub type W = crate::W<SchSpec>;
#[doc = "Field `SEC` reader - 1588 Timer Second Comparison Value"]
pub type SecR = crate::FieldReader<u16>;
#[doc = "Field `SEC` writer - 1588 Timer Second Comparison Value"]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<SchSpec> {
        SecW::new(self, 0)
    }
}
#[doc = "1588 Timer Second Comparison High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SchSpec;
impl crate::RegisterSpec for SchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sch::R`](R) reader structure"]
impl crate::Readable for SchSpec {}
#[doc = "`write(|w| ..)` method takes [`sch::W`](W) writer structure"]
impl crate::Writable for SchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCH to value 0"]
impl crate::Resettable for SchSpec {}
