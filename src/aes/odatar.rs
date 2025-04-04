#[doc = "Register `ODATAR[%s]` reader"]
pub type R = crate::R<OdatarSpec>;
#[doc = "Field `ODATA` reader - Output Data"]
pub type OdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> OdataR {
        OdataR::new(self.bits)
    }
}
#[doc = "Output Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`odatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdatarSpec;
impl crate::RegisterSpec for OdatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odatar::R`](R) reader structure"]
impl crate::Readable for OdatarSpec {}
#[doc = "`reset()` method sets ODATAR[%s] to value 0"]
impl crate::Resettable for OdatarSpec {}
