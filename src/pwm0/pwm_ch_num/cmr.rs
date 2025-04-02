#[doc = "Register `CMR` reader"]
pub type R = crate::R<CmrSpec>;
#[doc = "Register `CMR` writer"]
pub type W = crate::W<CmrSpec>;
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpreselect {
    #[doc = "0: Peripheral clock"]
    Mck = 0,
    #[doc = "1: Peripheral clock/2"]
    MckDiv2 = 1,
    #[doc = "2: Peripheral clock/4"]
    MckDiv4 = 2,
    #[doc = "3: Peripheral clock/8"]
    MckDiv8 = 3,
    #[doc = "4: Peripheral clock/16"]
    MckDiv16 = 4,
    #[doc = "5: Peripheral clock/32"]
    MckDiv32 = 5,
    #[doc = "6: Peripheral clock/64"]
    MckDiv64 = 6,
    #[doc = "7: Peripheral clock/128"]
    MckDiv128 = 7,
    #[doc = "8: Peripheral clock/256"]
    MckDiv256 = 8,
    #[doc = "9: Peripheral clock/512"]
    MckDiv512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    MckDiv1024 = 10,
    #[doc = "11: Clock A"]
    Clka = 11,
    #[doc = "12: Clock B"]
    Clkb = 12,
}
impl From<Cpreselect> for u8 {
    #[inline(always)]
    fn from(variant: Cpreselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpreselect {
    type Ux = u8;
}
impl crate::IsEnum for Cpreselect {}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub type CpreR = crate::FieldReader<Cpreselect>;
impl CpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpreselect> {
        match self.bits {
            0 => Some(Cpreselect::Mck),
            1 => Some(Cpreselect::MckDiv2),
            2 => Some(Cpreselect::MckDiv4),
            3 => Some(Cpreselect::MckDiv8),
            4 => Some(Cpreselect::MckDiv16),
            5 => Some(Cpreselect::MckDiv32),
            6 => Some(Cpreselect::MckDiv64),
            7 => Some(Cpreselect::MckDiv128),
            8 => Some(Cpreselect::MckDiv256),
            9 => Some(Cpreselect::MckDiv512),
            10 => Some(Cpreselect::MckDiv1024),
            11 => Some(Cpreselect::Clka),
            12 => Some(Cpreselect::Clkb),
            _ => None,
        }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cpreselect::Mck
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        *self == Cpreselect::MckDiv2
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        *self == Cpreselect::MckDiv4
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        *self == Cpreselect::MckDiv8
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        *self == Cpreselect::MckDiv16
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        *self == Cpreselect::MckDiv32
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        *self == Cpreselect::MckDiv64
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        *self == Cpreselect::MckDiv128
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        *self == Cpreselect::MckDiv256
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        *self == Cpreselect::MckDiv512
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == Cpreselect::MckDiv1024
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == Cpreselect::Clka
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == Cpreselect::Clkb
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub type CpreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cpreselect>;
impl<'a, REG> CpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::Mck)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::MckDiv1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::Clka)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut crate::W<REG> {
        self.variant(Cpreselect::Clkb)
    }
}
#[doc = "Channel Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calgselect {
    #[doc = "0: Left aligned"]
    LeftAligned = 0,
    #[doc = "1: Center aligned"]
    CenterAligned = 1,
}
impl From<Calgselect> for bool {
    #[inline(always)]
    fn from(variant: Calgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub type CalgR = crate::BitReader<Calgselect>;
impl CalgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calgselect {
        match self.bits {
            false => Calgselect::LeftAligned,
            true => Calgselect::CenterAligned,
        }
    }
    #[doc = "Left aligned"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == Calgselect::LeftAligned
    }
    #[doc = "Center aligned"]
    #[inline(always)]
    pub fn is_center_aligned(&self) -> bool {
        *self == Calgselect::CenterAligned
    }
}
#[doc = "Field `CALG` writer - Channel Alignment"]
pub type CalgW<'a, REG> = crate::BitWriter<'a, REG, Calgselect>;
impl<'a, REG> CalgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Left aligned"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(Calgselect::LeftAligned)
    }
    #[doc = "Center aligned"]
    #[inline(always)]
    pub fn center_aligned(self) -> &'a mut crate::W<REG> {
        self.variant(Calgselect::CenterAligned)
    }
}
#[doc = "Channel Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpolselect {
    #[doc = "0: Waveform starts at low level"]
    LowPolarity = 0,
    #[doc = "1: Waveform starts at high level"]
    HighPolarity = 1,
}
impl From<Cpolselect> for bool {
    #[inline(always)]
    fn from(variant: Cpolselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub type CpolR = crate::BitReader<Cpolselect>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpolselect {
        match self.bits {
            false => Cpolselect::LowPolarity,
            true => Cpolselect::HighPolarity,
        }
    }
    #[doc = "Waveform starts at low level"]
    #[inline(always)]
    pub fn is_low_polarity(&self) -> bool {
        *self == Cpolselect::LowPolarity
    }
    #[doc = "Waveform starts at high level"]
    #[inline(always)]
    pub fn is_high_polarity(&self) -> bool {
        *self == Cpolselect::HighPolarity
    }
}
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpolselect>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Waveform starts at low level"]
    #[inline(always)]
    pub fn low_polarity(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::LowPolarity)
    }
    #[doc = "Waveform starts at high level"]
    #[inline(always)]
    pub fn high_polarity(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::HighPolarity)
    }
}
#[doc = "Counter Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cesselect {
    #[doc = "0: At the end of PWM period"]
    SingleEvent = 0,
    #[doc = "1: At half of PWM period AND at the end of PWM period"]
    DoubleEvent = 1,
}
impl From<Cesselect> for bool {
    #[inline(always)]
    fn from(variant: Cesselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CES` reader - Counter Event Selection"]
pub type CesR = crate::BitReader<Cesselect>;
impl CesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cesselect {
        match self.bits {
            false => Cesselect::SingleEvent,
            true => Cesselect::DoubleEvent,
        }
    }
    #[doc = "At the end of PWM period"]
    #[inline(always)]
    pub fn is_single_event(&self) -> bool {
        *self == Cesselect::SingleEvent
    }
    #[doc = "At half of PWM period AND at the end of PWM period"]
    #[inline(always)]
    pub fn is_double_event(&self) -> bool {
        *self == Cesselect::DoubleEvent
    }
}
#[doc = "Field `CES` writer - Counter Event Selection"]
pub type CesW<'a, REG> = crate::BitWriter<'a, REG, Cesselect>;
impl<'a, REG> CesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "At the end of PWM period"]
    #[inline(always)]
    pub fn single_event(self) -> &'a mut crate::W<REG> {
        self.variant(Cesselect::SingleEvent)
    }
    #[doc = "At half of PWM period AND at the end of PWM period"]
    #[inline(always)]
    pub fn double_event(self) -> &'a mut crate::W<REG> {
        self.variant(Cesselect::DoubleEvent)
    }
}
#[doc = "Update Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Updsselect {
    #[doc = "0: At the next end of PWM period"]
    UpdateAtPeriod = 0,
    #[doc = "1: At the next end of Half PWM period"]
    UpdateAtHalfPeriod = 1,
}
impl From<Updsselect> for bool {
    #[inline(always)]
    fn from(variant: Updsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDS` reader - Update Selection"]
pub type UpdsR = crate::BitReader<Updsselect>;
impl UpdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Updsselect {
        match self.bits {
            false => Updsselect::UpdateAtPeriod,
            true => Updsselect::UpdateAtHalfPeriod,
        }
    }
    #[doc = "At the next end of PWM period"]
    #[inline(always)]
    pub fn is_update_at_period(&self) -> bool {
        *self == Updsselect::UpdateAtPeriod
    }
    #[doc = "At the next end of Half PWM period"]
    #[inline(always)]
    pub fn is_update_at_half_period(&self) -> bool {
        *self == Updsselect::UpdateAtHalfPeriod
    }
}
#[doc = "Field `UPDS` writer - Update Selection"]
pub type UpdsW<'a, REG> = crate::BitWriter<'a, REG, Updsselect>;
impl<'a, REG> UpdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "At the next end of PWM period"]
    #[inline(always)]
    pub fn update_at_period(self) -> &'a mut crate::W<REG> {
        self.variant(Updsselect::UpdateAtPeriod)
    }
    #[doc = "At the next end of Half PWM period"]
    #[inline(always)]
    pub fn update_at_half_period(self) -> &'a mut crate::W<REG> {
        self.variant(Updsselect::UpdateAtHalfPeriod)
    }
}
#[doc = "Field `DPOLI` reader - Disabled Polarity Inverted"]
pub type DpoliR = crate::BitReader;
#[doc = "Field `DPOLI` writer - Disabled Polarity Inverted"]
pub type DpoliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCTS` reader - Timer Counter Trigger Selection"]
pub type TctsR = crate::BitReader;
#[doc = "Field `TCTS` writer - Timer Counter Trigger Selection"]
pub type TctsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE` reader - Dead-Time Generator Enable"]
pub type DteR = crate::BitReader;
#[doc = "Field `DTE` writer - Dead-Time Generator Enable"]
pub type DteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHI` reader - Dead-Time PWMHx Output Inverted"]
pub type DthiR = crate::BitReader;
#[doc = "Field `DTHI` writer - Dead-Time PWMHx Output Inverted"]
pub type DthiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLI` reader - Dead-Time PWMLx Output Inverted"]
pub type DtliR = crate::BitReader;
#[doc = "Field `DTLI` writer - Dead-Time PWMLx Output Inverted"]
pub type DtliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPM` reader - Push-Pull Mode"]
pub type PpmR = crate::BitReader;
#[doc = "Field `PPM` writer - Push-Pull Mode"]
pub type PpmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CpreR {
        CpreR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CalgR {
        CalgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CesR {
        CesR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&self) -> UpdsR {
        UpdsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&self) -> DpoliR {
        DpoliR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&self) -> TctsR {
        TctsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DteR {
        DteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DthiR {
        DthiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DtliR {
        DtliR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&self) -> PpmR {
        PpmR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&mut self) -> CpreW<CmrSpec> {
        CpreW::new(self, 0)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&mut self) -> CalgW<CmrSpec> {
        CalgW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<CmrSpec> {
        CpolW::new(self, 9)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&mut self) -> CesW<CmrSpec> {
        CesW::new(self, 10)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&mut self) -> UpdsW<CmrSpec> {
        UpdsW::new(self, 11)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&mut self) -> DpoliW<CmrSpec> {
        DpoliW::new(self, 12)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&mut self) -> TctsW<CmrSpec> {
        TctsW::new(self, 13)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DteW<CmrSpec> {
        DteW::new(self, 16)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&mut self) -> DthiW<CmrSpec> {
        DthiW::new(self, 17)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&mut self) -> DtliW<CmrSpec> {
        DtliW::new(self, 18)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&mut self) -> PpmW<CmrSpec> {
        PpmW::new(self, 19)
    }
}
#[doc = "PWM Channel Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrSpec;
impl crate::RegisterSpec for CmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr::R`](R) reader structure"]
impl crate::Readable for CmrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmr::W`](W) writer structure"]
impl crate::Writable for CmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CmrSpec {}
