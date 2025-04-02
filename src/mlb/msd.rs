#[doc = "Register `MSD` reader"]
pub type R = crate::R<MsdSpec>;
#[doc = "Field `SD0` reader - System Data (Byte 0)"]
pub type Sd0R = crate::FieldReader;
#[doc = "Field `SD1` reader - System Data (Byte 1)"]
pub type Sd1R = crate::FieldReader;
#[doc = "Field `SD2` reader - System Data (Byte 2)"]
pub type Sd2R = crate::FieldReader;
#[doc = "Field `SD3` reader - System Data (Byte 3)"]
pub type Sd3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - System Data (Byte 0)"]
    #[inline(always)]
    pub fn sd0(&self) -> Sd0R {
        Sd0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System Data (Byte 1)"]
    #[inline(always)]
    pub fn sd1(&self) -> Sd1R {
        Sd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System Data (Byte 2)"]
    #[inline(always)]
    pub fn sd2(&self) -> Sd2R {
        Sd2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System Data (Byte 3)"]
    #[inline(always)]
    pub fn sd3(&self) -> Sd3R {
        Sd3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "MediaLB System Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsdSpec;
impl crate::RegisterSpec for MsdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msd::R`](R) reader structure"]
impl crate::Readable for MsdSpec {}
#[doc = "`reset()` method sets MSD to value 0"]
impl crate::Resettable for MsdSpec {}
