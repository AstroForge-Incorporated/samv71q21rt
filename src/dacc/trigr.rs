#[doc = "Register `TRIGR` reader"]
pub type R = crate::R<TrigrSpec>;
#[doc = "Register `TRIGR` writer"]
pub type W = crate::W<TrigrSpec>;
#[doc = "Trigger Enable of Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgen0select {
    #[doc = "0: External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    Dis = 0,
    #[doc = "1: External trigger mode enabled."]
    En = 1,
}
impl From<Trgen0select> for bool {
    #[inline(always)]
    fn from(variant: Trgen0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN0` reader - Trigger Enable of Channel 0"]
pub type Trgen0R = crate::BitReader<Trgen0select>;
impl Trgen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgen0select {
        match self.bits {
            false => Trgen0select::Dis,
            true => Trgen0select::En,
        }
    }
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgen0select::Dis
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Trgen0select::En
    }
}
#[doc = "Field `TRGEN0` writer - Trigger Enable of Channel 0"]
pub type Trgen0W<'a, REG> = crate::BitWriter<'a, REG, Trgen0select>;
impl<'a, REG> Trgen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen0select::Dis)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen0select::En)
    }
}
#[doc = "Trigger Enable of Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgen1select {
    #[doc = "0: External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    Dis = 0,
    #[doc = "1: External trigger mode enabled."]
    En = 1,
}
impl From<Trgen1select> for bool {
    #[inline(always)]
    fn from(variant: Trgen1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEN1` reader - Trigger Enable of Channel 1"]
pub type Trgen1R = crate::BitReader<Trgen1select>;
impl Trgen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgen1select {
        match self.bits {
            false => Trgen1select::Dis,
            true => Trgen1select::En,
        }
    }
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trgen1select::Dis
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Trgen1select::En
    }
}
#[doc = "Field `TRGEN1` writer - Trigger Enable of Channel 1"]
pub type Trgen1W<'a, REG> = crate::BitWriter<'a, REG, Trgen1select>;
impl<'a, REG> Trgen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen1select::Dis)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Trgen1select::En)
    }
}
#[doc = "Trigger Selection of Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsel0select {
    #[doc = "0: DAC External Trigger Input (DATRG)"]
    Trgsel0 = 0,
    #[doc = "1: TC0 Channel 0 Output (TIOA0)"]
    Trgsel1 = 1,
    #[doc = "2: TC0 Channel 1 Output (TIOA1)"]
    Trgsel2 = 2,
    #[doc = "3: TC0 Channel 2 Output (TIOA2)"]
    Trgsel3 = 3,
    #[doc = "4: PWM0 Event Line 0"]
    Trgsel4 = 4,
    #[doc = "5: PWM0 Event Line 1"]
    Trgsel5 = 5,
    #[doc = "6: PWM1 Event Line 0"]
    Trgsel6 = 6,
    #[doc = "7: PWM1 Event Line 1"]
    Trgsel7 = 7,
}
impl From<Trgsel0select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsel0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsel0select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsel0select {}
#[doc = "Field `TRGSEL0` reader - Trigger Selection of Channel 0"]
pub type Trgsel0R = crate::FieldReader<Trgsel0select>;
impl Trgsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsel0select {
        match self.bits {
            0 => Trgsel0select::Trgsel0,
            1 => Trgsel0select::Trgsel1,
            2 => Trgsel0select::Trgsel2,
            3 => Trgsel0select::Trgsel3,
            4 => Trgsel0select::Trgsel4,
            5 => Trgsel0select::Trgsel5,
            6 => Trgsel0select::Trgsel6,
            7 => Trgsel0select::Trgsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC External Trigger Input (DATRG)"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == Trgsel0select::Trgsel0
    }
    #[doc = "TC0 Channel 0 Output (TIOA0)"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == Trgsel0select::Trgsel1
    }
    #[doc = "TC0 Channel 1 Output (TIOA1)"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == Trgsel0select::Trgsel2
    }
    #[doc = "TC0 Channel 2 Output (TIOA2)"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == Trgsel0select::Trgsel3
    }
    #[doc = "PWM0 Event Line 0"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == Trgsel0select::Trgsel4
    }
    #[doc = "PWM0 Event Line 1"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == Trgsel0select::Trgsel5
    }
    #[doc = "PWM1 Event Line 0"]
    #[inline(always)]
    pub fn is_trgsel6(&self) -> bool {
        *self == Trgsel0select::Trgsel6
    }
    #[doc = "PWM1 Event Line 1"]
    #[inline(always)]
    pub fn is_trgsel7(&self) -> bool {
        *self == Trgsel0select::Trgsel7
    }
}
#[doc = "Field `TRGSEL0` writer - Trigger Selection of Channel 0"]
pub type Trgsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgsel0select, crate::Safe>;
impl<'a, REG> Trgsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC External Trigger Input (DATRG)"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel0)
    }
    #[doc = "TC0 Channel 0 Output (TIOA0)"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel1)
    }
    #[doc = "TC0 Channel 1 Output (TIOA1)"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel2)
    }
    #[doc = "TC0 Channel 2 Output (TIOA2)"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel3)
    }
    #[doc = "PWM0 Event Line 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel4)
    }
    #[doc = "PWM0 Event Line 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel5)
    }
    #[doc = "PWM1 Event Line 0"]
    #[inline(always)]
    pub fn trgsel6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel6)
    }
    #[doc = "PWM1 Event Line 1"]
    #[inline(always)]
    pub fn trgsel7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel0select::Trgsel7)
    }
}
#[doc = "Trigger Selection of Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsel1select {
    #[doc = "0: DAC External Trigger Input (DATRG)"]
    Trgsel0 = 0,
    #[doc = "1: TC0 Channel 0 Output (TIOA0)"]
    Trgsel1 = 1,
    #[doc = "2: TC0 Channel 1 Output (TIOA1)"]
    Trgsel2 = 2,
    #[doc = "3: TC0 Channel 2 Output (TIOA2)"]
    Trgsel3 = 3,
    #[doc = "4: PWM0 Event Line 0"]
    Trgsel4 = 4,
    #[doc = "5: PWM0 Event Line 1"]
    Trgsel5 = 5,
    #[doc = "6: PWM1 Event Line 0"]
    Trgsel6 = 6,
    #[doc = "7: PWM1 Event Line 1"]
    Trgsel7 = 7,
}
impl From<Trgsel1select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsel1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsel1select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsel1select {}
#[doc = "Field `TRGSEL1` reader - Trigger Selection of Channel 1"]
pub type Trgsel1R = crate::FieldReader<Trgsel1select>;
impl Trgsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsel1select {
        match self.bits {
            0 => Trgsel1select::Trgsel0,
            1 => Trgsel1select::Trgsel1,
            2 => Trgsel1select::Trgsel2,
            3 => Trgsel1select::Trgsel3,
            4 => Trgsel1select::Trgsel4,
            5 => Trgsel1select::Trgsel5,
            6 => Trgsel1select::Trgsel6,
            7 => Trgsel1select::Trgsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC External Trigger Input (DATRG)"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == Trgsel1select::Trgsel0
    }
    #[doc = "TC0 Channel 0 Output (TIOA0)"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == Trgsel1select::Trgsel1
    }
    #[doc = "TC0 Channel 1 Output (TIOA1)"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == Trgsel1select::Trgsel2
    }
    #[doc = "TC0 Channel 2 Output (TIOA2)"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == Trgsel1select::Trgsel3
    }
    #[doc = "PWM0 Event Line 0"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == Trgsel1select::Trgsel4
    }
    #[doc = "PWM0 Event Line 1"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == Trgsel1select::Trgsel5
    }
    #[doc = "PWM1 Event Line 0"]
    #[inline(always)]
    pub fn is_trgsel6(&self) -> bool {
        *self == Trgsel1select::Trgsel6
    }
    #[doc = "PWM1 Event Line 1"]
    #[inline(always)]
    pub fn is_trgsel7(&self) -> bool {
        *self == Trgsel1select::Trgsel7
    }
}
#[doc = "Field `TRGSEL1` writer - Trigger Selection of Channel 1"]
pub type Trgsel1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgsel1select, crate::Safe>;
impl<'a, REG> Trgsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC External Trigger Input (DATRG)"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel0)
    }
    #[doc = "TC0 Channel 0 Output (TIOA0)"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel1)
    }
    #[doc = "TC0 Channel 1 Output (TIOA1)"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel2)
    }
    #[doc = "TC0 Channel 2 Output (TIOA2)"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel3)
    }
    #[doc = "PWM0 Event Line 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel4)
    }
    #[doc = "PWM0 Event Line 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel5)
    }
    #[doc = "PWM1 Event Line 0"]
    #[inline(always)]
    pub fn trgsel6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel6)
    }
    #[doc = "PWM1 Event Line 1"]
    #[inline(always)]
    pub fn trgsel7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel1select::Trgsel7)
    }
}
#[doc = "Over Sampling Ratio of Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osr0select {
    #[doc = "0: OSR = 1"]
    Osr1 = 0,
    #[doc = "1: OSR = 2"]
    Osr2 = 1,
    #[doc = "2: OSR = 4"]
    Osr4 = 2,
    #[doc = "3: OSR = 8"]
    Osr8 = 3,
    #[doc = "4: OSR = 16"]
    Osr16 = 4,
    #[doc = "5: OSR = 32"]
    Osr32 = 5,
}
impl From<Osr0select> for u8 {
    #[inline(always)]
    fn from(variant: Osr0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osr0select {
    type Ux = u8;
}
impl crate::IsEnum for Osr0select {}
#[doc = "Field `OSR0` reader - Over Sampling Ratio of Channel 0"]
pub type Osr0R = crate::FieldReader<Osr0select>;
impl Osr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osr0select> {
        match self.bits {
            0 => Some(Osr0select::Osr1),
            1 => Some(Osr0select::Osr2),
            2 => Some(Osr0select::Osr4),
            3 => Some(Osr0select::Osr8),
            4 => Some(Osr0select::Osr16),
            5 => Some(Osr0select::Osr32),
            _ => None,
        }
    }
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == Osr0select::Osr1
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == Osr0select::Osr2
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == Osr0select::Osr4
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == Osr0select::Osr8
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == Osr0select::Osr16
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == Osr0select::Osr32
    }
}
#[doc = "Field `OSR0` writer - Over Sampling Ratio of Channel 0"]
pub type Osr0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Osr0select>;
impl<'a, REG> Osr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Osr0select::Osr1)
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Osr0select::Osr2)
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut crate::W<REG> {
        self.variant(Osr0select::Osr4)
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut crate::W<REG> {
        self.variant(Osr0select::Osr8)
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Osr0select::Osr16)
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut crate::W<REG> {
        self.variant(Osr0select::Osr32)
    }
}
#[doc = "Over Sampling Ratio of Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osr1select {
    #[doc = "0: OSR = 1"]
    Osr1 = 0,
    #[doc = "1: OSR = 2"]
    Osr2 = 1,
    #[doc = "2: OSR = 4"]
    Osr4 = 2,
    #[doc = "3: OSR = 8"]
    Osr8 = 3,
    #[doc = "4: OSR = 16"]
    Osr16 = 4,
    #[doc = "5: OSR = 32"]
    Osr32 = 5,
}
impl From<Osr1select> for u8 {
    #[inline(always)]
    fn from(variant: Osr1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osr1select {
    type Ux = u8;
}
impl crate::IsEnum for Osr1select {}
#[doc = "Field `OSR1` reader - Over Sampling Ratio of Channel 1"]
pub type Osr1R = crate::FieldReader<Osr1select>;
impl Osr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osr1select> {
        match self.bits {
            0 => Some(Osr1select::Osr1),
            1 => Some(Osr1select::Osr2),
            2 => Some(Osr1select::Osr4),
            3 => Some(Osr1select::Osr8),
            4 => Some(Osr1select::Osr16),
            5 => Some(Osr1select::Osr32),
            _ => None,
        }
    }
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == Osr1select::Osr1
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == Osr1select::Osr2
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == Osr1select::Osr4
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == Osr1select::Osr8
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == Osr1select::Osr16
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == Osr1select::Osr32
    }
}
#[doc = "Field `OSR1` writer - Over Sampling Ratio of Channel 1"]
pub type Osr1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Osr1select>;
impl<'a, REG> Osr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Osr1select::Osr1)
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Osr1select::Osr2)
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut crate::W<REG> {
        self.variant(Osr1select::Osr4)
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut crate::W<REG> {
        self.variant(Osr1select::Osr8)
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Osr1select::Osr16)
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut crate::W<REG> {
        self.variant(Osr1select::Osr32)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable of Channel 0"]
    #[inline(always)]
    pub fn trgen0(&self) -> Trgen0R {
        Trgen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger Enable of Channel 1"]
    #[inline(always)]
    pub fn trgen1(&self) -> Trgen1R {
        Trgen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger Selection of Channel 0"]
    #[inline(always)]
    pub fn trgsel0(&self) -> Trgsel0R {
        Trgsel0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Trigger Selection of Channel 1"]
    #[inline(always)]
    pub fn trgsel1(&self) -> Trgsel1R {
        Trgsel1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Over Sampling Ratio of Channel 0"]
    #[inline(always)]
    pub fn osr0(&self) -> Osr0R {
        Osr0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Over Sampling Ratio of Channel 1"]
    #[inline(always)]
    pub fn osr1(&self) -> Osr1R {
        Osr1R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable of Channel 0"]
    #[inline(always)]
    pub fn trgen0(&mut self) -> Trgen0W<TrigrSpec> {
        Trgen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger Enable of Channel 1"]
    #[inline(always)]
    pub fn trgen1(&mut self) -> Trgen1W<TrigrSpec> {
        Trgen1W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Trigger Selection of Channel 0"]
    #[inline(always)]
    pub fn trgsel0(&mut self) -> Trgsel0W<TrigrSpec> {
        Trgsel0W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Trigger Selection of Channel 1"]
    #[inline(always)]
    pub fn trgsel1(&mut self) -> Trgsel1W<TrigrSpec> {
        Trgsel1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Over Sampling Ratio of Channel 0"]
    #[inline(always)]
    pub fn osr0(&mut self) -> Osr0W<TrigrSpec> {
        Osr0W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Over Sampling Ratio of Channel 1"]
    #[inline(always)]
    pub fn osr1(&mut self) -> Osr1W<TrigrSpec> {
        Osr1W::new(self, 20)
    }
}
#[doc = "Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trigr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigrSpec;
impl crate::RegisterSpec for TrigrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigr::R`](R) reader structure"]
impl crate::Readable for TrigrSpec {}
#[doc = "`write(|w| ..)` method takes [`trigr::W`](W) writer structure"]
impl crate::Writable for TrigrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGR to value 0"]
impl crate::Resettable for TrigrSpec {}
