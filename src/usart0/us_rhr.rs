#[doc = "Register `US_RHR` reader"]
pub type R = crate::R<UsRhrSpec>;
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RxchrR = crate::FieldReader<u16>;
#[doc = "Field `RXSYNH` reader - Received Sync"]
pub type RxsynhR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RxchrR {
        RxchrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RxsynhR {
        RxsynhR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_rhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsRhrSpec;
impl crate::RegisterSpec for UsRhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_rhr::R`](R) reader structure"]
impl crate::Readable for UsRhrSpec {}
#[doc = "`reset()` method sets US_RHR to value 0"]
impl crate::Resettable for UsRhrSpec {}
