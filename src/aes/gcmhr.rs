#[doc = "Register `GCMHR[%s]` reader"]
pub type R = crate::R<GcmhrSpec>;
#[doc = "Register `GCMHR[%s]` writer"]
pub type W = crate::W<GcmhrSpec>;
#[doc = "Field `H` reader - GCM H Word x"]
pub type HR = crate::FieldReader<u32>;
#[doc = "Field `H` writer - GCM H Word x"]
pub type HW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GCM H Word x"]
    #[inline(always)]
    pub fn h(&self) -> HR {
        HR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GCM H Word x"]
    #[inline(always)]
    pub fn h(&mut self) -> HW<GcmhrSpec> {
        HW::new(self, 0)
    }
}
#[doc = "GCM H Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmhrSpec;
impl crate::RegisterSpec for GcmhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmhr::R`](R) reader structure"]
impl crate::Readable for GcmhrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmhr::W`](W) writer structure"]
impl crate::Writable for GcmhrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMHR[%s] to value 0"]
impl crate::Resettable for GcmhrSpec {}
