#[doc = "Register `US_NER` reader"]
pub type R = crate::R<UsNerSpec>;
#[doc = "Field `NB_ERRORS` reader - Number of Errors"]
pub type NbErrorsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NbErrorsR {
        NbErrorsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_ner::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsNerSpec;
impl crate::RegisterSpec for UsNerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_ner::R`](R) reader structure"]
impl crate::Readable for UsNerSpec {}
#[doc = "`reset()` method sets US_NER to value 0"]
impl crate::Resettable for UsNerSpec {}
