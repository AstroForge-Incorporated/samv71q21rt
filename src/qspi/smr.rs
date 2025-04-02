#[doc = "Register `SMR` reader"]
pub type R = crate::R<SmrSpec>;
#[doc = "Register `SMR` writer"]
pub type W = crate::W<SmrSpec>;
#[doc = "Scrambling/Unscrambling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screnselect {
    #[doc = "0: The scrambling/unscrambling is disabled."]
    Disabled = 0,
    #[doc = "1: The scrambling/unscrambling is enabled."]
    Enabled = 1,
}
impl From<Screnselect> for bool {
    #[inline(always)]
    fn from(variant: Screnselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCREN` reader - Scrambling/Unscrambling Enable"]
pub type ScrenR = crate::BitReader<Screnselect>;
impl ScrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Screnselect {
        match self.bits {
            false => Screnselect::Disabled,
            true => Screnselect::Enabled,
        }
    }
    #[doc = "The scrambling/unscrambling is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Screnselect::Disabled
    }
    #[doc = "The scrambling/unscrambling is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Screnselect::Enabled
    }
}
#[doc = "Field `SCREN` writer - Scrambling/Unscrambling Enable"]
pub type ScrenW<'a, REG> = crate::BitWriter<'a, REG, Screnselect>;
impl<'a, REG> ScrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The scrambling/unscrambling is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Screnselect::Disabled)
    }
    #[doc = "The scrambling/unscrambling is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Screnselect::Enabled)
    }
}
#[doc = "Field `RVDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub type RvdisR = crate::BitReader;
#[doc = "Field `RVDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub type RvdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn scren(&self) -> ScrenR {
        ScrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&self) -> RvdisR {
        RvdisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn scren(&mut self) -> ScrenW<SmrSpec> {
        ScrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&mut self) -> RvdisW<SmrSpec> {
        RvdisW::new(self, 1)
    }
}
#[doc = "Scrambling Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmrSpec;
impl crate::RegisterSpec for SmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smr::R`](R) reader structure"]
impl crate::Readable for SmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smr::W`](W) writer structure"]
impl crate::Writable for SmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SmrSpec {}
