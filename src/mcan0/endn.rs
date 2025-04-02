#[doc = "Register `ENDN` reader"]
pub type R = crate::R<EndnSpec>;
#[doc = "Field `ETV` reader - Endianness Test Value"]
pub type EtvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endianness Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(self.bits)
    }
}
#[doc = "Endian Register\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndnSpec;
impl crate::RegisterSpec for EndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endn::R`](R) reader structure"]
impl crate::Readable for EndnSpec {}
#[doc = "`reset()` method sets ENDN to value 0"]
impl crate::Resettable for EndnSpec {}
