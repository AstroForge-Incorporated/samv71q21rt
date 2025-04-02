#[doc = "Register `TAGR[%s]` reader"]
pub type R = crate::R<TagrSpec>;
#[doc = "Field `TAG` reader - GCM Authentication Tag x"]
pub type TagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Authentication Tag x"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(self.bits)
    }
}
#[doc = "GCM Authentication Tag Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tagr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TagrSpec;
impl crate::RegisterSpec for TagrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagr::R`](R) reader structure"]
impl crate::Readable for TagrSpec {}
#[doc = "`reset()` method sets TAGR[%s] to value 0"]
impl crate::Resettable for TagrSpec {}
