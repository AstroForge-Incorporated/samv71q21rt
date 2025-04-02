#[doc = "Register `PCK[%s]` reader"]
pub type R = crate::R<PckSpec>;
#[doc = "Register `PCK[%s]` writer"]
pub type W = crate::W<PckSpec>;
#[doc = "Programmable Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cssselect {
    #[doc = "0: SLCK is selected"]
    SlowClk = 0,
    #[doc = "1: MAINCK is selected"]
    MainClk = 1,
    #[doc = "2: PLLACK is selected"]
    PllaClk = 2,
    #[doc = "3: UPLLCKDIV is selected"]
    UpllClk = 3,
    #[doc = "4: MCK is selected"]
    Mck = 4,
}
impl From<Cssselect> for u8 {
    #[inline(always)]
    fn from(variant: Cssselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cssselect {
    type Ux = u8;
}
impl crate::IsEnum for Cssselect {}
#[doc = "Field `CSS` reader - Programmable Clock Source Selection"]
pub type CssR = crate::FieldReader<Cssselect>;
impl CssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cssselect> {
        match self.bits {
            0 => Some(Cssselect::SlowClk),
            1 => Some(Cssselect::MainClk),
            2 => Some(Cssselect::PllaClk),
            3 => Some(Cssselect::UpllClk),
            4 => Some(Cssselect::Mck),
            _ => None,
        }
    }
    #[doc = "SLCK is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == Cssselect::SlowClk
    }
    #[doc = "MAINCK is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Cssselect::MainClk
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == Cssselect::PllaClk
    }
    #[doc = "UPLLCKDIV is selected"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == Cssselect::UpllClk
    }
    #[doc = "MCK is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cssselect::Mck
    }
}
#[doc = "Field `CSS` writer - Programmable Clock Source Selection"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cssselect>;
impl<'a, REG> CssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SLCK is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::SlowClk)
    }
    #[doc = "MAINCK is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::MainClk)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::PllaClk)
    }
    #[doc = "UPLLCKDIV is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::UpllClk)
    }
    #[doc = "MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::Mck)
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PresR = crate::FieldReader;
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<PckSpec> {
        CssW::new(self, 0)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PresW<PckSpec> {
        PresW::new(self, 4)
    }
}
#[doc = "Programmable Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PckSpec;
impl crate::RegisterSpec for PckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pck::R`](R) reader structure"]
impl crate::Readable for PckSpec {}
#[doc = "`write(|w| ..)` method takes [`pck::W`](W) writer structure"]
impl crate::Writable for PckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCK[%s] to value 0"]
impl crate::Resettable for PckSpec {}
