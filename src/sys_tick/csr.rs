#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Enables the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: counter disabled"]
    Value0 = 0,
    #[doc = "1: counter enabled"]
    Value1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enables the counter"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Value0,
            true => Enableselect::Value1,
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Enableselect::Value0
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Enableselect::Value1
    }
}
#[doc = "Field `ENABLE` writer - Enables the counter"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Value0)
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Value1)
    }
}
#[doc = "Enables SysTick exception request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tickintselect {
    #[doc = "0: counting down to 0 does not assert the SysTick exception request"]
    Value0 = 0,
    #[doc = "1: counting down to 0 asserts the SysTick exception request"]
    Value1 = 1,
}
impl From<Tickintselect> for bool {
    #[inline(always)]
    fn from(variant: Tickintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - Enables SysTick exception request"]
pub type TickintR = crate::BitReader<Tickintselect>;
impl TickintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tickintselect {
        match self.bits {
            false => Tickintselect::Value0,
            true => Tickintselect::Value1,
        }
    }
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Tickintselect::Value0
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Tickintselect::Value1
    }
}
#[doc = "Field `TICKINT` writer - Enables SysTick exception request"]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG, Tickintselect>;
impl<'a, REG> TickintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tickintselect::Value0)
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tickintselect::Value1)
    }
}
#[doc = "Indicates the clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksourceselect {
    #[doc = "0: external clock"]
    Value0 = 0,
    #[doc = "1: processor clock"]
    Value1 = 1,
}
impl From<Clksourceselect> for bool {
    #[inline(always)]
    fn from(variant: Clksourceselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - Indicates the clock source"]
pub type ClksourceR = crate::BitReader<Clksourceselect>;
impl ClksourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksourceselect {
        match self.bits {
            false => Clksourceselect::Value0,
            true => Clksourceselect::Value1,
        }
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Clksourceselect::Value0
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Clksourceselect::Value1
    }
}
#[doc = "Field `CLKSOURCE` writer - Indicates the clock source"]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG, Clksourceselect>;
impl<'a, REG> ClksourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external clock"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksourceselect::Value0)
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksourceselect::Value1)
    }
}
#[doc = "Field `COUNTFLAG` reader - Returns 1 if timer counted to 0 since last time this was read"]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Returns 1 if timer counted to 0 since last time this was read"]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the counter"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables SysTick exception request"]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the clock source"]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read"]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the counter"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables SysTick exception request"]
    #[inline(always)]
    pub fn tickint(&mut self) -> TickintW<CsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the clock source"]
    #[inline(always)]
    pub fn clksource(&mut self) -> ClksourceW<CsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read"]
    #[inline(always)]
    pub fn countflag(&mut self) -> CountflagW<CsrSpec> {
        CountflagW::new(self, 16)
    }
}
#[doc = "Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0x04"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
