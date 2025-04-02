#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Brownout Detector Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodrstenselect {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    NotEnable = 0,
    #[doc = "1: The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    Enable = 1,
}
impl From<Bodrstenselect> for bool {
    #[inline(always)]
    fn from(variant: Bodrstenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTEN` reader - Brownout Detector Reset Enable"]
pub type BodrstenR = crate::BitReader<Bodrstenselect>;
impl BodrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrstenselect {
        match self.bits {
            false => Bodrstenselect::NotEnable,
            true => Bodrstenselect::Enable,
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Bodrstenselect::NotEnable
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bodrstenselect::Enable
    }
}
#[doc = "Field `BODRSTEN` writer - Brownout Detector Reset Enable"]
pub type BodrstenW<'a, REG> = crate::BitWriter<'a, REG, Bodrstenselect>;
impl<'a, REG> BodrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstenselect::NotEnable)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrstenselect::Enable)
    }
}
#[doc = "Brownout Detector Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boddisselect {
    #[doc = "0: The core brownout detector is enabled."]
    Enable = 0,
    #[doc = "1: The core brownout detector is disabled."]
    Disable = 1,
}
impl From<Boddisselect> for bool {
    #[inline(always)]
    fn from(variant: Boddisselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODDIS` reader - Brownout Detector Disable"]
pub type BoddisR = crate::BitReader<Boddisselect>;
impl BoddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boddisselect {
        match self.bits {
            false => Boddisselect::Enable,
            true => Boddisselect::Disable,
        }
    }
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Boddisselect::Enable
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Boddisselect::Disable
    }
}
#[doc = "Field `BODDIS` writer - Brownout Detector Disable"]
pub type BoddisW<'a, REG> = crate::BitWriter<'a, REG, Boddisselect>;
impl<'a, REG> BoddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Boddisselect::Enable)
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Boddisselect::Disable)
    }
}
#[doc = "Voltage Regulator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onregselect {
    #[doc = "0: Internal voltage regulator is not used (external power supply is used)."]
    OnregUnused = 0,
    #[doc = "1: Internal voltage regulator is used."]
    OnregUsed = 1,
}
impl From<Onregselect> for bool {
    #[inline(always)]
    fn from(variant: Onregselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONREG` reader - Voltage Regulator Enable"]
pub type OnregR = crate::BitReader<Onregselect>;
impl OnregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onregselect {
        match self.bits {
            false => Onregselect::OnregUnused,
            true => Onregselect::OnregUsed,
        }
    }
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    #[inline(always)]
    pub fn is_onreg_unused(&self) -> bool {
        *self == Onregselect::OnregUnused
    }
    #[doc = "Internal voltage regulator is used."]
    #[inline(always)]
    pub fn is_onreg_used(&self) -> bool {
        *self == Onregselect::OnregUsed
    }
}
#[doc = "Field `ONREG` writer - Voltage Regulator Enable"]
pub type OnregW<'a, REG> = crate::BitWriter<'a, REG, Onregselect>;
impl<'a, REG> OnregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    #[inline(always)]
    pub fn onreg_unused(self) -> &'a mut crate::W<REG> {
        self.variant(Onregselect::OnregUnused)
    }
    #[doc = "Internal voltage regulator is used."]
    #[inline(always)]
    pub fn onreg_used(self) -> &'a mut crate::W<REG> {
        self.variant(Onregselect::OnregUsed)
    }
}
#[doc = "Field `BKUPRETON` reader - SRAM On In Backup Mode"]
pub type BkupretonR = crate::BitReader;
#[doc = "Field `BKUPRETON` writer - SRAM On In Backup Mode"]
pub type BkupretonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscbypassselect {
    #[doc = "0: No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    NoEffect = 0,
    #[doc = "1: The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    Bypass = 1,
}
impl From<Oscbypassselect> for bool {
    #[inline(always)]
    fn from(variant: Oscbypassselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub type OscbypassR = crate::BitReader<Oscbypassselect>;
impl OscbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscbypassselect {
        match self.bits {
            false => Oscbypassselect::NoEffect,
            true => Oscbypassselect::Bypass,
        }
    }
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Oscbypassselect::NoEffect
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Oscbypassselect::Bypass
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub type OscbypassW<'a, REG> = crate::BitWriter<'a, REG, Oscbypassselect>;
impl<'a, REG> OscbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypassselect::NoEffect)
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypassselect::Bypass)
    }
}
#[doc = "Password Key\n\nValue on reset: 0"]
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
#[doc = "Field `KEY` reader - Password Key"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            165 => Some(Keyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Keyselect::Passwd
    }
}
#[doc = "Field `KEY` writer - Password Key"]
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
impl R {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BodrstenR {
        BodrstenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BoddisR {
        BoddisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Voltage Regulator Enable"]
    #[inline(always)]
    pub fn onreg(&self) -> OnregR {
        OnregR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM On In Backup Mode"]
    #[inline(always)]
    pub fn bkupreton(&self) -> BkupretonR {
        BkupretonR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OscbypassR {
        OscbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&mut self) -> BodrstenW<MrSpec> {
        BodrstenW::new(self, 12)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&mut self) -> BoddisW<MrSpec> {
        BoddisW::new(self, 13)
    }
    #[doc = "Bit 14 - Voltage Regulator Enable"]
    #[inline(always)]
    pub fn onreg(&mut self) -> OnregW<MrSpec> {
        OnregW::new(self, 14)
    }
    #[doc = "Bit 17 - SRAM On In Backup Mode"]
    #[inline(always)]
    pub fn bkupreton(&mut self) -> BkupretonW<MrSpec> {
        BkupretonW::new(self, 17)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&mut self) -> OscbypassW<MrSpec> {
        OscbypassW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
