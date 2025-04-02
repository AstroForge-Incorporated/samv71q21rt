#[doc = "Register `US_LONDL` reader"]
pub type R = crate::R<UsLondlSpec>;
#[doc = "Register `US_LONDL` writer"]
pub type W = crate::W<UsLondlSpec>;
#[doc = "Field `LONDL` reader - LON Data Length"]
pub type LondlR = crate::FieldReader;
#[doc = "Field `LONDL` writer - LON Data Length"]
pub type LondlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&self) -> LondlR {
        LondlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&mut self) -> LondlW<UsLondlSpec> {
        LondlW::new(self, 0)
    }
}
#[doc = "LON Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_londl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_londl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLondlSpec;
impl crate::RegisterSpec for UsLondlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_londl::R`](R) reader structure"]
impl crate::Readable for UsLondlSpec {}
#[doc = "`write(|w| ..)` method takes [`us_londl::W`](W) writer structure"]
impl crate::Writable for UsLondlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_LONDL to value 0"]
impl crate::Resettable for UsLondlSpec {}
