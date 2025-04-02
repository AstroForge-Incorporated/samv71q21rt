#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgenselect {
    #[doc = "0: Hardware triggers are disabled. Starting a conversion is only possible by software."]
    Dis = 0,
    #[doc = "1: Hardware trigger selected by TRGSEL field is enabled."]
    En = 1,
}
impl From<Trgenselect> for bool {
    #[inline(always)]
    fn from(variant: Trgenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TrgenR = crate::BitReader<Trgenselect>;
impl TrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgenselect {
        match self.bits {
            false => Trgenselect::Dis,
            true => Trgenselect::En,
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgenselect::Dis
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Trgenselect::En
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TrgenW<'a, REG> = crate::BitWriter<'a, REG, Trgenselect>;
impl<'a, REG> TrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgenselect::Dis)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Trgenselect::En)
    }
}
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgselselect {
    #[doc = "0: AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    AfecTrig0 = 0,
    #[doc = "1: TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    AfecTrig1 = 1,
    #[doc = "2: TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    AfecTrig2 = 2,
    #[doc = "3: TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    AfecTrig3 = 3,
    #[doc = "4: PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    AfecTrig4 = 4,
    #[doc = "5: PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    AfecTrig5 = 5,
    #[doc = "6: Analog Comparator"]
    AfecTrig6 = 6,
}
impl From<Trgselselect> for u8 {
    #[inline(always)]
    fn from(variant: Trgselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgselselect {
    type Ux = u8;
}
impl crate::IsEnum for Trgselselect {}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TrgselR = crate::FieldReader<Trgselselect>;
impl TrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgselselect> {
        match self.bits {
            0 => Some(Trgselselect::AfecTrig0),
            1 => Some(Trgselselect::AfecTrig1),
            2 => Some(Trgselselect::AfecTrig2),
            3 => Some(Trgselselect::AfecTrig3),
            4 => Some(Trgselselect::AfecTrig4),
            5 => Some(Trgselselect::AfecTrig5),
            6 => Some(Trgselselect::AfecTrig6),
            _ => None,
        }
    }
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig0(&self) -> bool {
        *self == Trgselselect::AfecTrig0
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig1(&self) -> bool {
        *self == Trgselselect::AfecTrig1
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig2(&self) -> bool {
        *self == Trgselselect::AfecTrig2
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig3(&self) -> bool {
        *self == Trgselselect::AfecTrig3
    }
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig4(&self) -> bool {
        *self == Trgselselect::AfecTrig4
    }
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig5(&self) -> bool {
        *self == Trgselselect::AfecTrig5
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn is_afec_trig6(&self) -> bool {
        *self == Trgselselect::AfecTrig6
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgselselect>;
impl<'a, REG> TrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    #[inline(always)]
    pub fn afec_trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig0)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig1)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig2)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig3)
    }
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig4)
    }
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig5)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn afec_trig6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgselselect::AfecTrig6)
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepselect {
    #[doc = "0: Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    Normal = 0,
    #[doc = "1: Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    Sleep = 1,
}
impl From<Sleepselect> for bool {
    #[inline(always)]
    fn from(variant: Sleepselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SleepR = crate::BitReader<Sleepselect>;
impl SleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepselect {
        match self.bits {
            false => Sleepselect::Normal,
            true => Sleepselect::Sleep,
        }
    }
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sleepselect::Normal
    }
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Sleepselect::Sleep
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG, Sleepselect>;
impl<'a, REG> SleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepselect::Normal)
    }
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepselect::Sleep)
    }
}
#[doc = "Fast Wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwupselect {
    #[doc = "0: Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    Off = 0,
    #[doc = "1: Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    On = 1,
}
impl From<Fwupselect> for bool {
    #[inline(always)]
    fn from(variant: Fwupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWUP` reader - Fast Wake-up"]
pub type FwupR = crate::BitReader<Fwupselect>;
impl FwupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwupselect {
        match self.bits {
            false => Fwupselect::Off,
            true => Fwupselect::On,
        }
    }
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Fwupselect::Off
    }
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Fwupselect::On
    }
}
#[doc = "Field `FWUP` writer - Fast Wake-up"]
pub type FwupW<'a, REG> = crate::BitWriter<'a, REG, Fwupselect>;
impl<'a, REG> FwupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupselect::Off)
    }
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Fwupselect::On)
    }
}
#[doc = "Free Run Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Freerunselect {
    #[doc = "0: Normal mode"]
    Off = 0,
    #[doc = "1: Free Run mode: Never wait for any trigger."]
    On = 1,
}
impl From<Freerunselect> for bool {
    #[inline(always)]
    fn from(variant: Freerunselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREERUN` reader - Free Run Mode"]
pub type FreerunR = crate::BitReader<Freerunselect>;
impl FreerunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freerunselect {
        match self.bits {
            false => Freerunselect::Off,
            true => Freerunselect::On,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Freerunselect::Off
    }
    #[doc = "Free Run mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Freerunselect::On
    }
}
#[doc = "Field `FREERUN` writer - Free Run Mode"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG, Freerunselect>;
impl<'a, REG> FreerunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Freerunselect::Off)
    }
    #[doc = "Free Run mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Freerunselect::On)
    }
}
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PrescalR = crate::FieldReader;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PrescalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Start-up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 0 periods of AFE clock"]
    Sut0 = 0,
    #[doc = "1: 8 periods of AFE clock"]
    Sut8 = 1,
    #[doc = "2: 16 periods of AFE clock"]
    Sut16 = 2,
    #[doc = "3: 24 periods of AFE clock"]
    Sut24 = 3,
    #[doc = "4: 64 periods of AFE clock"]
    Sut64 = 4,
    #[doc = "5: 80 periods of AFE clock"]
    Sut80 = 5,
    #[doc = "6: 96 periods of AFE clock"]
    Sut96 = 6,
    #[doc = "7: 112 periods of AFE clock"]
    Sut112 = 7,
    #[doc = "8: 512 periods of AFE clock"]
    Sut512 = 8,
    #[doc = "9: 576 periods of AFE clock"]
    Sut576 = 9,
    #[doc = "10: 640 periods of AFE clock"]
    Sut640 = 10,
    #[doc = "11: 704 periods of AFE clock"]
    Sut704 = 11,
    #[doc = "12: 768 periods of AFE clock"]
    Sut768 = 12,
    #[doc = "13: 832 periods of AFE clock"]
    Sut832 = 13,
    #[doc = "14: 896 periods of AFE clock"]
    Sut896 = 14,
    #[doc = "15: 960 periods of AFE clock"]
    Sut960 = 15,
}
impl From<Startupselect> for u8 {
    #[inline(always)]
    fn from(variant: Startupselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startupselect {
    type Ux = u8;
}
impl crate::IsEnum for Startupselect {}
#[doc = "Field `STARTUP` reader - Start-up Time"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startupselect {
        match self.bits {
            0 => Startupselect::Sut0,
            1 => Startupselect::Sut8,
            2 => Startupselect::Sut16,
            3 => Startupselect::Sut24,
            4 => Startupselect::Sut64,
            5 => Startupselect::Sut80,
            6 => Startupselect::Sut96,
            7 => Startupselect::Sut112,
            8 => Startupselect::Sut512,
            9 => Startupselect::Sut576,
            10 => Startupselect::Sut640,
            11 => Startupselect::Sut704,
            12 => Startupselect::Sut768,
            13 => Startupselect::Sut832,
            14 => Startupselect::Sut896,
            15 => Startupselect::Sut960,
            _ => unreachable!(),
        }
    }
    #[doc = "0 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == Startupselect::Sut0
    }
    #[doc = "8 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == Startupselect::Sut8
    }
    #[doc = "16 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == Startupselect::Sut16
    }
    #[doc = "24 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == Startupselect::Sut24
    }
    #[doc = "64 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == Startupselect::Sut64
    }
    #[doc = "80 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == Startupselect::Sut80
    }
    #[doc = "96 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == Startupselect::Sut96
    }
    #[doc = "112 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == Startupselect::Sut112
    }
    #[doc = "512 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == Startupselect::Sut512
    }
    #[doc = "576 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == Startupselect::Sut576
    }
    #[doc = "640 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == Startupselect::Sut640
    }
    #[doc = "704 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == Startupselect::Sut704
    }
    #[doc = "768 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == Startupselect::Sut768
    }
    #[doc = "832 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == Startupselect::Sut832
    }
    #[doc = "896 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == Startupselect::Sut896
    }
    #[doc = "960 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == Startupselect::Sut960
    }
}
#[doc = "Field `STARTUP` writer - Start-up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startupselect, crate::Safe>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 periods of AFE clock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut0)
    }
    #[doc = "8 periods of AFE clock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut8)
    }
    #[doc = "16 periods of AFE clock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut16)
    }
    #[doc = "24 periods of AFE clock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut24)
    }
    #[doc = "64 periods of AFE clock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut64)
    }
    #[doc = "80 periods of AFE clock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut80)
    }
    #[doc = "96 periods of AFE clock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut96)
    }
    #[doc = "112 periods of AFE clock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut112)
    }
    #[doc = "512 periods of AFE clock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut512)
    }
    #[doc = "576 periods of AFE clock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut576)
    }
    #[doc = "640 periods of AFE clock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut640)
    }
    #[doc = "704 periods of AFE clock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut704)
    }
    #[doc = "768 periods of AFE clock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut768)
    }
    #[doc = "832 periods of AFE clock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut832)
    }
    #[doc = "896 periods of AFE clock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut896)
    }
    #[doc = "960 periods of AFE clock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Sut960)
    }
}
#[doc = "Field `ONE` reader - One"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - One"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACKTIM` reader - Tracking Time"]
pub type TracktimR = crate::FieldReader;
#[doc = "Field `TRACKTIM` writer - Tracking Time"]
pub type TracktimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRANSFER` reader - Transfer Period"]
pub type TransferR = crate::FieldReader;
#[doc = "Field `TRANSFER` writer - Transfer Period"]
pub type TransferW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "User Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Useqselect {
    #[doc = "0: Normal mode: The controller converts channels in a simple numeric order."]
    NumOrder = 0,
    #[doc = "1: User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    RegOrder = 1,
}
impl From<Useqselect> for bool {
    #[inline(always)]
    fn from(variant: Useqselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEQ` reader - User Sequence Enable"]
pub type UseqR = crate::BitReader<Useqselect>;
impl UseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Useqselect {
        match self.bits {
            false => Useqselect::NumOrder,
            true => Useqselect::RegOrder,
        }
    }
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == Useqselect::NumOrder
    }
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == Useqselect::RegOrder
    }
}
#[doc = "Field `USEQ` writer - User Sequence Enable"]
pub type UseqW<'a, REG> = crate::BitWriter<'a, REG, Useqselect>;
impl<'a, REG> UseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut crate::W<REG> {
        self.variant(Useqselect::NumOrder)
    }
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut crate::W<REG> {
        self.variant(Useqselect::RegOrder)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TrgenR {
        TrgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline(always)]
    pub fn fwup(&self) -> FwupR {
        FwupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PrescalR {
        PrescalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - One"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TracktimR {
        TracktimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&self) -> TransferR {
        TransferR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> UseqR {
        UseqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&mut self) -> TrgenW<MrSpec> {
        TrgenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<MrSpec> {
        TrgselW::new(self, 1)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<MrSpec> {
        SleepW::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline(always)]
    pub fn fwup(&mut self) -> FwupW<MrSpec> {
        FwupW::new(self, 6)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FreerunW<MrSpec> {
        FreerunW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&mut self) -> PrescalW<MrSpec> {
        PrescalW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> StartupW<MrSpec> {
        StartupW::new(self, 16)
    }
    #[doc = "Bit 23 - One"]
    #[inline(always)]
    pub fn one(&mut self) -> OneW<MrSpec> {
        OneW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&mut self) -> TracktimW<MrSpec> {
        TracktimW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&mut self) -> TransferW<MrSpec> {
        TransferW::new(self, 28)
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline(always)]
    pub fn useq(&mut self) -> UseqW<MrSpec> {
        UseqW::new(self, 31)
    }
}
#[doc = "AFEC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
