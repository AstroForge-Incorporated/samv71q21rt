#[doc = "Register `US_LONL2HDR` reader"]
pub type R = crate::R<UsLonl2hdrSpec>;
#[doc = "Register `US_LONL2HDR` writer"]
pub type W = crate::W<UsLonl2hdrSpec>;
#[doc = "Field `BLI` reader - LON Backlog Increment"]
pub type BliR = crate::FieldReader;
#[doc = "Field `BLI` writer - LON Backlog Increment"]
pub type BliW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALTP` reader - LON Alternate Path Bit"]
pub type AltpR = crate::BitReader;
#[doc = "Field `ALTP` writer - LON Alternate Path Bit"]
pub type AltpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB` reader - LON Priority Bit"]
pub type PbR = crate::BitReader;
#[doc = "Field `PB` writer - LON Priority Bit"]
pub type PbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&self) -> BliR {
        BliR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&self) -> AltpR {
        AltpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&self) -> PbR {
        PbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&mut self) -> BliW<UsLonl2hdrSpec> {
        BliW::new(self, 0)
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&mut self) -> AltpW<UsLonl2hdrSpec> {
        AltpW::new(self, 6)
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&mut self) -> PbW<UsLonl2hdrSpec> {
        PbW::new(self, 7)
    }
}
#[doc = "LON L2HDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonl2hdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonl2hdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLonl2hdrSpec;
impl crate::RegisterSpec for UsLonl2hdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonl2hdr::R`](R) reader structure"]
impl crate::Readable for UsLonl2hdrSpec {}
#[doc = "`write(|w| ..)` method takes [`us_lonl2hdr::W`](W) writer structure"]
impl crate::Writable for UsLonl2hdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_LONL2HDR to value 0"]
impl crate::Resettable for UsLonl2hdrSpec {}
