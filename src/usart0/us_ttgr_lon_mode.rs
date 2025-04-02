#[doc = "Register `US_TTGR_LON_MODE` reader"]
pub type R = crate::R<UsTtgrLonModeSpec>;
#[doc = "Register `US_TTGR_LON_MODE` writer"]
pub type W = crate::W<UsTtgrLonModeSpec>;
#[doc = "Field `PCYCLE` reader - LON PCYCLE Length"]
pub type PcycleR = crate::FieldReader<u32>;
#[doc = "Field `PCYCLE` writer - LON PCYCLE Length"]
pub type PcycleW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON PCYCLE Length"]
    #[inline(always)]
    pub fn pcycle(&self) -> PcycleR {
        PcycleR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON PCYCLE Length"]
    #[inline(always)]
    pub fn pcycle(&mut self) -> PcycleW<UsTtgrLonModeSpec> {
        PcycleW::new(self, 0)
    }
}
#[doc = "Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_ttgr_lon_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_ttgr_lon_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsTtgrLonModeSpec;
impl crate::RegisterSpec for UsTtgrLonModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_ttgr_lon_mode::R`](R) reader structure"]
impl crate::Readable for UsTtgrLonModeSpec {}
#[doc = "`write(|w| ..)` method takes [`us_ttgr_lon_mode::W`](W) writer structure"]
impl crate::Writable for UsTtgrLonModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_TTGR_LON_MODE to value 0"]
impl crate::Resettable for UsTtgrLonModeSpec {}
