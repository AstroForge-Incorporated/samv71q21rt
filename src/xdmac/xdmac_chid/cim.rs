#[doc = "Register `CIM` reader"]
pub type R = crate::R<CimSpec>;
#[doc = "Field `BIM` reader - End of Block Interrupt Mask Bit"]
pub type BimR = crate::BitReader;
#[doc = "Field `LIM` reader - End of Linked List Interrupt Mask Bit"]
pub type LimR = crate::BitReader;
#[doc = "Field `DIM` reader - End of Disable Interrupt Mask Bit"]
pub type DimR = crate::BitReader;
#[doc = "Field `FIM` reader - End of Flush Interrupt Mask Bit"]
pub type FimR = crate::BitReader;
#[doc = "Field `RBEIM` reader - Read Bus Error Interrupt Mask Bit"]
pub type RbeimR = crate::BitReader;
#[doc = "Field `WBEIM` reader - Write Bus Error Interrupt Mask Bit"]
pub type WbeimR = crate::BitReader;
#[doc = "Field `ROIM` reader - Request Overflow Error Interrupt Mask Bit"]
pub type RoimR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Mask Bit"]
    #[inline(always)]
    pub fn bim(&self) -> BimR {
        BimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Mask Bit"]
    #[inline(always)]
    pub fn lim(&self) -> LimR {
        LimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Mask Bit"]
    #[inline(always)]
    pub fn dim(&self) -> DimR {
        DimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Mask Bit"]
    #[inline(always)]
    pub fn fim(&self) -> FimR {
        FimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn rbeim(&self) -> RbeimR {
        RbeimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn wbeim(&self) -> WbeimR {
        WbeimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn roim(&self) -> RoimR {
        RoimR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Channel Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CimSpec;
impl crate::RegisterSpec for CimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cim::R`](R) reader structure"]
impl crate::Readable for CimSpec {}
#[doc = "`reset()` method sets CIM to value 0"]
impl crate::Resettable for CimSpec {}
