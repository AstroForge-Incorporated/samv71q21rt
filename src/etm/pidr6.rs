#[doc = "Register `PIDR6` reader"]
pub type R = crate::R<Pidr6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Peripheral Identification Register #6\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr6Spec;
impl crate::RegisterSpec for Pidr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr6::R`](R) reader structure"]
impl crate::Readable for Pidr6Spec {}
#[doc = "`reset()` method sets PIDR6 to value 0"]
impl crate::Resettable for Pidr6Spec {}
