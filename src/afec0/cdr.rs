#[doc = "Register `CDR` reader"]
pub type R = crate::R<CdrSpec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "AFEC Channel Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CdrSpec {}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {}
