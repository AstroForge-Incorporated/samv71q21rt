#[doc = "Register `US_FIDI_LON_MODE` reader"]
pub type R = crate::R<UsFidiLonModeSpec>;
#[doc = "Register `US_FIDI_LON_MODE` writer"]
pub type W = crate::W<UsFidiLonModeSpec>;
#[doc = "Field `BETA2` reader - LON BETA2 Length"]
pub type Beta2R = crate::FieldReader<u32>;
#[doc = "Field `BETA2` writer - LON BETA2 Length"]
pub type Beta2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    pub fn beta2(&self) -> Beta2R {
        Beta2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    pub fn beta2(&mut self) -> Beta2W<UsFidiLonModeSpec> {
        Beta2W::new(self, 0)
    }
}
#[doc = "FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_fidi_lon_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_fidi_lon_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsFidiLonModeSpec;
impl crate::RegisterSpec for UsFidiLonModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_fidi_lon_mode::R`](R) reader structure"]
impl crate::Readable for UsFidiLonModeSpec {}
#[doc = "`write(|w| ..)` method takes [`us_fidi_lon_mode::W`](W) writer structure"]
impl crate::Writable for UsFidiLonModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_FIDI_LON_MODE to value 0"]
impl crate::Resettable for UsFidiLonModeSpec {}
