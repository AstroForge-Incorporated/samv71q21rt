#[doc = "Register `NSC` reader"]
pub type R = crate::R<NscSpec>;
#[doc = "Register `NSC` writer"]
pub type W = crate::W<NscSpec>;
#[doc = "Field `NANOSEC` reader - 1588 Timer Nanosecond Comparison Value"]
pub type NanosecR = crate::FieldReader<u32>;
#[doc = "Field `NANOSEC` writer - 1588 Timer Nanosecond Comparison Value"]
pub type NanosecW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - 1588 Timer Nanosecond Comparison Value"]
    #[inline(always)]
    pub fn nanosec(&self) -> NanosecR {
        NanosecR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - 1588 Timer Nanosecond Comparison Value"]
    #[inline(always)]
    pub fn nanosec(&mut self) -> NanosecW<NscSpec> {
        NanosecW::new(self, 0)
    }
}
#[doc = "1588 Timer Nanosecond Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NscSpec;
impl crate::RegisterSpec for NscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsc::R`](R) reader structure"]
impl crate::Readable for NscSpec {}
#[doc = "`write(|w| ..)` method takes [`nsc::W`](W) writer structure"]
impl crate::Writable for NscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSC to value 0"]
impl crate::Resettable for NscSpec {}
