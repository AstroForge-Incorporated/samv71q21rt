#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `FIFOMODE` reader - HSMCI Internal FIFO control mode"]
pub type FifomodeR = crate::BitReader;
#[doc = "Field `FIFOMODE` writer - HSMCI Internal FIFO control mode"]
pub type FifomodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRCTRL` reader - Flow Error flag reset control mode"]
pub type FerrctrlR = crate::BitReader;
#[doc = "Field `FERRCTRL` writer - Flow Error flag reset control mode"]
pub type FerrctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSMODE` reader - High Speed Mode"]
pub type HsmodeR = crate::BitReader;
#[doc = "Field `HSMODE` writer - High Speed Mode"]
pub type HsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSYNC` reader - Synchronize on the last block"]
pub type LsyncR = crate::BitReader;
#[doc = "Field `LSYNC` writer - Synchronize on the last block"]
pub type LsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&self) -> FifomodeR {
        FifomodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&self) -> FerrctrlR {
        FerrctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HsmodeR {
        HsmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&self) -> LsyncR {
        LsyncR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&mut self) -> FifomodeW<CfgSpec> {
        FifomodeW::new(self, 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&mut self) -> FerrctrlW<CfgSpec> {
        FerrctrlW::new(self, 4)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&mut self) -> HsmodeW<CfgSpec> {
        HsmodeW::new(self, 8)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&mut self) -> LsyncW<CfgSpec> {
        LsyncW::new(self, 12)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
