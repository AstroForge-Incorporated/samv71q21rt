#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RD` reader - Receive Data"]
pub type RdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {}
