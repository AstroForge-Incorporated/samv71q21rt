#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `PGA0EN` reader - PGA0 Enable"]
pub type Pga0enR = crate::BitReader;
#[doc = "Field `PGA0EN` writer - PGA0 Enable"]
pub type Pga0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGA1EN` reader - PGA1 Enable"]
pub type Pga1enR = crate::BitReader;
#[doc = "Field `PGA1EN` writer - PGA1 Enable"]
pub type Pga1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBCTL` reader - AFE Bias Current Control"]
pub type IbctlR = crate::FieldReader;
#[doc = "Field `IBCTL` writer - AFE Bias Current Control"]
pub type IbctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 2 - PGA0 Enable"]
    #[inline(always)]
    pub fn pga0en(&self) -> Pga0enR {
        Pga0enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PGA1 Enable"]
    #[inline(always)]
    pub fn pga1en(&self) -> Pga1enR {
        Pga1enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - AFE Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IbctlR {
        IbctlR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PGA0 Enable"]
    #[inline(always)]
    pub fn pga0en(&mut self) -> Pga0enW<AcrSpec> {
        Pga0enW::new(self, 2)
    }
    #[doc = "Bit 3 - PGA1 Enable"]
    #[inline(always)]
    pub fn pga1en(&mut self) -> Pga1enW<AcrSpec> {
        Pga1enW::new(self, 3)
    }
    #[doc = "Bits 8:9 - AFE Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&mut self) -> IbctlW<AcrSpec> {
        IbctlW::new(self, 8)
    }
}
#[doc = "AFEC Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {}
