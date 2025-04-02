#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Trigger Source for Input A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcaselect {
    #[doc = "0: The trigger/capture input A is driven by external pin TIOAx"]
    ExternalTioax = 0,
    #[doc = "1: The trigger/capture input A is driven internally by PWMx"]
    Pwmx = 1,
}
impl From<Trigsrcaselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrcaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrcaselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrcaselect {}
#[doc = "Field `TRIGSRCA` reader - Trigger Source for Input A"]
pub type TrigsrcaR = crate::FieldReader<Trigsrcaselect>;
impl TrigsrcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsrcaselect> {
        match self.bits {
            0 => Some(Trigsrcaselect::ExternalTioax),
            1 => Some(Trigsrcaselect::Pwmx),
            _ => None,
        }
    }
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn is_external_tioax(&self) -> bool {
        *self == Trigsrcaselect::ExternalTioax
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == Trigsrcaselect::Pwmx
    }
}
#[doc = "Field `TRIGSRCA` writer - Trigger Source for Input A"]
pub type TrigsrcaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigsrcaselect>;
impl<'a, REG> TrigsrcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn external_tioax(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcaselect::ExternalTioax)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcaselect::Pwmx)
    }
}
#[doc = "Trigger Source for Input B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrcbselect {
    #[doc = "0: The trigger/capture input B is driven by external pin TIOBx"]
    ExternalTiobx = 0,
    #[doc = "1: For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    Pwmx = 1,
}
impl From<Trigsrcbselect> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrcbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrcbselect {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrcbselect {}
#[doc = "Field `TRIGSRCB` reader - Trigger Source for Input B"]
pub type TrigsrcbR = crate::FieldReader<Trigsrcbselect>;
impl TrigsrcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigsrcbselect> {
        match self.bits {
            0 => Some(Trigsrcbselect::ExternalTiobx),
            1 => Some(Trigsrcbselect::Pwmx),
            _ => None,
        }
    }
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn is_external_tiobx(&self) -> bool {
        *self == Trigsrcbselect::ExternalTiobx
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == Trigsrcbselect::Pwmx
    }
}
#[doc = "Field `TRIGSRCB` writer - Trigger Source for Input B"]
pub type TrigsrcbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigsrcbselect>;
impl<'a, REG> TrigsrcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn external_tiobx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcbselect::ExternalTiobx)
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrcbselect::Pwmx)
    }
}
#[doc = "Field `NODIVCLK` reader - No Divided Clock"]
pub type NodivclkR = crate::BitReader;
#[doc = "Field `NODIVCLK` writer - No Divided Clock"]
pub type NodivclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&self) -> TrigsrcaR {
        TrigsrcaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&self) -> TrigsrcbR {
        TrigsrcbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&self) -> NodivclkR {
        NodivclkR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&mut self) -> TrigsrcaW<EmrSpec> {
        TrigsrcaW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&mut self) -> TrigsrcbW<EmrSpec> {
        TrigsrcbW::new(self, 4)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&mut self) -> NodivclkW<EmrSpec> {
        NodivclkW::new(self, 8)
    }
}
#[doc = "Extended Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {}
