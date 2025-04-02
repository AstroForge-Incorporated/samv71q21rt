#[doc = "Register `PCR` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "Field `PID` reader - Peripheral ID"]
pub type PidR = crate::FieldReader;
#[doc = "Field `PID` writer - Peripheral ID"]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Generic Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gclkcssselect {
    #[doc = "0: Slow clock is selected"]
    SlowClk = 0,
    #[doc = "1: Main clock is selected"]
    MainClk = 1,
    #[doc = "2: PLLACK is selected"]
    PllaClk = 2,
    #[doc = "3: UPLL Clock is selected"]
    UpllClk = 3,
    #[doc = "4: Master Clock is selected"]
    MckClk = 4,
}
impl From<Gclkcssselect> for u8 {
    #[inline(always)]
    fn from(variant: Gclkcssselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gclkcssselect {
    type Ux = u8;
}
impl crate::IsEnum for Gclkcssselect {}
#[doc = "Field `GCLKCSS` reader - Generic Clock Source Selection"]
pub type GclkcssR = crate::FieldReader<Gclkcssselect>;
impl GclkcssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gclkcssselect> {
        match self.bits {
            0 => Some(Gclkcssselect::SlowClk),
            1 => Some(Gclkcssselect::MainClk),
            2 => Some(Gclkcssselect::PllaClk),
            3 => Some(Gclkcssselect::UpllClk),
            4 => Some(Gclkcssselect::MckClk),
            _ => None,
        }
    }
    #[doc = "Slow clock is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == Gclkcssselect::SlowClk
    }
    #[doc = "Main clock is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Gclkcssselect::MainClk
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == Gclkcssselect::PllaClk
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == Gclkcssselect::UpllClk
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn is_mck_clk(&self) -> bool {
        *self == Gclkcssselect::MckClk
    }
}
#[doc = "Field `GCLKCSS` writer - Generic Clock Source Selection"]
pub type GclkcssW<'a, REG> = crate::FieldWriter<'a, REG, 3, Gclkcssselect>;
impl<'a, REG> GclkcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Gclkcssselect::SlowClk)
    }
    #[doc = "Main clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Gclkcssselect::MainClk)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Gclkcssselect::PllaClk)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Gclkcssselect::UpllClk)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Gclkcssselect::MckClk)
    }
}
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCLKDIV` reader - Generic Clock Division Ratio"]
pub type GclkdivR = crate::FieldReader;
#[doc = "Field `GCLKDIV` writer - Generic Clock Division Ratio"]
pub type GclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCLKEN` reader - Generic Clock Enable"]
pub type GclkenR = crate::BitReader;
#[doc = "Field `GCLKEN` writer - Generic Clock Enable"]
pub type GclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&self) -> GclkcssR {
        GclkcssR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&self) -> GclkdivR {
        GclkdivR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&self) -> GclkenR {
        GclkenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<PcrSpec> {
        PidW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&mut self) -> GclkcssW<PcrSpec> {
        GclkcssW::new(self, 8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<PcrSpec> {
        CmdW::new(self, 12)
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&mut self) -> GclkdivW<PcrSpec> {
        GclkdivW::new(self, 20)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<PcrSpec> {
        EnW::new(self, 28)
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&mut self) -> GclkenW<PcrSpec> {
        GclkenW::new(self, 29)
    }
}
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PcrSpec {}
