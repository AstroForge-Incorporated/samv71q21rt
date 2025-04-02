#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CccrSpec>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CccrSpec>;
#[doc = "Initialization (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initselect {
    #[doc = "0: Normal operation."]
    Disabled = 0,
    #[doc = "1: Initialization is started."]
    Enabled = 1,
}
impl From<Initselect> for bool {
    #[inline(always)]
    fn from(variant: Initselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization (read/write)"]
pub type InitR = crate::BitReader<Initselect>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initselect {
        match self.bits {
            false => Initselect::Disabled,
            true => Initselect::Enabled,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Initselect::Disabled
    }
    #[doc = "Initialization is started."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Initselect::Enabled
    }
}
#[doc = "Field `INIT` writer - Initialization (read/write)"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Initselect>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Initselect::Disabled)
    }
    #[doc = "Initialization is started."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Initselect::Enabled)
    }
}
#[doc = "Configuration Change Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cceselect {
    #[doc = "0: The processor has no write access to the protected configuration registers."]
    Protected = 0,
    #[doc = "1: The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    Configurable = 1,
}
impl From<Cceselect> for bool {
    #[inline(always)]
    fn from(variant: Cceselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable (read/write, write protection)"]
pub type CceR = crate::BitReader<Cceselect>;
impl CceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cceselect {
        match self.bits {
            false => Cceselect::Protected,
            true => Cceselect::Configurable,
        }
    }
    #[doc = "The processor has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == Cceselect::Protected
    }
    #[doc = "The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    #[inline(always)]
    pub fn is_configurable(&self) -> bool {
        *self == Cceselect::Configurable
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable (read/write, write protection)"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG, Cceselect>;
impl<'a, REG> CceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The processor has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(Cceselect::Protected)
    }
    #[doc = "The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    #[inline(always)]
    pub fn configurable(self) -> &'a mut crate::W<REG> {
        self.variant(Cceselect::Configurable)
    }
}
#[doc = "Restricted Operation Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asmselect {
    #[doc = "0: Normal CAN operation."]
    Normal = 0,
    #[doc = "1: Restricted Operation mode active."]
    Restricted = 1,
}
impl From<Asmselect> for bool {
    #[inline(always)]
    fn from(variant: Asmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASM` reader - Restricted Operation Mode (read/write, write protection against '1')"]
pub type AsmR = crate::BitReader<Asmselect>;
impl AsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asmselect {
        match self.bits {
            false => Asmselect::Normal,
            true => Asmselect::Restricted,
        }
    }
    #[doc = "Normal CAN operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Asmselect::Normal
    }
    #[doc = "Restricted Operation mode active."]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == Asmselect::Restricted
    }
}
#[doc = "Field `ASM` writer - Restricted Operation Mode (read/write, write protection against '1')"]
pub type AsmW<'a, REG> = crate::BitWriter<'a, REG, Asmselect>;
impl<'a, REG> AsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal CAN operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Asmselect::Normal)
    }
    #[doc = "Restricted Operation mode active."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut crate::W<REG> {
        self.variant(Asmselect::Restricted)
    }
}
#[doc = "Field `CSA` reader - Clock Stop Acknowledge (read-only)"]
pub type CsaR = crate::BitReader;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge (read-only)"]
pub type CsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Stop Request (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csrselect {
    #[doc = "0: No clock stop is requested."]
    NoClockStop = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    ClockStop = 1,
}
impl From<Csrselect> for bool {
    #[inline(always)]
    fn from(variant: Csrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Clock Stop Request (read/write)"]
pub type CsrR = crate::BitReader<Csrselect>;
impl CsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csrselect {
        match self.bits {
            false => Csrselect::NoClockStop,
            true => Csrselect::ClockStop,
        }
    }
    #[doc = "No clock stop is requested."]
    #[inline(always)]
    pub fn is_no_clock_stop(&self) -> bool {
        *self == Csrselect::NoClockStop
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn is_clock_stop(&self) -> bool {
        *self == Csrselect::ClockStop
    }
}
#[doc = "Field `CSR` writer - Clock Stop Request (read/write)"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG, Csrselect>;
impl<'a, REG> CsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No clock stop is requested."]
    #[inline(always)]
    pub fn no_clock_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Csrselect::NoClockStop)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn clock_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Csrselect::ClockStop)
    }
}
#[doc = "Bus Monitoring Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monselect {
    #[doc = "0: Bus Monitoring mode is disabled."]
    Disabled = 0,
    #[doc = "1: Bus Monitoring mode is enabled."]
    Enabled = 1,
}
impl From<Monselect> for bool {
    #[inline(always)]
    fn from(variant: Monselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Bus Monitoring Mode (read/write, write protection against '1')"]
pub type MonR = crate::BitReader<Monselect>;
impl MonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monselect {
        match self.bits {
            false => Monselect::Disabled,
            true => Monselect::Enabled,
        }
    }
    #[doc = "Bus Monitoring mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Monselect::Disabled
    }
    #[doc = "Bus Monitoring mode is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Monselect::Enabled
    }
}
#[doc = "Field `MON` writer - Bus Monitoring Mode (read/write, write protection against '1')"]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG, Monselect>;
impl<'a, REG> MonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Monitoring mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monselect::Disabled)
    }
    #[doc = "Bus Monitoring mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monselect::Enabled)
    }
}
#[doc = "Disable Automatic Retransmission (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Darselect {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled."]
    AutoRetx = 0,
    #[doc = "1: Automatic retransmission disabled."]
    NoAutoRetx = 1,
}
impl From<Darselect> for bool {
    #[inline(always)]
    fn from(variant: Darselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAR` reader - Disable Automatic Retransmission (read/write, write protection)"]
pub type DarR = crate::BitReader<Darselect>;
impl DarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Darselect {
        match self.bits {
            false => Darselect::AutoRetx,
            true => Darselect::NoAutoRetx,
        }
    }
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled."]
    #[inline(always)]
    pub fn is_auto_retx(&self) -> bool {
        *self == Darselect::AutoRetx
    }
    #[doc = "Automatic retransmission disabled."]
    #[inline(always)]
    pub fn is_no_auto_retx(&self) -> bool {
        *self == Darselect::NoAutoRetx
    }
}
#[doc = "Field `DAR` writer - Disable Automatic Retransmission (read/write, write protection)"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG, Darselect>;
impl<'a, REG> DarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled."]
    #[inline(always)]
    pub fn auto_retx(self) -> &'a mut crate::W<REG> {
        self.variant(Darselect::AutoRetx)
    }
    #[doc = "Automatic retransmission disabled."]
    #[inline(always)]
    pub fn no_auto_retx(self) -> &'a mut crate::W<REG> {
        self.variant(Darselect::NoAutoRetx)
    }
}
#[doc = "Test Mode Enable (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testselect {
    #[doc = "0: Normal operation, MCAN_TEST register holds reset values."]
    Disabled = 0,
    #[doc = "1: Test mode, write access to MCAN_TEST register enabled."]
    Enabled = 1,
}
impl From<Testselect> for bool {
    #[inline(always)]
    fn from(variant: Testselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST` reader - Test Mode Enable (read/write, write protection against '1')"]
pub type TestR = crate::BitReader<Testselect>;
impl TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testselect {
        match self.bits {
            false => Testselect::Disabled,
            true => Testselect::Enabled,
        }
    }
    #[doc = "Normal operation, MCAN_TEST register holds reset values."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Testselect::Disabled
    }
    #[doc = "Test mode, write access to MCAN_TEST register enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Testselect::Enabled
    }
}
#[doc = "Field `TEST` writer - Test Mode Enable (read/write, write protection against '1')"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG, Testselect>;
impl<'a, REG> TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, MCAN_TEST register holds reset values."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Testselect::Disabled)
    }
    #[doc = "Test mode, write access to MCAN_TEST register enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Testselect::Enabled)
    }
}
#[doc = "CAN FD Operation Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdoeselect {
    #[doc = "0: FD operation disabled."]
    Disabled = 0,
    #[doc = "1: FD operation enabled."]
    Enabled = 1,
}
impl From<Fdoeselect> for bool {
    #[inline(always)]
    fn from(variant: Fdoeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDOE` reader - CAN FD Operation Enable (read/write, write protection)"]
pub type FdoeR = crate::BitReader<Fdoeselect>;
impl FdoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdoeselect {
        match self.bits {
            false => Fdoeselect::Disabled,
            true => Fdoeselect::Enabled,
        }
    }
    #[doc = "FD operation disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fdoeselect::Disabled
    }
    #[doc = "FD operation enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fdoeselect::Enabled
    }
}
#[doc = "Field `FDOE` writer - CAN FD Operation Enable (read/write, write protection)"]
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG, Fdoeselect>;
impl<'a, REG> FdoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FD operation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fdoeselect::Disabled)
    }
    #[doc = "FD operation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fdoeselect::Enabled)
    }
}
#[doc = "Bit Rate Switching Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brseselect {
    #[doc = "0: Bit rate switching for transmissions disabled."]
    Disabled = 0,
    #[doc = "1: Bit rate switching for transmissions enabled."]
    Enabled = 1,
}
impl From<Brseselect> for bool {
    #[inline(always)]
    fn from(variant: Brseselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSE` reader - Bit Rate Switching Enable (read/write, write protection)"]
pub type BrseR = crate::BitReader<Brseselect>;
impl BrseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brseselect {
        match self.bits {
            false => Brseselect::Disabled,
            true => Brseselect::Enabled,
        }
    }
    #[doc = "Bit rate switching for transmissions disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Brseselect::Disabled
    }
    #[doc = "Bit rate switching for transmissions enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Brseselect::Enabled
    }
}
#[doc = "Field `BRSE` writer - Bit Rate Switching Enable (read/write, write protection)"]
pub type BrseW<'a, REG> = crate::BitWriter<'a, REG, Brseselect>;
impl<'a, REG> BrseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit rate switching for transmissions disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Brseselect::Disabled)
    }
    #[doc = "Bit rate switching for transmissions enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Brseselect::Enabled)
    }
}
#[doc = "Field `PXHD` reader - Protocol Exception Event Handling (read/write, write protection)"]
pub type PxhdR = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol Exception Event Handling (read/write, write protection)"]
pub type PxhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration (read/write, write protection)"]
pub type EfbiR = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration (read/write, write protection)"]
pub type EfbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - Transmit Pause (read/write, write protection)"]
pub type TxpR = crate::BitReader;
#[doc = "Field `TXP` writer - Transmit Pause (read/write, write protection)"]
pub type TxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - Non-ISO Operation"]
pub type NisoR = crate::BitReader;
#[doc = "Field `NISO` writer - Non-ISO Operation"]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&self) -> AsmR {
        AsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN FD Operation Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switching Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn brse(&self) -> BrseR {
        BrseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Event Handling (read/write, write protection)"]
    #[inline(always)]
    pub fn pxhd(&self) -> PxhdR {
        PxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration (read/write, write protection)"]
    #[inline(always)]
    pub fn efbi(&self) -> EfbiR {
        EfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<CccrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<CccrSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&mut self) -> AsmW<CccrSpec> {
        AsmW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&mut self) -> CsaW<CccrSpec> {
        CsaW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&mut self) -> CsrW<CccrSpec> {
        CsrW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&mut self) -> MonW<CccrSpec> {
        MonW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<CccrSpec> {
        DarW::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&mut self) -> TestW<CccrSpec> {
        TestW::new(self, 7)
    }
    #[doc = "Bit 8 - CAN FD Operation Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn fdoe(&mut self) -> FdoeW<CccrSpec> {
        FdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - Bit Rate Switching Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn brse(&mut self) -> BrseW<CccrSpec> {
        BrseW::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol Exception Event Handling (read/write, write protection)"]
    #[inline(always)]
    pub fn pxhd(&mut self) -> PxhdW<CccrSpec> {
        PxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration (read/write, write protection)"]
    #[inline(always)]
    pub fn efbi(&mut self) -> EfbiW<CccrSpec> {
        EfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&mut self) -> TxpW<CccrSpec> {
        TxpW::new(self, 14)
    }
    #[doc = "Bit 15 - Non-ISO Operation"]
    #[inline(always)]
    pub fn niso(&mut self) -> NisoW<CccrSpec> {
        NisoW::new(self, 15)
    }
}
#[doc = "CC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccrSpec;
impl crate::RegisterSpec for CccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CccrSpec {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCCR to value 0"]
impl crate::Resettable for CccrSpec {}
