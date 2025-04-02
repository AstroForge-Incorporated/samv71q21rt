#[doc = "Register `RXLPITIME` reader"]
pub type R = crate::R<RxlpitimeSpec>;
#[doc = "Field `LPITIME` reader - Time in LPI (cleared on read)"]
pub type LpitimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI (cleared on read)"]
    #[inline(always)]
    pub fn lpitime(&self) -> LpitimeR {
        LpitimeR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Received LPI Time\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpitime::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlpitimeSpec;
impl crate::RegisterSpec for RxlpitimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpitime::R`](R) reader structure"]
impl crate::Readable for RxlpitimeSpec {}
#[doc = "`reset()` method sets RXLPITIME to value 0"]
impl crate::Resettable for RxlpitimeSpec {}
