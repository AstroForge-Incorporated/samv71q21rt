#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `DIS_DONE` writer - Disable Done Interrupt Enable"]
pub type DisDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST` writer - Software Reset Interrupt Enable"]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC` writer - Vertical Synchronization Interrupt Enable"]
pub type VsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXFR_DONE` writer - Preview DMA Transfer Done Interrupt Enable"]
pub type PxfrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CXFR_DONE` writer - Codec DMA Transfer Done Interrupt Enable"]
pub type CxfrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_OVR` writer - Preview Datapath Overflow Interrupt Enable"]
pub type POvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_OVR` writer - Codec Datapath Overflow Interrupt Enable"]
pub type COvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERR` writer - Embedded Synchronization CRC Error Interrupt Enable"]
pub type CrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FR_OVR` writer - Frame Rate Overflow Interrupt Enable"]
pub type FrOvrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Disable Done Interrupt Enable"]
    #[inline(always)]
    pub fn dis_done(&mut self) -> DisDoneW<IerSpec> {
        DisDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset Interrupt Enable"]
    #[inline(always)]
    pub fn srst(&mut self) -> SrstW<IerSpec> {
        SrstW::new(self, 2)
    }
    #[doc = "Bit 10 - Vertical Synchronization Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VsyncW<IerSpec> {
        VsyncW::new(self, 10)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn pxfr_done(&mut self) -> PxfrDoneW<IerSpec> {
        PxfrDoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn cxfr_done(&mut self) -> CxfrDoneW<IerSpec> {
        CxfrDoneW::new(self, 17)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn p_ovr(&mut self) -> POvrW<IerSpec> {
        POvrW::new(self, 24)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn c_ovr(&mut self) -> COvrW<IerSpec> {
        COvrW::new(self, 25)
    }
    #[doc = "Bit 26 - Embedded Synchronization CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crc_err(&mut self) -> CrcErrW<IerSpec> {
        CrcErrW::new(self, 26)
    }
    #[doc = "Bit 27 - Frame Rate Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fr_ovr(&mut self) -> FrOvrW<IerSpec> {
        FrOvrW::new(self, 27)
    }
}
#[doc = "ISI Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
