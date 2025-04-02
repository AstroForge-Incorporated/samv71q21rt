#[doc = "Register `CCFG_PCCR` reader"]
pub type R = crate::R<CcfgPccrSpec>;
#[doc = "Register `CCFG_PCCR` writer"]
pub type W = crate::W<CcfgPccrSpec>;
#[doc = "Field `TC0CC` reader - TC0 Clock Configuration"]
pub type Tc0ccR = crate::BitReader;
#[doc = "Field `TC0CC` writer - TC0 Clock Configuration"]
pub type Tc0ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SC0CC` reader - I2SC0 Clock Configuration"]
pub type I2sc0ccR = crate::BitReader;
#[doc = "Field `I2SC0CC` writer - I2SC0 Clock Configuration"]
pub type I2sc0ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SC1CC` reader - I2SC1 Clock Configuration"]
pub type I2sc1ccR = crate::BitReader;
#[doc = "Field `I2SC1CC` writer - I2SC1 Clock Configuration"]
pub type I2sc1ccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&self) -> Tc0ccR {
        Tc0ccR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&self) -> I2sc0ccR {
        I2sc0ccR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&self) -> I2sc1ccR {
        I2sc1ccR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&mut self) -> Tc0ccW<CcfgPccrSpec> {
        Tc0ccW::new(self, 20)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&mut self) -> I2sc0ccW<CcfgPccrSpec> {
        I2sc0ccW::new(self, 21)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&mut self) -> I2sc1ccW<CcfgPccrSpec> {
        I2sc1ccW::new(self, 22)
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_pccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_pccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgPccrSpec;
impl crate::RegisterSpec for CcfgPccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_pccr::R`](R) reader structure"]
impl crate::Readable for CcfgPccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_pccr::W`](W) writer structure"]
impl crate::Writable for CcfgPccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCFG_PCCR to value 0"]
impl crate::Resettable for CcfgPccrSpec {}
