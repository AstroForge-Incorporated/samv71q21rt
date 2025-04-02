#[doc = "Register `SSPR` reader"]
pub type R = crate::R<SsprSpec>;
#[doc = "Register `SSPR` writer"]
pub type W = crate::W<SsprSpec>;
#[doc = "Field `SPRD` reader - Spread Spectrum Limit Value"]
pub type SprdR = crate::FieldReader<u32>;
#[doc = "Field `SPRD` writer - Spread Spectrum Limit Value"]
pub type SprdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SPRDM` reader - Spread Spectrum Counter Mode"]
pub type SprdmR = crate::BitReader;
#[doc = "Field `SPRDM` writer - Spread Spectrum Counter Mode"]
pub type SprdmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&self) -> SprdR {
        SprdR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&self) -> SprdmR {
        SprdmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&mut self) -> SprdW<SsprSpec> {
        SprdW::new(self, 0)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&mut self) -> SprdmW<SsprSpec> {
        SprdmW::new(self, 24)
    }
}
#[doc = "PWM Spread Spectrum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsprSpec;
impl crate::RegisterSpec for SsprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspr::R`](R) reader structure"]
impl crate::Readable for SsprSpec {}
#[doc = "`write(|w| ..)` method takes [`sspr::W`](W) writer structure"]
impl crate::Writable for SsprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSPR to value 0"]
impl crate::Resettable for SsprSpec {}
