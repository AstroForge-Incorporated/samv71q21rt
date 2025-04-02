#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "SDRAMC Command Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    Normal = 0,
    #[doc = "1: The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    Nop = 1,
    #[doc = "2: The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    AllbanksPrecharge = 2,
    #[doc = "3: The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    LoadModereg = 3,
    #[doc = "4: The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    AutoRefresh = 4,
    #[doc = "5: The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    ExtLoadModereg = 5,
    #[doc = "6: Deep Power-down mode. Enters Deep Power-down mode."]
    DeepPowerdown = 6,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - SDRAMC Command Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::Normal),
            1 => Some(Modeselect::Nop),
            2 => Some(Modeselect::AllbanksPrecharge),
            3 => Some(Modeselect::LoadModereg),
            4 => Some(Modeselect::AutoRefresh),
            5 => Some(Modeselect::ExtLoadModereg),
            6 => Some(Modeselect::DeepPowerdown),
            _ => None,
        }
    }
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Modeselect::Normal
    }
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == Modeselect::Nop
    }
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn is_allbanks_precharge(&self) -> bool {
        *self == Modeselect::AllbanksPrecharge
    }
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn is_load_modereg(&self) -> bool {
        *self == Modeselect::LoadModereg
    }
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn is_auto_refresh(&self) -> bool {
        *self == Modeselect::AutoRefresh
    }
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline(always)]
    pub fn is_ext_load_modereg(&self) -> bool {
        *self == Modeselect::ExtLoadModereg
    }
    #[doc = "Deep Power-down mode. Enters Deep Power-down mode."]
    #[inline(always)]
    pub fn is_deep_powerdown(&self) -> bool {
        *self == Modeselect::DeepPowerdown
    }
}
#[doc = "Field `MODE` writer - SDRAMC Command Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Normal)
    }
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Nop)
    }
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn allbanks_precharge(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::AllbanksPrecharge)
    }
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn load_modereg(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::LoadModereg)
    }
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn auto_refresh(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::AutoRefresh)
    }
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline(always)]
    pub fn ext_load_modereg(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::ExtLoadModereg)
    }
    #[doc = "Deep Power-down mode. Enters Deep Power-down mode."]
    #[inline(always)]
    pub fn deep_powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::DeepPowerdown)
    }
}
impl R {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<MrSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "SDRAMC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
