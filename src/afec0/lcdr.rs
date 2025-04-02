#[doc = "Register `LCDR` reader"]
pub type R = crate::R<LcdrSpec>;
#[doc = "Field `LDATA` reader - Last Data Converted"]
pub type LdataR = crate::FieldReader<u16>;
#[doc = "Field `CHNB` reader - Channel Number"]
pub type ChnbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Last Data Converted"]
    #[inline(always)]
    pub fn ldata(&self) -> LdataR {
        LdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - Channel Number"]
    #[inline(always)]
    pub fn chnb(&self) -> ChnbR {
        ChnbR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "AFEC Last Converted Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdrSpec;
impl crate::RegisterSpec for LcdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdr::R`](R) reader structure"]
impl crate::Readable for LcdrSpec {}
#[doc = "`reset()` method sets LCDR to value 0"]
impl crate::Resettable for LcdrSpec {}
