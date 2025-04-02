#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Current Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iselselect {
    #[doc = "0: Low-power option."]
    Lopw = 0,
    #[doc = "1: High-speed option."]
    Hisp = 1,
}
impl From<Iselselect> for bool {
    #[inline(always)]
    fn from(variant: Iselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - Current Selection"]
pub type IselR = crate::BitReader<Iselselect>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iselselect {
        match self.bits {
            false => Iselselect::Lopw,
            true => Iselselect::Hisp,
        }
    }
    #[doc = "Low-power option."]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        *self == Iselselect::Lopw
    }
    #[doc = "High-speed option."]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        *self == Iselselect::Hisp
    }
}
#[doc = "Field `ISEL` writer - Current Selection"]
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Iselselect>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut crate::W<REG> {
        self.variant(Iselselect::Lopw)
    }
    #[doc = "High-speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut crate::W<REG> {
        self.variant(Iselselect::Hisp)
    }
}
#[doc = "Field `HYST` reader - Hysteresis Selection"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - Hysteresis Selection"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<AcrSpec> {
        IselW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<AcrSpec> {
        HystW::new(self, 1)
    }
}
#[doc = "Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
