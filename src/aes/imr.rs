#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `DATRDY` reader - Data Ready Interrupt Mask"]
pub type DatrdyR = crate::BitReader;
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Interrupt Mask"]
pub type UradR = crate::BitReader;
#[doc = "Field `TAGRDY` reader - GCM Tag Ready Interrupt Mask"]
pub type TagrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn datrdy(&self) -> DatrdyR {
        DatrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> UradR {
        UradR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Mask"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TagrdyR {
        TagrdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
