#[doc = "Register `US_LINBRR` reader"]
pub type R = crate::R<UsLinbrrSpec>;
#[doc = "Field `LINCD` reader - Clock Divider after Synchronization"]
pub type LincdR = crate::FieldReader<u16>;
#[doc = "Field `LINFP` reader - Fractional Part after Synchronization"]
pub type LinfpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline(always)]
    pub fn lincd(&self) -> LincdR {
        LincdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline(always)]
    pub fn linfp(&self) -> LinfpR {
        LinfpR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "LIN Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_linbrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsLinbrrSpec;
impl crate::RegisterSpec for UsLinbrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_linbrr::R`](R) reader structure"]
impl crate::Readable for UsLinbrrSpec {}
#[doc = "`reset()` method sets US_LINBRR to value 0"]
impl crate::Resettable for UsLinbrrSpec {}
