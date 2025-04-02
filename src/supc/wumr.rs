#[doc = "Register `WUMR` reader"]
pub type R = crate::R<WumrSpec>;
#[doc = "Register `WUMR` writer"]
pub type W = crate::W<WumrSpec>;
#[doc = "Supply Monitor Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smenselect {
    #[doc = "0: The supply monitor detection has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: The supply monitor detection forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Smenselect> for bool {
    #[inline(always)]
    fn from(variant: Smenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake-up Enable"]
pub type SmenR = crate::BitReader<Smenselect>;
impl SmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smenselect {
        match self.bits {
            false => Smenselect::NotEnable,
            true => Smenselect::Enable,
        }
    }
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smenselect::NotEnable
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smenselect::Enable
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake-up Enable"]
pub type SmenW<'a, REG> = crate::BitWriter<'a, REG, Smenselect>;
impl<'a, REG> SmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smenselect::NotEnable)
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smenselect::Enable)
    }
}
#[doc = "Real-time Timer Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rttenselect {
    #[doc = "0: The RTT alarm signal has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: The RTT alarm signal forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Rttenselect> for bool {
    #[inline(always)]
    fn from(variant: Rttenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTTEN` reader - Real-time Timer Wake-up Enable"]
pub type RttenR = crate::BitReader<Rttenselect>;
impl RttenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rttenselect {
        match self.bits {
            false => Rttenselect::NotEnable,
            true => Rttenselect::Enable,
        }
    }
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Rttenselect::NotEnable
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rttenselect::Enable
    }
}
#[doc = "Field `RTTEN` writer - Real-time Timer Wake-up Enable"]
pub type RttenW<'a, REG> = crate::BitWriter<'a, REG, Rttenselect>;
impl<'a, REG> RttenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rttenselect::NotEnable)
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rttenselect::Enable)
    }
}
#[doc = "Real-time Clock Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcenselect {
    #[doc = "0: The RTC alarm signal has no wake-up effect."]
    NotEnable = 0,
    #[doc = "1: The RTC alarm signal forces the wake-up of the core power supply."]
    Enable = 1,
}
impl From<Rtcenselect> for bool {
    #[inline(always)]
    fn from(variant: Rtcenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - Real-time Clock Wake-up Enable"]
pub type RtcenR = crate::BitReader<Rtcenselect>;
impl RtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcenselect {
        match self.bits {
            false => Rtcenselect::NotEnable,
            true => Rtcenselect::Enable,
        }
    }
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Rtcenselect::NotEnable
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtcenselect::Enable
    }
}
#[doc = "Field `RTCEN` writer - Real-time Clock Wake-up Enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcenselect>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcenselect::NotEnable)
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcenselect::Enable)
    }
}
#[doc = "Low-power Debouncer Enable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcen0select {
    #[doc = "0: The WKUP0 input pin is not connected to the low-power debouncer."]
    NotEnable = 0,
    #[doc = "1: The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    Enable = 1,
}
impl From<Lpdbcen0select> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcen0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN0` reader - Low-power Debouncer Enable WKUP0"]
pub type Lpdbcen0R = crate::BitReader<Lpdbcen0select>;
impl Lpdbcen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcen0select {
        match self.bits {
            false => Lpdbcen0select::NotEnable,
            true => Lpdbcen0select::Enable,
        }
    }
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Lpdbcen0select::NotEnable
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpdbcen0select::Enable
    }
}
#[doc = "Field `LPDBCEN0` writer - Low-power Debouncer Enable WKUP0"]
pub type Lpdbcen0W<'a, REG> = crate::BitWriter<'a, REG, Lpdbcen0select>;
impl<'a, REG> Lpdbcen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen0select::NotEnable)
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen0select::Enable)
    }
}
#[doc = "Low-power Debouncer Enable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcen1select {
    #[doc = "0: The WKUP1 input pin is not connected to the low-power debouncer."]
    NotEnable = 0,
    #[doc = "1: The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    Enable = 1,
}
impl From<Lpdbcen1select> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcen1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN1` reader - Low-power Debouncer Enable WKUP1"]
pub type Lpdbcen1R = crate::BitReader<Lpdbcen1select>;
impl Lpdbcen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcen1select {
        match self.bits {
            false => Lpdbcen1select::NotEnable,
            true => Lpdbcen1select::Enable,
        }
    }
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Lpdbcen1select::NotEnable
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpdbcen1select::Enable
    }
}
#[doc = "Field `LPDBCEN1` writer - Low-power Debouncer Enable WKUP1"]
pub type Lpdbcen1W<'a, REG> = crate::BitWriter<'a, REG, Lpdbcen1select>;
impl<'a, REG> Lpdbcen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen1select::NotEnable)
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcen1select::Enable)
    }
}
#[doc = "Low-power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcclrselect {
    #[doc = "0: A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NotEnable = 0,
    #[doc = "1: A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    Enable = 1,
}
impl From<Lpdbcclrselect> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcclrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCCLR` reader - Low-power Debouncer Clear"]
pub type LpdbcclrR = crate::BitReader<Lpdbcclrselect>;
impl LpdbcclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcclrselect {
        match self.bits {
            false => Lpdbcclrselect::NotEnable,
            true => Lpdbcclrselect::Enable,
        }
    }
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Lpdbcclrselect::NotEnable
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lpdbcclrselect::Enable
    }
}
#[doc = "Field `LPDBCCLR` writer - Low-power Debouncer Clear"]
pub type LpdbcclrW<'a, REG> = crate::BitWriter<'a, REG, Lpdbcclrselect>;
impl<'a, REG> LpdbcclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcclrselect::NotEnable)
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcclrselect::Enable)
    }
}
#[doc = "Wake-up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wkupdbcselect {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    Immediate = 0,
    #[doc = "1: WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3Slck = 1,
    #[doc = "2: WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32Slck = 2,
    #[doc = "3: WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512Slck = 3,
    #[doc = "4: WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096Slck = 4,
    #[doc = "5: WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768Slck = 5,
}
impl From<Wkupdbcselect> for u8 {
    #[inline(always)]
    fn from(variant: Wkupdbcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wkupdbcselect {
    type Ux = u8;
}
impl crate::IsEnum for Wkupdbcselect {}
#[doc = "Field `WKUPDBC` reader - Wake-up Inputs Debouncer Period"]
pub type WkupdbcR = crate::FieldReader<Wkupdbcselect>;
impl WkupdbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wkupdbcselect> {
        match self.bits {
            0 => Some(Wkupdbcselect::Immediate),
            1 => Some(Wkupdbcselect::_3Slck),
            2 => Some(Wkupdbcselect::_32Slck),
            3 => Some(Wkupdbcselect::_512Slck),
            4 => Some(Wkupdbcselect::_4096Slck),
            5 => Some(Wkupdbcselect::_32768Slck),
            _ => None,
        }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Wkupdbcselect::Immediate
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn is_3_slck(&self) -> bool {
        *self == Wkupdbcselect::_3Slck
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32_slck(&self) -> bool {
        *self == Wkupdbcselect::_32Slck
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn is_512_slck(&self) -> bool {
        *self == Wkupdbcselect::_512Slck
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn is_4096_slck(&self) -> bool {
        *self == Wkupdbcselect::_4096Slck
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn is_32768_slck(&self) -> bool {
        *self == Wkupdbcselect::_32768Slck
    }
}
#[doc = "Field `WKUPDBC` writer - Wake-up Inputs Debouncer Period"]
pub type WkupdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wkupdbcselect>;
impl<'a, REG> WkupdbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbcselect::Immediate)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_slck(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbcselect::_3Slck)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_slck(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbcselect::_32Slck)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_slck(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbcselect::_512Slck)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_slck(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbcselect::_4096Slck)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_slck(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupdbcselect::_32768Slck)
    }
}
#[doc = "Low-power Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpdbcselect {
    #[doc = "0: Disable the low-power debouncers."]
    Disable = 0,
    #[doc = "1: WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2Rtcout = 1,
    #[doc = "2: WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3Rtcout = 2,
    #[doc = "3: WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4Rtcout = 3,
    #[doc = "4: WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5Rtcout = 4,
    #[doc = "5: WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6Rtcout = 5,
    #[doc = "6: WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7Rtcout = 6,
    #[doc = "7: WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8Rtcout = 7,
}
impl From<Lpdbcselect> for u8 {
    #[inline(always)]
    fn from(variant: Lpdbcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpdbcselect {
    type Ux = u8;
}
impl crate::IsEnum for Lpdbcselect {}
#[doc = "Field `LPDBC` reader - Low-power Debouncer Period"]
pub type LpdbcR = crate::FieldReader<Lpdbcselect>;
impl LpdbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcselect {
        match self.bits {
            0 => Lpdbcselect::Disable,
            1 => Lpdbcselect::_2Rtcout,
            2 => Lpdbcselect::_3Rtcout,
            3 => Lpdbcselect::_4Rtcout,
            4 => Lpdbcselect::_5Rtcout,
            5 => Lpdbcselect::_6Rtcout,
            6 => Lpdbcselect::_7Rtcout,
            7 => Lpdbcselect::_8Rtcout,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable the low-power debouncers."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lpdbcselect::Disable
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_2_rtcout(&self) -> bool {
        *self == Lpdbcselect::_2Rtcout
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_3_rtcout(&self) -> bool {
        *self == Lpdbcselect::_3Rtcout
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_4_rtcout(&self) -> bool {
        *self == Lpdbcselect::_4Rtcout
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_5_rtcout(&self) -> bool {
        *self == Lpdbcselect::_5Rtcout
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_6_rtcout(&self) -> bool {
        *self == Lpdbcselect::_6Rtcout
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_7_rtcout(&self) -> bool {
        *self == Lpdbcselect::_7Rtcout
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn is_8_rtcout(&self) -> bool {
        *self == Lpdbcselect::_8Rtcout
    }
}
#[doc = "Field `LPDBC` writer - Low-power Debouncer Period"]
pub type LpdbcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lpdbcselect, crate::Safe>;
impl<'a, REG> LpdbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable the low-power debouncers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::Disable)
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _2_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_2Rtcout)
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _3_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_3Rtcout)
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _4_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_4Rtcout)
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _5_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_5Rtcout)
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _6_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_6Rtcout)
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _7_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_7Rtcout)
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _8_rtcout(self) -> &'a mut crate::W<REG> {
        self.variant(Lpdbcselect::_8Rtcout)
    }
}
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SmenR {
        SmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RttenR {
        RttenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> Lpdbcen0R {
        Lpdbcen0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> Lpdbcen1R {
        Lpdbcen1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LpdbcclrR {
        LpdbcclrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WkupdbcR {
        WkupdbcR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LpdbcR {
        LpdbcR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SmenW<WumrSpec> {
        SmenW::new(self, 1)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&mut self) -> RttenW<WumrSpec> {
        RttenW::new(self, 2)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<WumrSpec> {
        RtcenW::new(self, 3)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&mut self) -> Lpdbcen0W<WumrSpec> {
        Lpdbcen0W::new(self, 5)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&mut self) -> Lpdbcen1W<WumrSpec> {
        Lpdbcen1W::new(self, 6)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&mut self) -> LpdbcclrW<WumrSpec> {
        LpdbcclrW::new(self, 7)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&mut self) -> WkupdbcW<WumrSpec> {
        WkupdbcW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&mut self) -> LpdbcW<WumrSpec> {
        LpdbcW::new(self, 16)
    }
}
#[doc = "Supply Controller Wake-up Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wumr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wumr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WumrSpec;
impl crate::RegisterSpec for WumrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wumr::R`](R) reader structure"]
impl crate::Readable for WumrSpec {}
#[doc = "`write(|w| ..)` method takes [`wumr::W`](W) writer structure"]
impl crate::Writable for WumrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUMR to value 0"]
impl crate::Resettable for WumrSpec {}
