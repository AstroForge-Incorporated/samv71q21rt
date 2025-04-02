#[doc = "Register `CCFG_SYSIO` reader"]
pub type R = crate::R<CcfgSysioSpec>;
#[doc = "Register `CCFG_SYSIO` writer"]
pub type W = crate::W<CcfgSysioSpec>;
#[doc = "Field `SYSIO4` reader - PB4 or TDI Assignment"]
pub type Sysio4R = crate::BitReader;
#[doc = "Field `SYSIO4` writer - PB4 or TDI Assignment"]
pub type Sysio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO5` reader - PB5 or TDO/TRACESWO Assignment"]
pub type Sysio5R = crate::BitReader;
#[doc = "Field `SYSIO5` writer - PB5 or TDO/TRACESWO Assignment"]
pub type Sysio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO6` reader - PB6 or TMS/SWDIO Assignment"]
pub type Sysio6R = crate::BitReader;
#[doc = "Field `SYSIO6` writer - PB6 or TMS/SWDIO Assignment"]
pub type Sysio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO7` reader - PB7 or TCK/SWCLK Assignment"]
pub type Sysio7R = crate::BitReader;
#[doc = "Field `SYSIO7` writer - PB7 or TCK/SWCLK Assignment"]
pub type Sysio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO12` reader - PB12 or ERASE Assignment"]
pub type Sysio12R = crate::BitReader;
#[doc = "Field `SYSIO12` writer - PB12 or ERASE Assignment"]
pub type Sysio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1DMABA` reader - CAN1 DMA Base Address"]
pub type Can1dmabaR = crate::FieldReader<u16>;
#[doc = "Field `CAN1DMABA` writer - CAN1 DMA Base Address"]
pub type Can1dmabaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&self) -> Sysio4R {
        Sysio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&self) -> Sysio5R {
        Sysio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&self) -> Sysio6R {
        Sysio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&self) -> Sysio7R {
        Sysio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> Sysio12R {
        Sysio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - CAN1 DMA Base Address"]
    #[inline(always)]
    pub fn can1dmaba(&self) -> Can1dmabaR {
        Can1dmabaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&mut self) -> Sysio4W<CcfgSysioSpec> {
        Sysio4W::new(self, 4)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&mut self) -> Sysio5W<CcfgSysioSpec> {
        Sysio5W::new(self, 5)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&mut self) -> Sysio6W<CcfgSysioSpec> {
        Sysio6W::new(self, 6)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&mut self) -> Sysio7W<CcfgSysioSpec> {
        Sysio7W::new(self, 7)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&mut self) -> Sysio12W<CcfgSysioSpec> {
        Sysio12W::new(self, 12)
    }
    #[doc = "Bits 16:31 - CAN1 DMA Base Address"]
    #[inline(always)]
    pub fn can1dmaba(&mut self) -> Can1dmabaW<CcfgSysioSpec> {
        Can1dmabaW::new(self, 16)
    }
}
#[doc = "System I/O and CAN1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_sysio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_sysio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgSysioSpec;
impl crate::RegisterSpec for CcfgSysioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_sysio::R`](R) reader structure"]
impl crate::Readable for CcfgSysioSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_sysio::W`](W) writer structure"]
impl crate::Writable for CcfgSysioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0"]
impl crate::Resettable for CcfgSysioSpec {}
