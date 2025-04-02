#[doc = "Register `LPR` reader"]
pub type R = crate::R<LprSpec>;
#[doc = "Register `LPR` writer"]
pub type W = crate::W<LprSpec>;
#[doc = "Low-power Configuration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpcbselect {
    #[doc = "0: The low-power feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    Disabled = 0,
    #[doc = "1: The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self-refresh mode when accessed and enters it after the access."]
    SelfRefresh = 1,
    #[doc = "2: The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down mode when accessed and enters it after the access."]
    PowerDown = 2,
    #[doc = "3: The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    DeepPowerDown = 3,
}
impl From<Lpcbselect> for u8 {
    #[inline(always)]
    fn from(variant: Lpcbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpcbselect {
    type Ux = u8;
}
impl crate::IsEnum for Lpcbselect {}
#[doc = "Field `LPCB` reader - Low-power Configuration Bits"]
pub type LpcbR = crate::FieldReader<Lpcbselect>;
impl LpcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpcbselect {
        match self.bits {
            0 => Lpcbselect::Disabled,
            1 => Lpcbselect::SelfRefresh,
            2 => Lpcbselect::PowerDown,
            3 => Lpcbselect::DeepPowerDown,
            _ => unreachable!(),
        }
    }
    #[doc = "The low-power feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lpcbselect::Disabled
    }
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self-refresh mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == Lpcbselect::SelfRefresh
    }
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == Lpcbselect::PowerDown
    }
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        *self == Lpcbselect::DeepPowerDown
    }
}
#[doc = "Field `LPCB` writer - Low-power Configuration Bits"]
pub type LpcbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpcbselect, crate::Safe>;
impl<'a, REG> LpcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The low-power feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcbselect::Disabled)
    }
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self-refresh mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn self_refresh(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcbselect::SelfRefresh)
    }
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcbselect::PowerDown)
    }
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcbselect::DeepPowerDown)
    }
}
#[doc = "Field `PASR` reader - Partial Array Self-refresh (only for low-power SDRAM)"]
pub type PasrR = crate::FieldReader;
#[doc = "Field `PASR` writer - Partial Array Self-refresh (only for low-power SDRAM)"]
pub type PasrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCSR` reader - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub type TcsrR = crate::FieldReader;
#[doc = "Field `TCSR` writer - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub type TcsrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DS` reader - Drive Strength (only for low-power SDRAM)"]
pub type DsR = crate::FieldReader;
#[doc = "Field `DS` writer - Drive Strength (only for low-power SDRAM)"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Time to Define When Low-power Mode Is Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutselect {
    #[doc = "0: The SDRAMC activates the SDRAM Low-power mode immediately after the end of the last transfer."]
    LpLastXfer = 0,
    #[doc = "1: The SDRAMC activates the SDRAM Low-power mode 64 clock cycles after the end of the last transfer."]
    LpLastXfer64 = 1,
    #[doc = "2: The SDRAMC activates the SDRAM Low-power mode 128 clock cycles after the end of the last transfer."]
    LpLastXfer128 = 2,
}
impl From<Timeoutselect> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutselect {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutselect {}
#[doc = "Field `TIMEOUT` reader - Time to Define When Low-power Mode Is Enabled"]
pub type TimeoutR = crate::FieldReader<Timeoutselect>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timeoutselect> {
        match self.bits {
            0 => Some(Timeoutselect::LpLastXfer),
            1 => Some(Timeoutselect::LpLastXfer64),
            2 => Some(Timeoutselect::LpLastXfer128),
            _ => None,
        }
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn is_lp_last_xfer(&self) -> bool {
        *self == Timeoutselect::LpLastXfer
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn is_lp_last_xfer_64(&self) -> bool {
        *self == Timeoutselect::LpLastXfer64
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn is_lp_last_xfer_128(&self) -> bool {
        *self == Timeoutselect::LpLastXfer128
    }
}
#[doc = "Field `TIMEOUT` writer - Time to Define When Low-power Mode Is Enabled"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timeoutselect>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The SDRAMC activates the SDRAM Low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutselect::LpLastXfer)
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_64(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutselect::LpLastXfer64)
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_128(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutselect::LpLastXfer128)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&self) -> LpcbR {
        LpcbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&self) -> PasrR {
        PasrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&self) -> TcsrR {
        TcsrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&mut self) -> LpcbW<LprSpec> {
        LpcbW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PasrW<LprSpec> {
        PasrW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&mut self) -> TcsrW<LprSpec> {
        TcsrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<LprSpec> {
        DsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<LprSpec> {
        TimeoutW::new(self, 12)
    }
}
#[doc = "SDRAMC Low Power Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LprSpec;
impl crate::RegisterSpec for LprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpr::R`](R) reader structure"]
impl crate::Readable for LprSpec {}
#[doc = "`write(|w| ..)` method takes [`lpr::W`](W) writer structure"]
impl crate::Writable for LprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPR to value 0"]
impl crate::Resettable for LprSpec {}
