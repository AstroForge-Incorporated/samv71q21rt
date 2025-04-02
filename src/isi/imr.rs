#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `DIS_DONE` reader - Module Disable Operation Completed"]
pub type DisDoneR = crate::BitReader;
#[doc = "Field `SRST` reader - Software Reset Completed"]
pub type SrstR = crate::BitReader;
#[doc = "Field `VSYNC` reader - Vertical Synchronization"]
pub type VsyncR = crate::BitReader;
#[doc = "Field `PXFR_DONE` reader - Preview DMA Transfer Completed"]
pub type PxfrDoneR = crate::BitReader;
#[doc = "Field `CXFR_DONE` reader - Codec DMA Transfer Completed"]
pub type CxfrDoneR = crate::BitReader;
#[doc = "Field `P_OVR` reader - Preview FIFO Overflow"]
pub type POvrR = crate::BitReader;
#[doc = "Field `C_OVR` reader - Codec FIFO Overflow"]
pub type COvrR = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - CRC Synchronization Error"]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `FR_OVR` reader - Frame Rate Overrun"]
pub type FrOvrR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Module Disable Operation Completed"]
    #[inline(always)]
    pub fn dis_done(&self) -> DisDoneR {
        DisDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset Completed"]
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        VsyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Completed"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PxfrDoneR {
        PxfrDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Completed"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CxfrDoneR {
        CxfrDoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Preview FIFO Overflow"]
    #[inline(always)]
    pub fn p_ovr(&self) -> POvrR {
        POvrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Codec FIFO Overflow"]
    #[inline(always)]
    pub fn c_ovr(&self) -> COvrR {
        COvrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error"]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FrOvrR {
        FrOvrR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "ISI Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
