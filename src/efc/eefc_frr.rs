#[doc = "Register `EEFC_FRR` reader"]
pub type R = crate::R<EefcFrrSpec>;
#[doc = "Field `FVALUE` reader - Flash Result Value"]
pub type FvalueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Result Value"]
    #[inline(always)]
    pub fn fvalue(&self) -> FvalueR {
        FvalueR::new(self.bits)
    }
}
#[doc = "EEFC Flash Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_frr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EefcFrrSpec;
impl crate::RegisterSpec for EefcFrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefc_frr::R`](R) reader structure"]
impl crate::Readable for EefcFrrSpec {}
#[doc = "`reset()` method sets EEFC_FRR to value 0"]
impl crate::Resettable for EefcFrrSpec {}
