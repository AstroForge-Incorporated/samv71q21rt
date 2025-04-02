#[doc = "Register `CIS` reader"]
pub type R = crate::R<CisSpec>;
#[doc = "Field `BIS` reader - End of Block Interrupt Status Bit"]
pub type BisR = crate::BitReader;
#[doc = "Field `LIS` reader - End of Linked List Interrupt Status Bit"]
pub type LisR = crate::BitReader;
#[doc = "Field `DIS` reader - End of Disable Interrupt Status Bit"]
pub type DisR = crate::BitReader;
#[doc = "Field `FIS` reader - End of Flush Interrupt Status Bit"]
pub type FisR = crate::BitReader;
#[doc = "Field `RBEIS` reader - Read Bus Error Interrupt Status Bit"]
pub type RbeisR = crate::BitReader;
#[doc = "Field `WBEIS` reader - Write Bus Error Interrupt Status Bit"]
pub type WbeisR = crate::BitReader;
#[doc = "Field `ROIS` reader - Request Overflow Error Interrupt Status Bit"]
pub type RoisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Status Bit"]
    #[inline(always)]
    pub fn bis(&self) -> BisR {
        BisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Status Bit"]
    #[inline(always)]
    pub fn lis(&self) -> LisR {
        LisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Status Bit"]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Status Bit"]
    #[inline(always)]
    pub fn fis(&self) -> FisR {
        FisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rbeis(&self) -> RbeisR {
        RbeisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn wbeis(&self) -> WbeisR {
        WbeisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rois(&self) -> RoisR {
        RoisR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Channel Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CisSpec;
impl crate::RegisterSpec for CisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cis::R`](R) reader structure"]
impl crate::Readable for CisSpec {}
#[doc = "`reset()` method sets CIS to value 0"]
impl crate::Resettable for CisSpec {}
