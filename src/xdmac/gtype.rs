#[doc = "Register `GTYPE` reader"]
pub type R = crate::R<GtypeSpec>;
#[doc = "Field `NB_CH` reader - Number of Channels Minus One"]
pub type NbChR = crate::FieldReader;
#[doc = "Field `FIFO_SZ` reader - Number of Bytes"]
pub type FifoSzR = crate::FieldReader<u16>;
#[doc = "Field `NB_REQ` reader - Number of Peripheral Requests Minus One"]
pub type NbReqR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&self) -> NbChR {
        NbChR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&self) -> FifoSzR {
        FifoSzR::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&self) -> NbReqR {
        NbReqR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Global Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtypeSpec;
impl crate::RegisterSpec for GtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtype::R`](R) reader structure"]
impl crate::Readable for GtypeSpec {}
#[doc = "`reset()` method sets GTYPE to value 0"]
impl crate::Resettable for GtypeSpec {}
