#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `DIS_DONE` writer - Disable Done Interrupt Disable"]
pub type DisDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST` writer - Software Reset Interrupt Disable"]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC` writer - Vertical Synchronization Interrupt Disable"]
pub type VsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXFR_DONE` writer - Preview DMA Transfer Done Interrupt Disable"]
pub type PxfrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CXFR_DONE` writer - Codec DMA Transfer Done Interrupt Disable"]
pub type CxfrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_OVR` writer - Preview Datapath Overflow Interrupt Disable"]
pub type POvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_OVR` writer - Codec Datapath Overflow Interrupt Disable"]
pub type COvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERR` writer - Embedded Synchronization CRC Error Interrupt Disable"]
pub type CrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FR_OVR` writer - Frame Rate Overflow Interrupt Disable"]
pub type FrOvrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Disable Done Interrupt Disable"]
    #[inline(always)]
    pub fn dis_done(&mut self) -> DisDoneW<IdrSpec> {
        DisDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset Interrupt Disable"]
    #[inline(always)]
    pub fn srst(&mut self) -> SrstW<IdrSpec> {
        SrstW::new(self, 2)
    }
    #[doc = "Bit 10 - Vertical Synchronization Interrupt Disable"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VsyncW<IdrSpec> {
        VsyncW::new(self, 10)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Done Interrupt Disable"]
    #[inline(always)]
    pub fn pxfr_done(&mut self) -> PxfrDoneW<IdrSpec> {
        PxfrDoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Done Interrupt Disable"]
    #[inline(always)]
    pub fn cxfr_done(&mut self) -> CxfrDoneW<IdrSpec> {
        CxfrDoneW::new(self, 17)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn p_ovr(&mut self) -> POvrW<IdrSpec> {
        POvrW::new(self, 24)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn c_ovr(&mut self) -> COvrW<IdrSpec> {
        COvrW::new(self, 25)
    }
    #[doc = "Bit 26 - Embedded Synchronization CRC Error Interrupt Disable"]
    #[inline(always)]
    pub fn crc_err(&mut self) -> CrcErrW<IdrSpec> {
        CrcErrW::new(self, 26)
    }
    #[doc = "Bit 27 - Frame Rate Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn fr_ovr(&mut self) -> FrOvrW<IdrSpec> {
        FrOvrW::new(self, 27)
    }
}
#[doc = "ISI Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
