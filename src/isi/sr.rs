#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ENABLE` reader - Module Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `DIS_DONE` reader - Module Disable Request has Terminated (cleared on read)"]
pub type DisDoneR = crate::BitReader;
#[doc = "Field `SRST` reader - Module Software Reset Request has Terminated (cleared on read)"]
pub type SrstR = crate::BitReader;
#[doc = "Field `CDC_PND` reader - Pending Codec Request"]
pub type CdcPndR = crate::BitReader;
#[doc = "Field `VSYNC` reader - Vertical Synchronization (cleared on read)"]
pub type VsyncR = crate::BitReader;
#[doc = "Field `PXFR_DONE` reader - Preview DMA Transfer has Terminated (cleared on read)"]
pub type PxfrDoneR = crate::BitReader;
#[doc = "Field `CXFR_DONE` reader - Codec DMA Transfer has Terminated (cleared on read)"]
pub type CxfrDoneR = crate::BitReader;
#[doc = "Field `SIP` reader - Synchronization in Progress"]
pub type SipR = crate::BitReader;
#[doc = "Field `P_OVR` reader - Preview Datapath Overflow (cleared on read)"]
pub type POvrR = crate::BitReader;
#[doc = "Field `C_OVR` reader - Codec Datapath Overflow (cleared on read)"]
pub type COvrR = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - CRC Synchronization Error (cleared on read)"]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `FR_OVR` reader - Frame Rate Overrun (cleared on read)"]
pub type FrOvrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn dis_done(&self) -> DisDoneR {
        DisDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Module Software Reset Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending Codec Request"]
    #[inline(always)]
    pub fn cdc_pnd(&self) -> CdcPndR {
        CdcPndR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization (cleared on read)"]
    #[inline(always)]
    pub fn vsync(&self) -> VsyncR {
        VsyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PxfrDoneR {
        PxfrDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CxfrDoneR {
        CxfrDoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Synchronization in Progress"]
    #[inline(always)]
    pub fn sip(&self) -> SipR {
        SipR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn p_ovr(&self) -> POvrR {
        POvrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn c_ovr(&self) -> COvrR {
        COvrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error (cleared on read)"]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun (cleared on read)"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FrOvrR {
        FrOvrR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "ISI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
