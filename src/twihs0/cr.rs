#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` writer - Send a START Condition"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Send a STOP Condition"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN` writer - TWIHS Master Mode Enabled"]
pub type MsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDIS` writer - TWIHS Master Mode Disabled"]
pub type MsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVEN` writer - TWIHS Slave Mode Enabled"]
pub type SvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVDIS` writer - TWIHS Slave Mode Disabled"]
pub type SvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUICK` writer - SMBus Quick Command"]
pub type QuickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEN` writer - TWIHS High-Speed Mode Enabled"]
pub type HsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDIS` writer - TWIHS High-Speed Mode Disabled"]
pub type HsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBEN` writer - SMBus Mode Enabled"]
pub type SmbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDIS` writer - SMBus Mode Disabled"]
pub type SmbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECDIS` writer - Packet Error Checking Disable"]
pub type PecdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECRQ` writer - PEC Request"]
pub type PecrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` writer - Bus CLEAR Command"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMEN` writer - Alternative Command Mode Enable"]
pub type AcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMDIS` writer - Alternative Command Mode Disable"]
pub type AcmdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRCLR` writer - Transmit Holding Register Clear"]
pub type ThrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKCLR` writer - Lock Clear"]
pub type LockclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub type FifodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Send a START Condition"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Send a STOP Condition"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<CrSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - TWIHS Master Mode Enabled"]
    #[inline(always)]
    pub fn msen(&mut self) -> MsenW<CrSpec> {
        MsenW::new(self, 2)
    }
    #[doc = "Bit 3 - TWIHS Master Mode Disabled"]
    #[inline(always)]
    pub fn msdis(&mut self) -> MsdisW<CrSpec> {
        MsdisW::new(self, 3)
    }
    #[doc = "Bit 4 - TWIHS Slave Mode Enabled"]
    #[inline(always)]
    pub fn sven(&mut self) -> SvenW<CrSpec> {
        SvenW::new(self, 4)
    }
    #[doc = "Bit 5 - TWIHS Slave Mode Disabled"]
    #[inline(always)]
    pub fn svdis(&mut self) -> SvdisW<CrSpec> {
        SvdisW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus Quick Command"]
    #[inline(always)]
    pub fn quick(&mut self) -> QuickW<CrSpec> {
        QuickW::new(self, 6)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CrSpec> {
        SwrstW::new(self, 7)
    }
    #[doc = "Bit 8 - TWIHS High-Speed Mode Enabled"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HsenW<CrSpec> {
        HsenW::new(self, 8)
    }
    #[doc = "Bit 9 - TWIHS High-Speed Mode Disabled"]
    #[inline(always)]
    pub fn hsdis(&mut self) -> HsdisW<CrSpec> {
        HsdisW::new(self, 9)
    }
    #[doc = "Bit 10 - SMBus Mode Enabled"]
    #[inline(always)]
    pub fn smben(&mut self) -> SmbenW<CrSpec> {
        SmbenW::new(self, 10)
    }
    #[doc = "Bit 11 - SMBus Mode Disabled"]
    #[inline(always)]
    pub fn smbdis(&mut self) -> SmbdisW<CrSpec> {
        SmbdisW::new(self, 11)
    }
    #[doc = "Bit 12 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<CrSpec> {
        PecenW::new(self, 12)
    }
    #[doc = "Bit 13 - Packet Error Checking Disable"]
    #[inline(always)]
    pub fn pecdis(&mut self) -> PecdisW<CrSpec> {
        PecdisW::new(self, 13)
    }
    #[doc = "Bit 14 - PEC Request"]
    #[inline(always)]
    pub fn pecrq(&mut self) -> PecrqW<CrSpec> {
        PecrqW::new(self, 14)
    }
    #[doc = "Bit 15 - Bus CLEAR Command"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<CrSpec> {
        ClearW::new(self, 15)
    }
    #[doc = "Bit 16 - Alternative Command Mode Enable"]
    #[inline(always)]
    pub fn acmen(&mut self) -> AcmenW<CrSpec> {
        AcmenW::new(self, 16)
    }
    #[doc = "Bit 17 - Alternative Command Mode Disable"]
    #[inline(always)]
    pub fn acmdis(&mut self) -> AcmdisW<CrSpec> {
        AcmdisW::new(self, 17)
    }
    #[doc = "Bit 24 - Transmit Holding Register Clear"]
    #[inline(always)]
    pub fn thrclr(&mut self) -> ThrclrW<CrSpec> {
        ThrclrW::new(self, 24)
    }
    #[doc = "Bit 26 - Lock Clear"]
    #[inline(always)]
    pub fn lockclr(&mut self) -> LockclrW<CrSpec> {
        LockclrW::new(self, 26)
    }
    #[doc = "Bit 28 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<CrSpec> {
        FifoenW::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO Disable"]
    #[inline(always)]
    pub fn fifodis(&mut self) -> FifodisW<CrSpec> {
        FifodisW::new(self, 29)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
