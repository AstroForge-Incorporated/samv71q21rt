#[doc = "Register `GCFG` reader"]
pub type R = crate::R<GcfgSpec>;
#[doc = "Register `GCFG` writer"]
pub type W = crate::W<GcfgSpec>;
#[doc = "Field `CGDISREG` reader - Configuration Registers Clock Gating Disable"]
pub type CgdisregR = crate::BitReader;
#[doc = "Field `CGDISREG` writer - Configuration Registers Clock Gating Disable"]
pub type CgdisregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGDISPIPE` reader - Pipeline Clock Gating Disable"]
pub type CgdispipeR = crate::BitReader;
#[doc = "Field `CGDISPIPE` writer - Pipeline Clock Gating Disable"]
pub type CgdispipeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGDISFIFO` reader - FIFO Clock Gating Disable"]
pub type CgdisfifoR = crate::BitReader;
#[doc = "Field `CGDISFIFO` writer - FIFO Clock Gating Disable"]
pub type CgdisfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGDISIF` reader - Bus Interface Clock Gating Disable"]
pub type CgdisifR = crate::BitReader;
#[doc = "Field `CGDISIF` writer - Bus Interface Clock Gating Disable"]
pub type CgdisifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BXKBEN` reader - Boundary X Kilobyte Enable"]
pub type BxkbenR = crate::BitReader;
#[doc = "Field `BXKBEN` writer - Boundary X Kilobyte Enable"]
pub type BxkbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&self) -> CgdisregR {
        CgdisregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&self) -> CgdispipeR {
        CgdispipeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&self) -> CgdisfifoR {
        CgdisfifoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&self) -> CgdisifR {
        CgdisifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&self) -> BxkbenR {
        BxkbenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&mut self) -> CgdisregW<GcfgSpec> {
        CgdisregW::new(self, 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&mut self) -> CgdispipeW<GcfgSpec> {
        CgdispipeW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&mut self) -> CgdisfifoW<GcfgSpec> {
        CgdisfifoW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&mut self) -> CgdisifW<GcfgSpec> {
        CgdisifW::new(self, 3)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&mut self) -> BxkbenW<GcfgSpec> {
        BxkbenW::new(self, 8)
    }
}
#[doc = "Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcfgSpec;
impl crate::RegisterSpec for GcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcfg::R`](R) reader structure"]
impl crate::Readable for GcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gcfg::W`](W) writer structure"]
impl crate::Writable for GcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCFG to value 0"]
impl crate::Resettable for GcfgSpec {}
