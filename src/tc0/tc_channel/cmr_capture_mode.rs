#[doc = "Register `CMR_CAPTURE_MODE` reader"]
pub type R = crate::R<CmrCaptureModeSpec>;
#[doc = "Register `CMR_CAPTURE_MODE` writer"]
pub type W = crate::W<CmrCaptureModeSpec>;
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
#[doc = "Field `LDBSTOP` reader - Counter Clock Stopped with RB Loading"]
pub type LdbstopR = crate::BitReader;
#[doc = "Field `LDBSTOP` writer - Counter Clock Stopped with RB Loading"]
pub type LdbstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDBDIS` reader - Counter Clock Disable with RB Loading"]
pub type LdbdisR = crate::BitReader;
#[doc = "Field `LDBDIS` writer - Counter Clock Disable with RB Loading"]
pub type LdbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etrgedgselect {
    #[doc = "0: The clock is not gated by an external signal."]
    None = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Each edge"]
    Edge = 3,
}
impl From<Etrgedgselect> for u8 {
    #[inline(always)]
    fn from(variant: Etrgedgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etrgedgselect {
    type Ux = u8;
}
impl crate::IsEnum for Etrgedgselect {}
#[doc = "Field `ETRGEDG` reader - External Trigger Edge Selection"]
pub type EtrgedgR = crate::FieldReader<Etrgedgselect>;
impl EtrgedgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etrgedgselect {
        match self.bits {
            0 => Etrgedgselect::None,
            1 => Etrgedgselect::Rising,
            2 => Etrgedgselect::Falling,
            3 => Etrgedgselect::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Etrgedgselect::None
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Etrgedgselect::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Etrgedgselect::Falling
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Etrgedgselect::Edge
    }
}
#[doc = "Field `ETRGEDG` writer - External Trigger Edge Selection"]
pub type EtrgedgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Etrgedgselect, crate::Safe>;
impl<'a, REG> EtrgedgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedgselect::None)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedgselect::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedgselect::Falling)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Etrgedgselect::Edge)
    }
}
#[doc = "Field `ABETRG` reader - TIOAx or TIOBx External Trigger Selection"]
pub type AbetrgR = crate::BitReader;
#[doc = "Field `ABETRG` writer - TIOAx or TIOBx External Trigger Selection"]
pub type AbetrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPCTRG` reader - RC Compare Trigger Enable"]
pub type CpctrgR = crate::BitReader;
#[doc = "Field `CPCTRG` writer - RC Compare Trigger Enable"]
pub type CpctrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - Waveform Mode"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Mode"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RA Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldraselect {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rising edge of TIOAx"]
    Rising = 1,
    #[doc = "2: Falling edge of TIOAx"]
    Falling = 2,
    #[doc = "3: Each edge of TIOAx"]
    Edge = 3,
}
impl From<Ldraselect> for u8 {
    #[inline(always)]
    fn from(variant: Ldraselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldraselect {
    type Ux = u8;
}
impl crate::IsEnum for Ldraselect {}
#[doc = "Field `LDRA` reader - RA Loading Edge Selection"]
pub type LdraR = crate::FieldReader<Ldraselect>;
impl LdraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldraselect {
        match self.bits {
            0 => Ldraselect::None,
            1 => Ldraselect::Rising,
            2 => Ldraselect::Falling,
            3 => Ldraselect::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ldraselect::None
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Ldraselect::Rising
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Ldraselect::Falling
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Ldraselect::Edge
    }
}
#[doc = "Field `LDRA` writer - RA Loading Edge Selection"]
pub type LdraW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ldraselect, crate::Safe>;
impl<'a, REG> LdraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ldraselect::None)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ldraselect::Rising)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ldraselect::Falling)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ldraselect::Edge)
    }
}
#[doc = "RB Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldrbselect {
    #[doc = "0: None"]
    None = 0,
    #[doc = "1: Rising edge of TIOAx"]
    Rising = 1,
    #[doc = "2: Falling edge of TIOAx"]
    Falling = 2,
    #[doc = "3: Each edge of TIOAx"]
    Edge = 3,
}
impl From<Ldrbselect> for u8 {
    #[inline(always)]
    fn from(variant: Ldrbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldrbselect {
    type Ux = u8;
}
impl crate::IsEnum for Ldrbselect {}
#[doc = "Field `LDRB` reader - RB Loading Edge Selection"]
pub type LdrbR = crate::FieldReader<Ldrbselect>;
impl LdrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldrbselect {
        match self.bits {
            0 => Ldrbselect::None,
            1 => Ldrbselect::Rising,
            2 => Ldrbselect::Falling,
            3 => Ldrbselect::Edge,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ldrbselect::None
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Ldrbselect::Rising
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Ldrbselect::Falling
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Ldrbselect::Edge
    }
}
#[doc = "Field `LDRB` writer - RB Loading Edge Selection"]
pub type LdrbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ldrbselect, crate::Safe>;
impl<'a, REG> LdrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrbselect::None)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrbselect::Rising)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrbselect::Falling)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ldrbselect::Edge)
    }
}
#[doc = "Loading Edge Subsampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sbsmplrselect {
    #[doc = "0: Load a Capture Register each selected edge"]
    One = 0,
    #[doc = "1: Load a Capture Register every 2 selected edges"]
    Half = 1,
    #[doc = "2: Load a Capture Register every 4 selected edges"]
    Fourth = 2,
    #[doc = "3: Load a Capture Register every 8 selected edges"]
    Eighth = 3,
    #[doc = "4: Load a Capture Register every 16 selected edges"]
    Sixteenth = 4,
}
impl From<Sbsmplrselect> for u8 {
    #[inline(always)]
    fn from(variant: Sbsmplrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sbsmplrselect {
    type Ux = u8;
}
impl crate::IsEnum for Sbsmplrselect {}
#[doc = "Field `SBSMPLR` reader - Loading Edge Subsampling Ratio"]
pub type SbsmplrR = crate::FieldReader<Sbsmplrselect>;
impl SbsmplrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sbsmplrselect> {
        match self.bits {
            0 => Some(Sbsmplrselect::One),
            1 => Some(Sbsmplrselect::Half),
            2 => Some(Sbsmplrselect::Fourth),
            3 => Some(Sbsmplrselect::Eighth),
            4 => Some(Sbsmplrselect::Sixteenth),
            _ => None,
        }
    }
    #[doc = "Load a Capture Register each selected edge"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Sbsmplrselect::One
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Sbsmplrselect::Half
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == Sbsmplrselect::Fourth
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == Sbsmplrselect::Eighth
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline(always)]
    pub fn is_sixteenth(&self) -> bool {
        *self == Sbsmplrselect::Sixteenth
    }
}
#[doc = "Field `SBSMPLR` writer - Loading Edge Subsampling Ratio"]
pub type SbsmplrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sbsmplrselect>;
impl<'a, REG> SbsmplrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Load a Capture Register each selected edge"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsmplrselect::One)
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsmplrselect::Half)
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsmplrselect::Fourth)
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsmplrselect::Eighth)
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline(always)]
    pub fn sixteenth(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsmplrselect::Sixteenth)
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LdbstopR {
        LdbstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LdbdisR {
        LdbdisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> EtrgedgR {
        EtrgedgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> AbetrgR {
        AbetrgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CpctrgR {
        CpctrgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LdraR {
        LdraR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LdrbR {
        LdrbR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&self) -> SbsmplrR {
        SbsmplrR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TcclksW<CmrCaptureModeSpec> {
        TcclksW::new(self, 0)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> ClkiW<CmrCaptureModeSpec> {
        ClkiW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BurstW<CmrCaptureModeSpec> {
        BurstW::new(self, 4)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&mut self) -> LdbstopW<CmrCaptureModeSpec> {
        LdbstopW::new(self, 6)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&mut self) -> LdbdisW<CmrCaptureModeSpec> {
        LdbdisW::new(self, 7)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&mut self) -> EtrgedgW<CmrCaptureModeSpec> {
        EtrgedgW::new(self, 8)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&mut self) -> AbetrgW<CmrCaptureModeSpec> {
        AbetrgW::new(self, 10)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&mut self) -> CpctrgW<CmrCaptureModeSpec> {
        CpctrgW::new(self, 14)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<CmrCaptureModeSpec> {
        WaveW::new(self, 15)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&mut self) -> LdraW<CmrCaptureModeSpec> {
        LdraW::new(self, 16)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&mut self) -> LdrbW<CmrCaptureModeSpec> {
        LdrbW::new(self, 18)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&mut self) -> SbsmplrW<CmrCaptureModeSpec> {
        SbsmplrW::new(self, 20)
    }
}
#[doc = "Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr_capture_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_capture_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrCaptureModeSpec;
impl crate::RegisterSpec for CmrCaptureModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr_capture_mode::R`](R) reader structure"]
impl crate::Readable for CmrCaptureModeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmr_capture_mode::W`](W) writer structure"]
impl crate::Writable for CmrCaptureModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMR_CAPTURE_MODE to value 0"]
impl crate::Resettable for CmrCaptureModeSpec {}
