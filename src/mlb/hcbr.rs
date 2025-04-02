#[doc = "Register `HCBR[%s]` reader"]
pub type R = crate::R<HcbrSpec>;
#[doc = "Field `CHB` reader - Bitwise Channel Busy Bit \\[31:0\\]"]
pub type ChbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Busy Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chb(&self) -> ChbR {
        ChbR::new(self.bits)
    }
}
#[doc = "HBI Channel Busy 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcbrSpec;
impl crate::RegisterSpec for HcbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcbr::R`](R) reader structure"]
impl crate::Readable for HcbrSpec {}
#[doc = "`reset()` method sets HCBR[%s] to value 0"]
impl crate::Resettable for HcbrSpec {}
