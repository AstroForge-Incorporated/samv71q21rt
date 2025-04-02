#[doc = "Register `RXLPI` reader"]
pub type R = crate::R<RxlpiSpec>;
#[doc = "Field `COUNT` reader - Count of RX LPI transitions (cleared on read)"]
pub type CountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received LPI Transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlpiSpec;
impl crate::RegisterSpec for RxlpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpi::R`](R) reader structure"]
impl crate::Readable for RxlpiSpec {}
#[doc = "`reset()` method sets RXLPI to value 0"]
impl crate::Resettable for RxlpiSpec {}
