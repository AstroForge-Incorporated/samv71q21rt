#[doc = "Register `TXLPITIME` reader"]
pub type R = crate::R<TxlpitimeSpec>;
#[doc = "Field `LPITIME` reader - Time in LPI (cleared on read)"]
pub type LpitimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI (cleared on read)"]
    #[inline(always)]
    pub fn lpitime(&self) -> LpitimeR {
        LpitimeR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Transmit LPI Time\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpitime::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlpitimeSpec;
impl crate::RegisterSpec for TxlpitimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpitime::R`](R) reader structure"]
impl crate::Readable for TxlpitimeSpec {}
#[doc = "`reset()` method sets TXLPITIME to value 0"]
impl crate::Resettable for TxlpitimeSpec {}
