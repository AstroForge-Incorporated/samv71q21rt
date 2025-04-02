#[doc = "Register `PCISR` reader"]
pub type R = crate::R<PcisrSpec>;
#[doc = "Field `DRDY` reader - Parallel Capture Mode Data Ready"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Parallel Capture Mode Overrun Error"]
pub type OvreR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Parallel Capture Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcisrSpec;
impl crate::RegisterSpec for PcisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcisr::R`](R) reader structure"]
impl crate::Readable for PcisrSpec {}
#[doc = "`reset()` method sets PCISR to value 0"]
impl crate::Resettable for PcisrSpec {}
