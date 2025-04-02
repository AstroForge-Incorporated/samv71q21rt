#[doc = "Register `CMR_WAVEFORM_MODE` reader"]
pub type R = crate::R<CmrWaveformModeSpec>;
#[doc = "Register `CMR_WAVEFORM_MODE` writer"]
pub type W = crate::W<CmrWaveformModeSpec>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcclksselect {
    #[doc = "0: Clock selected: internal PCK6 clock signal (from PMC)"]
    TimerClock1 = 0,
    #[doc = "1: Clock selected: internal MCK/8 clock signal (from PMC)"]
    TimerClock2 = 1,
    #[doc = "2: Clock selected: internal MCK/32 clock signal (from PMC)"]
    TimerClock3 = 2,
    #[doc = "3: Clock selected: internal MCK/128 clock signal (from PMC)"]
    TimerClock4 = 3,
    #[doc = "4: Clock selected: internal SLCK clock signal (from PMC)"]
    TimerClock5 = 4,
    #[doc = "5: Clock selected: XC0"]
    Xc0 = 5,
    #[doc = "6: Clock selected: XC1"]
    Xc1 = 6,
    #[doc = "7: Clock selected: XC2"]
    Xc2 = 7,
}
impl From<Tcclksselect> for u8 {
    #[inline(always)]
    fn from(variant: Tcclksselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcclksselect {
    type Ux = u8;
}
impl crate::IsEnum for Tcclksselect {}
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub type TcclksR = crate::FieldReader<Tcclksselect>;
impl TcclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcclksselect {
        match self.bits {
            0 => Tcclksselect::TimerClock1,
            1 => Tcclksselect::TimerClock2,
            2 => Tcclksselect::TimerClock3,
            3 => Tcclksselect::TimerClock4,
            4 => Tcclksselect::TimerClock5,
            5 => Tcclksselect::Xc0,
            6 => Tcclksselect::Xc1,
            7 => Tcclksselect::Xc2,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == Tcclksselect::TimerClock1
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == Tcclksselect::TimerClock2
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == Tcclksselect::TimerClock3
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == Tcclksselect::TimerClock4
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == Tcclksselect::TimerClock5
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == Tcclksselect::Xc0
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == Tcclksselect::Xc1
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == Tcclksselect::Xc2
    }
}
#[doc = "Field `TCCLKS` writer - Clock Selection"]
pub type TcclksW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tcclksselect, crate::Safe>;
impl<'a, REG> TcclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::TimerClock1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::TimerClock2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::TimerClock3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::TimerClock4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::TimerClock5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::Xc0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::Xc1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut crate::W<REG> {
        self.variant(Tcclksselect::Xc2)
    }
}
#[doc = "Field `CLKI` reader - Clock Invert"]
pub type ClkiR = crate::BitReader;
#[doc = "Field `CLKI` writer - Clock Invert"]
pub type ClkiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstselect {
    #[doc = "0: The clock is not gated by an external signal."]
    None = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    Xc0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    Xc1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    Xc2 = 3,
}
impl From<Burstselect> for u8 {
    #[inline(always)]
    fn from(variant: Burstselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burstselect {
    type Ux = u8;
}
impl crate::IsEnum for Burstselect {}
#[doc = "Field `BURST` reader - Burst Signal Selection"]
pub type BurstR = crate::FieldReader<Burstselect>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burstselect {
        match self.bits {
            0 => Burstselect::None,
            1 => Burstselect::Xc0,
            2 => Burstselect::Xc1,
            3 => Burstselect::Xc2,
            _ => unreachable!(),
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Burstselect::None
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == Burstselect::Xc0
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == Burstselect::Xc1
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == Burstselect::Xc2
    }
}
#[doc = "Field `BURST` writer - Burst Signal Selection"]
pub type BurstW<'a, REG> = crate::FieldWriter<'a, REG, 2, Burstselect, crate::Safe>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Burstselect::None)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut crate::W<REG> {
        self.variant(Burstselect::Xc0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut crate::W<REG> {
        self.variant(Burstselect::Xc1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut crate::W<REG> {
        self.variant(Burstselect::Xc2)
    }
}
#[doc = "Field `CPCSTOP` reader - Counter Clock Stopped with RC Compare"]
pub type CpcstopR = crate::BitReader;
#[doc = "Field `CPCSTOP` writer - Counter Clock Stopped with RC Compare"]
pub type CpcstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCDIS` reader - Counter Clock Disable with RC Loading"]
pub type CpcdisR = crate::BitReader;
#[doc = "Field `CPCDIS` writer - Counter Clock Disable with RC Loading"]
pub type CpcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Event Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eevtedgselect {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Each edges"]
    Edge = 3,
}
impl From<Eevtedgselect> for u8 {
    #[inline(always)]
    fn from(variant: Eevtedgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eevtedgselect {
    type Ux = u8;
}
impl crate::IsEnum for Eevtedgselect {}
#[doc = "Field `EEVTEDG` reader - External Event Edge Selection"]
pub type EevtedgR = crate::FieldReader<Eevtedgselect>;
impl EevtedgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eevtedgselect {
        match self.bits {
            0 => Eevtedgselect::None,
            1 => Eevtedgselect::Rising,
            2 => Eevtedgselect::Falling,
            3 => Eevtedgselect::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Eevtedgselect::None
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Eevtedgselect::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Eevtedgselect::Falling
    }
    #[doc = "Each edges"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Eevtedgselect::Edge
    }
}
#[doc = "Field `EEVTEDG` writer - External Event Edge Selection"]
pub type EevtedgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eevtedgselect, crate::Safe>;
impl<'a, REG> EevtedgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedgselect::None)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedgselect::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedgselect::Falling)
    }
    #[doc = "Each edges"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtedgselect::Edge)
    }
}
#[doc = "External Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eevtselect {
    #[doc = "0: TIOB"]
    Tiob = 0,
    #[doc = "1: XC0"]
    Xc0 = 1,
    #[doc = "2: XC1"]
    Xc1 = 2,
    #[doc = "3: XC2"]
    Xc2 = 3,
}
impl From<Eevtselect> for u8 {
    #[inline(always)]
    fn from(variant: Eevtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eevtselect {
    type Ux = u8;
}
impl crate::IsEnum for Eevtselect {}
#[doc = "Field `EEVT` reader - External Event Selection"]
pub type EevtR = crate::FieldReader<Eevtselect>;
impl EevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eevtselect {
        match self.bits {
            0 => Eevtselect::Tiob,
            1 => Eevtselect::Xc0,
            2 => Eevtselect::Xc1,
            3 => Eevtselect::Xc2,
            _ => unreachable!(),
        }
    }
    #[doc = "TIOB"]
    #[inline(always)]
    pub fn is_tiob(&self) -> bool {
        *self == Eevtselect::Tiob
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == Eevtselect::Xc0
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == Eevtselect::Xc1
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == Eevtselect::Xc2
    }
}
#[doc = "Field `EEVT` writer - External Event Selection"]
pub type EevtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eevtselect, crate::Safe>;
impl<'a, REG> EevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIOB"]
    #[inline(always)]
    pub fn tiob(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtselect::Tiob)
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtselect::Xc0)
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtselect::Xc1)
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut crate::W<REG> {
        self.variant(Eevtselect::Xc2)
    }
}
#[doc = "Field `ENETRG` reader - External Event Trigger Enable"]
pub type EnetrgR = crate::BitReader;
#[doc = "Field `ENETRG` writer - External Event Trigger Enable"]
pub type EnetrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Waveform Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavselselect {
    #[doc = "0: UP mode without automatic trigger on RC Compare"]
    Up = 0,
    #[doc = "1: UPDOWN mode without automatic trigger on RC Compare"]
    Updown = 1,
    #[doc = "2: UP mode with automatic trigger on RC Compare"]
    UpRc = 2,
    #[doc = "3: UPDOWN mode with automatic trigger on RC Compare"]
    UpdownRc = 3,
}
impl From<Wavselselect> for u8 {
    #[inline(always)]
    fn from(variant: Wavselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wavselselect {
    type Ux = u8;
}
impl crate::IsEnum for Wavselselect {}
#[doc = "Field `WAVSEL` reader - Waveform Selection"]
pub type WavselR = crate::FieldReader<Wavselselect>;
impl WavselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wavselselect {
        match self.bits {
            0 => Wavselselect::Up,
            1 => Wavselselect::Updown,
            2 => Wavselselect::UpRc,
            3 => Wavselselect::UpdownRc,
            _ => unreachable!(),
        }
    }
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Wavselselect::Up
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Wavselselect::Updown
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_up_rc(&self) -> bool {
        *self == Wavselselect::UpRc
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn is_updown_rc(&self) -> bool {
        *self == Wavselselect::UpdownRc
    }
}
#[doc = "Field `WAVSEL` writer - Waveform Selection"]
pub type WavselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wavselselect, crate::Safe>;
impl<'a, REG> WavselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Wavselselect::Up)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Wavselselect::Updown)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_rc(self) -> &'a mut crate::W<REG> {
        self.variant(Wavselselect::UpRc)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_rc(self) -> &'a mut crate::W<REG> {
        self.variant(Wavselselect::UpdownRc)
    }
}
#[doc = "Field `WAVE` reader - Waveform Mode"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Mode"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RA Compare Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acpaselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Acpaselect> for u8 {
    #[inline(always)]
    fn from(variant: Acpaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acpaselect {
    type Ux = u8;
}
impl crate::IsEnum for Acpaselect {}
#[doc = "Field `ACPA` reader - RA Compare Effect on TIOAx"]
pub type AcpaR = crate::FieldReader<Acpaselect>;
impl AcpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acpaselect {
        match self.bits {
            0 => Acpaselect::None,
            1 => Acpaselect::Set,
            2 => Acpaselect::Clear,
            3 => Acpaselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Acpaselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Acpaselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Acpaselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Acpaselect::Toggle
    }
}
#[doc = "Field `ACPA` writer - RA Compare Effect on TIOAx"]
pub type AcpaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acpaselect, crate::Safe>;
impl<'a, REG> AcpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Acpaselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Acpaselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Acpaselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Acpaselect::Toggle)
    }
}
#[doc = "RC Compare Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acpcselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Acpcselect> for u8 {
    #[inline(always)]
    fn from(variant: Acpcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acpcselect {
    type Ux = u8;
}
impl crate::IsEnum for Acpcselect {}
#[doc = "Field `ACPC` reader - RC Compare Effect on TIOAx"]
pub type AcpcR = crate::FieldReader<Acpcselect>;
impl AcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acpcselect {
        match self.bits {
            0 => Acpcselect::None,
            1 => Acpcselect::Set,
            2 => Acpcselect::Clear,
            3 => Acpcselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Acpcselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Acpcselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Acpcselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Acpcselect::Toggle
    }
}
#[doc = "Field `ACPC` writer - RC Compare Effect on TIOAx"]
pub type AcpcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acpcselect, crate::Safe>;
impl<'a, REG> AcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Acpcselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Acpcselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Acpcselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Acpcselect::Toggle)
    }
}
#[doc = "External Event Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aeevtselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Aeevtselect> for u8 {
    #[inline(always)]
    fn from(variant: Aeevtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aeevtselect {
    type Ux = u8;
}
impl crate::IsEnum for Aeevtselect {}
#[doc = "Field `AEEVT` reader - External Event Effect on TIOAx"]
pub type AeevtR = crate::FieldReader<Aeevtselect>;
impl AeevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aeevtselect {
        match self.bits {
            0 => Aeevtselect::None,
            1 => Aeevtselect::Set,
            2 => Aeevtselect::Clear,
            3 => Aeevtselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Aeevtselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Aeevtselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Aeevtselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Aeevtselect::Toggle
    }
}
#[doc = "Field `AEEVT` writer - External Event Effect on TIOAx"]
pub type AeevtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aeevtselect, crate::Safe>;
impl<'a, REG> AeevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevtselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevtselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevtselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Aeevtselect::Toggle)
    }
}
#[doc = "Software Trigger Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aswtrgselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Aswtrgselect> for u8 {
    #[inline(always)]
    fn from(variant: Aswtrgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aswtrgselect {
    type Ux = u8;
}
impl crate::IsEnum for Aswtrgselect {}
#[doc = "Field `ASWTRG` reader - Software Trigger Effect on TIOAx"]
pub type AswtrgR = crate::FieldReader<Aswtrgselect>;
impl AswtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aswtrgselect {
        match self.bits {
            0 => Aswtrgselect::None,
            1 => Aswtrgselect::Set,
            2 => Aswtrgselect::Clear,
            3 => Aswtrgselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Aswtrgselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Aswtrgselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Aswtrgselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Aswtrgselect::Toggle
    }
}
#[doc = "Field `ASWTRG` writer - Software Trigger Effect on TIOAx"]
pub type AswtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aswtrgselect, crate::Safe>;
impl<'a, REG> AswtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrgselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrgselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrgselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Aswtrgselect::Toggle)
    }
}
#[doc = "RB Compare Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcpbselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Bcpbselect> for u8 {
    #[inline(always)]
    fn from(variant: Bcpbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcpbselect {
    type Ux = u8;
}
impl crate::IsEnum for Bcpbselect {}
#[doc = "Field `BCPB` reader - RB Compare Effect on TIOBx"]
pub type BcpbR = crate::FieldReader<Bcpbselect>;
impl BcpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcpbselect {
        match self.bits {
            0 => Bcpbselect::None,
            1 => Bcpbselect::Set,
            2 => Bcpbselect::Clear,
            3 => Bcpbselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Bcpbselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bcpbselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Bcpbselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Bcpbselect::Toggle
    }
}
#[doc = "Field `BCPB` writer - RB Compare Effect on TIOBx"]
pub type BcpbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bcpbselect, crate::Safe>;
impl<'a, REG> BcpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpbselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpbselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpbselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpbselect::Toggle)
    }
}
#[doc = "RC Compare Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcpcselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Bcpcselect> for u8 {
    #[inline(always)]
    fn from(variant: Bcpcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcpcselect {
    type Ux = u8;
}
impl crate::IsEnum for Bcpcselect {}
#[doc = "Field `BCPC` reader - RC Compare Effect on TIOBx"]
pub type BcpcR = crate::FieldReader<Bcpcselect>;
impl BcpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcpcselect {
        match self.bits {
            0 => Bcpcselect::None,
            1 => Bcpcselect::Set,
            2 => Bcpcselect::Clear,
            3 => Bcpcselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Bcpcselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bcpcselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Bcpcselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Bcpcselect::Toggle
    }
}
#[doc = "Field `BCPC` writer - RC Compare Effect on TIOBx"]
pub type BcpcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bcpcselect, crate::Safe>;
impl<'a, REG> BcpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpcselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpcselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpcselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Bcpcselect::Toggle)
    }
}
#[doc = "External Event Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Beevtselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Beevtselect> for u8 {
    #[inline(always)]
    fn from(variant: Beevtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Beevtselect {
    type Ux = u8;
}
impl crate::IsEnum for Beevtselect {}
#[doc = "Field `BEEVT` reader - External Event Effect on TIOBx"]
pub type BeevtR = crate::FieldReader<Beevtselect>;
impl BeevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Beevtselect {
        match self.bits {
            0 => Beevtselect::None,
            1 => Beevtselect::Set,
            2 => Beevtselect::Clear,
            3 => Beevtselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Beevtselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Beevtselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Beevtselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Beevtselect::Toggle
    }
}
#[doc = "Field `BEEVT` writer - External Event Effect on TIOBx"]
pub type BeevtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Beevtselect, crate::Safe>;
impl<'a, REG> BeevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Beevtselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Beevtselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Beevtselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Beevtselect::Toggle)
    }
}
#[doc = "Software Trigger Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bswtrgselect {
    #[doc = "0: NONE"]
    None = 0,
    #[doc = "1: SET"]
    Set = 1,
    #[doc = "2: CLEAR"]
    Clear = 2,
    #[doc = "3: TOGGLE"]
    Toggle = 3,
}
impl From<Bswtrgselect> for u8 {
    #[inline(always)]
    fn from(variant: Bswtrgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bswtrgselect {
    type Ux = u8;
}
impl crate::IsEnum for Bswtrgselect {}
#[doc = "Field `BSWTRG` reader - Software Trigger Effect on TIOBx"]
pub type BswtrgR = crate::FieldReader<Bswtrgselect>;
impl BswtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bswtrgselect {
        match self.bits {
            0 => Bswtrgselect::None,
            1 => Bswtrgselect::Set,
            2 => Bswtrgselect::Clear,
            3 => Bswtrgselect::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Bswtrgselect::None
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bswtrgselect::Set
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Bswtrgselect::Clear
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Bswtrgselect::Toggle
    }
}
#[doc = "Field `BSWTRG` writer - Software Trigger Effect on TIOBx"]
pub type BswtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bswtrgselect, crate::Safe>;
impl<'a, REG> BswtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrgselect::None)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrgselect::Set)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrgselect::Clear)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Bswtrgselect::Toggle)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TcclksR {
        TcclksR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> ClkiR {
        ClkiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&self) -> CpcstopR {
        CpcstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Loading"]
    #[inline(always)]
    pub fn cpcdis(&self) -> CpcdisR {
        CpcdisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&self) -> EevtedgR {
        EevtedgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&self) -> EevtR {
        EevtR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&self) -> EnetrgR {
        EnetrgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&self) -> WavselR {
        WavselR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpa(&self) -> AcpaR {
        AcpaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpc(&self) -> AcpcR {
        AcpcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOAx"]
    #[inline(always)]
    pub fn aeevt(&self) -> AeevtR {
        AeevtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOAx"]
    #[inline(always)]
    pub fn aswtrg(&self) -> AswtrgR {
        AswtrgR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpb(&self) -> BcpbR {
        BcpbR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpc(&self) -> BcpcR {
        BcpcR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOBx"]
    #[inline(always)]
    pub fn beevt(&self) -> BeevtR {
        BeevtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOBx"]
    #[inline(always)]
    pub fn bswtrg(&self) -> BswtrgR {
        BswtrgR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TcclksW<CmrWaveformModeSpec> {
        TcclksW::new(self, 0)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> ClkiW<CmrWaveformModeSpec> {
        ClkiW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BurstW<CmrWaveformModeSpec> {
        BurstW::new(self, 4)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&mut self) -> CpcstopW<CmrWaveformModeSpec> {
        CpcstopW::new(self, 6)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Loading"]
    #[inline(always)]
    pub fn cpcdis(&mut self) -> CpcdisW<CmrWaveformModeSpec> {
        CpcdisW::new(self, 7)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&mut self) -> EevtedgW<CmrWaveformModeSpec> {
        EevtedgW::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&mut self) -> EevtW<CmrWaveformModeSpec> {
        EevtW::new(self, 10)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&mut self) -> EnetrgW<CmrWaveformModeSpec> {
        EnetrgW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&mut self) -> WavselW<CmrWaveformModeSpec> {
        WavselW::new(self, 13)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<CmrWaveformModeSpec> {
        WaveW::new(self, 15)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpa(&mut self) -> AcpaW<CmrWaveformModeSpec> {
        AcpaW::new(self, 16)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpc(&mut self) -> AcpcW<CmrWaveformModeSpec> {
        AcpcW::new(self, 18)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOAx"]
    #[inline(always)]
    pub fn aeevt(&mut self) -> AeevtW<CmrWaveformModeSpec> {
        AeevtW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOAx"]
    #[inline(always)]
    pub fn aswtrg(&mut self) -> AswtrgW<CmrWaveformModeSpec> {
        AswtrgW::new(self, 22)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpb(&mut self) -> BcpbW<CmrWaveformModeSpec> {
        BcpbW::new(self, 24)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpc(&mut self) -> BcpcW<CmrWaveformModeSpec> {
        BcpcW::new(self, 26)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOBx"]
    #[inline(always)]
    pub fn beevt(&mut self) -> BeevtW<CmrWaveformModeSpec> {
        BeevtW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOBx"]
    #[inline(always)]
    pub fn bswtrg(&mut self) -> BswtrgW<CmrWaveformModeSpec> {
        BswtrgW::new(self, 30)
    }
}
#[doc = "Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr_waveform_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_waveform_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrWaveformModeSpec;
impl crate::RegisterSpec for CmrWaveformModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr_waveform_mode::R`](R) reader structure"]
impl crate::Readable for CmrWaveformModeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmr_waveform_mode::W`](W) writer structure"]
impl crate::Writable for CmrWaveformModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMR_WAVEFORM_MODE to value 0"]
impl crate::Resettable for CmrWaveformModeSpec {}
