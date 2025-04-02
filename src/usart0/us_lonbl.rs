#[doc = "Register `US_LONBL` reader"]
pub type R = crate::R<UsLonblSpec>;
#[doc = "Field `LONBL` reader - LON Node Backlog Value"]
pub type LonblR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - LON Node Backlog Value"]
    #[inline(always)]
    pub fn lonbl(&self) -> LonblR {
        LonblR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "LON Backlog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_lonbl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLonblSpec;
impl crate::RegisterSpec for UsLonblSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonbl::R`](R) reader structure"]
impl crate::Readable for UsLonblSpec {}
#[doc = "`reset()` method sets US_LONBL to value 0"]
impl crate::Resettable for UsLonblSpec {}
