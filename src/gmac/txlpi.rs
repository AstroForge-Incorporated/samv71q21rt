#[doc = "Register `TXLPI` reader"]
pub type R = crate::R<TxlpiSpec>;
#[doc = "Field `COUNT` reader - Count of LPI transitions (cleared on read)"]
pub type CountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count of LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmit LPI Transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlpiSpec;
impl crate::RegisterSpec for TxlpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpi::R`](R) reader structure"]
impl crate::Readable for TxlpiSpec {}
#[doc = "`reset()` method sets TXLPI to value 0"]
impl crate::Resettable for TxlpiSpec {}
