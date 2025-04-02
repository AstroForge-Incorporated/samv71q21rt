#[doc = "Register `SCFG[%s]` reader"]
pub type R = crate::R<ScfgSpec>;
#[doc = "Register `SCFG[%s]` writer"]
pub type W = crate::W<ScfgSpec>;
#[doc = "Field `SLOT_CYCLE` reader - Maximum Bus Grant Duration for Masters"]
pub type SlotCycleR = crate::FieldReader<u16>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Bus Grant Duration for Masters"]
pub type SlotCycleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Default Master Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DefmstrTypeselect {
    #[doc = "0: No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    None = 0,
    #[doc = "1: Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    Last = 1,
    #[doc = "2: Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    Fixed = 2,
}
impl From<DefmstrTypeselect> for u8 {
    #[inline(always)]
    fn from(variant: DefmstrTypeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DefmstrTypeselect {
    type Ux = u8;
}
impl crate::IsEnum for DefmstrTypeselect {}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DefmstrTypeR = crate::FieldReader<DefmstrTypeselect>;
impl DefmstrTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DefmstrTypeselect> {
        match self.bits {
            0 => Some(DefmstrTypeselect::None),
            1 => Some(DefmstrTypeselect::Last),
            2 => Some(DefmstrTypeselect::Fixed),
            _ => None,
        }
    }
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DefmstrTypeselect::None
    }
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DefmstrTypeselect::Last
    }
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DefmstrTypeselect::Fixed
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DefmstrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DefmstrTypeselect>;
impl<'a, REG> DefmstrTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrTypeselect::None)
    }
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrTypeselect::Last)
    }
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrTypeselect::Fixed)
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FixedDefmstrR = crate::FieldReader;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FixedDefmstrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SlotCycleR {
        SlotCycleR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DefmstrTypeR {
        DefmstrTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FixedDefmstrR {
        FixedDefmstrR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SlotCycleW<ScfgSpec> {
        SlotCycleW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DefmstrTypeW<ScfgSpec> {
        DefmstrTypeW::new(self, 16)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FixedDefmstrW<ScfgSpec> {
        FixedDefmstrW::new(self, 18)
    }
}
#[doc = "Slave Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScfgSpec;
impl crate::RegisterSpec for ScfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfg::R`](R) reader structure"]
impl crate::Readable for ScfgSpec {}
#[doc = "`write(|w| ..)` method takes [`scfg::W`](W) writer structure"]
impl crate::Writable for ScfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCFG[%s] to value 0"]
impl crate::Resettable for ScfgSpec {}
