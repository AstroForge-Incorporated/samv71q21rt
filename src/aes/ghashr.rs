#[doc = "Register `GHASHR[%s]` reader"]
pub type R = crate::R<GhashrSpec>;
#[doc = "Register `GHASHR[%s]` writer"]
pub type W = crate::W<GhashrSpec>;
#[doc = "Field `GHASH` reader - Intermediate GCM Hash Word x"]
pub type GhashR = crate::FieldReader<u32>;
#[doc = "Field `GHASH` writer - Intermediate GCM Hash Word x"]
pub type GhashW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&self) -> GhashR {
        GhashR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&mut self) -> GhashW<GhashrSpec> {
        GhashW::new(self, 0)
    }
}
#[doc = "GCM Intermediate Hash Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashrSpec;
impl crate::RegisterSpec for GhashrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashr::R`](R) reader structure"]
impl crate::Readable for GhashrSpec {}
#[doc = "`write(|w| ..)` method takes [`ghashr::W`](W) writer structure"]
impl crate::Writable for GhashrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GHASHR[%s] to value 0"]
impl crate::Resettable for GhashrSpec {}
