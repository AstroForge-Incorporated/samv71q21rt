#[doc = "Register `US_RTOR` reader"]
pub type R = crate::R<UsRtorSpec>;
#[doc = "Register `US_RTOR` writer"]
pub type W = crate::W<UsRtorSpec>;
#[doc = "Field `TO` reader - Timeout Value"]
pub type ToR = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Timeout Value"]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<UsRtorSpec> {
        ToW::new(self, 0)
    }
}
#[doc = "Receiver Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsRtorSpec;
impl crate::RegisterSpec for UsRtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_rtor::R`](R) reader structure"]
impl crate::Readable for UsRtorSpec {}
#[doc = "`write(|w| ..)` method takes [`us_rtor::W`](W) writer structure"]
impl crate::Writable for UsRtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_RTOR to value 0"]
impl crate::Resettable for UsRtorSpec {}
