#[doc = "Register `HCER[%s]` reader"]
pub type R = crate::R<HcerSpec>;
#[doc = "Field `CERR` reader - Bitwise Channel Error Bit \\[31:0\\]"]
pub type CerrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Error Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(self.bits)
    }
}
#[doc = "HBI Channel Error 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcerSpec;
impl crate::RegisterSpec for HcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcer::R`](R) reader structure"]
impl crate::Readable for HcerSpec {}
#[doc = "`reset()` method sets HCER[%s] to value 0"]
impl crate::Resettable for HcerSpec {}
