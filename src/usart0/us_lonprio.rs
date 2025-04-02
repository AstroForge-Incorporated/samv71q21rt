#[doc = "Register `US_LONPRIO` reader"]
pub type R = crate::R<UsLonprioSpec>;
#[doc = "Register `US_LONPRIO` writer"]
pub type W = crate::W<UsLonprioSpec>;
#[doc = "Field `PSNB` reader - LON Priority Slot Number"]
pub type PsnbR = crate::FieldReader;
#[doc = "Field `PSNB` writer - LON Priority Slot Number"]
pub type PsnbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NPS` reader - LON Node Priority Slot"]
pub type NpsR = crate::FieldReader;
#[doc = "Field `NPS` writer - LON Node Priority Slot"]
pub type NpsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&self) -> PsnbR {
        PsnbR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&self) -> NpsR {
        NpsR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&mut self) -> PsnbW<UsLonprioSpec> {
        PsnbW::new(self, 0)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&mut self) -> NpsW<UsLonprioSpec> {
        NpsW::new(self, 8)
    }
}
#[doc = "LON Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_lonprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLonprioSpec;
impl crate::RegisterSpec for UsLonprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonprio::R`](R) reader structure"]
impl crate::Readable for UsLonprioSpec {}
#[doc = "`write(|w| ..)` method takes [`us_lonprio::W`](W) writer structure"]
impl crate::Writable for UsLonprioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_LONPRIO to value 0"]
impl crate::Resettable for UsLonprioSpec {}
