#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Voltage Regulator Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vroffselect {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    StopVreg = 1,
}
impl From<Vroffselect> for bool {
    #[inline(always)]
    fn from(variant: Vroffselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VROFF` writer - Voltage Regulator Off"]
pub type VroffW<'a, REG> = crate::BitWriter<'a, REG, Vroffselect>;
impl<'a, REG> VroffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Vroffselect::NoEffect)
    }
    #[doc = "If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut crate::W<REG> {
        self.variant(Vroffselect::StopVreg)
    }
}
#[doc = "Crystal Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalselselect {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    CrystalSel = 1,
}
impl From<Xtalselselect> for bool {
    #[inline(always)]
    fn from(variant: Xtalselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSEL` writer - Crystal Oscillator Select"]
pub type XtalselW<'a, REG> = crate::BitWriter<'a, REG, Xtalselselect>;
impl<'a, REG> XtalselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalselselect::NoEffect)
    }
    #[doc = "If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalselselect::CrystalSel)
    }
}
#[doc = "Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` writer - Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
impl W {
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    pub fn vroff(&mut self) -> VroffW<CrSpec> {
        VroffW::new(self, 2)
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    pub fn xtalsel(&mut self) -> XtalselW<CrSpec> {
        XtalselW::new(self, 3)
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
