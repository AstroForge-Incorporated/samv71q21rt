#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Acknowledge for Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackupdselect {
    #[doc = "0: Time and calendar registers cannot be updated."]
    Freerun = 0,
    #[doc = "1: Time and calendar registers can be updated."]
    Update = 1,
}
impl From<Ackupdselect> for bool {
    #[inline(always)]
    fn from(variant: Ackupdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKUPD` reader - Acknowledge for Update"]
pub type AckupdR = crate::BitReader<Ackupdselect>;
impl AckupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackupdselect {
        match self.bits {
            false => Ackupdselect::Freerun,
            true => Ackupdselect::Update,
        }
    }
    #[doc = "Time and calendar registers cannot be updated."]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == Ackupdselect::Freerun
    }
    #[doc = "Time and calendar registers can be updated."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Ackupdselect::Update
    }
}
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alarmselect {
    #[doc = "0: No alarm matching condition occurred."]
    NoAlarmevent = 0,
    #[doc = "1: An alarm matching condition has occurred."]
    Alarmevent = 1,
}
impl From<Alarmselect> for bool {
    #[inline(always)]
    fn from(variant: Alarmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM` reader - Alarm Flag"]
pub type AlarmR = crate::BitReader<Alarmselect>;
impl AlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alarmselect {
        match self.bits {
            false => Alarmselect::NoAlarmevent,
            true => Alarmselect::Alarmevent,
        }
    }
    #[doc = "No alarm matching condition occurred."]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == Alarmselect::NoAlarmevent
    }
    #[doc = "An alarm matching condition has occurred."]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == Alarmselect::Alarmevent
    }
}
#[doc = "Second Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secselect {
    #[doc = "0: No second event has occurred since the last clear."]
    NoSecevent = 0,
    #[doc = "1: At least one second event has occurred since the last clear."]
    Secevent = 1,
}
impl From<Secselect> for bool {
    #[inline(always)]
    fn from(variant: Secselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC` reader - Second Event"]
pub type SecR = crate::BitReader<Secselect>;
impl SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secselect {
        match self.bits {
            false => Secselect::NoSecevent,
            true => Secselect::Secevent,
        }
    }
    #[doc = "No second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == Secselect::NoSecevent
    }
    #[doc = "At least one second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == Secselect::Secevent
    }
}
#[doc = "Time Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timevselect {
    #[doc = "0: No time event has occurred since the last clear."]
    NoTimevent = 0,
    #[doc = "1: At least one time event has occurred since the last clear."]
    Timevent = 1,
}
impl From<Timevselect> for bool {
    #[inline(always)]
    fn from(variant: Timevselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEV` reader - Time Event"]
pub type TimevR = crate::BitReader<Timevselect>;
impl TimevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timevselect {
        match self.bits {
            false => Timevselect::NoTimevent,
            true => Timevselect::Timevent,
        }
    }
    #[doc = "No time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == Timevselect::NoTimevent
    }
    #[doc = "At least one time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == Timevselect::Timevent
    }
}
#[doc = "Calendar Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calevselect {
    #[doc = "0: No calendar event has occurred since the last clear."]
    NoCalevent = 0,
    #[doc = "1: At least one calendar event has occurred since the last clear."]
    Calevent = 1,
}
impl From<Calevselect> for bool {
    #[inline(always)]
    fn from(variant: Calevselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALEV` reader - Calendar Event"]
pub type CalevR = crate::BitReader<Calevselect>;
impl CalevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calevselect {
        match self.bits {
            false => Calevselect::NoCalevent,
            true => Calevselect::Calevent,
        }
    }
    #[doc = "No calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == Calevselect::NoCalevent
    }
    #[doc = "At least one calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == Calevselect::Calevent
    }
}
#[doc = "Time and/or Date Free Running Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tderrselect {
    #[doc = "0: The internal free running counters are carrying valid values since the last read of the Status Register (RTC_SR)."]
    Correct = 0,
    #[doc = "1: The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    ErrTimedate = 1,
}
impl From<Tderrselect> for bool {
    #[inline(always)]
    fn from(variant: Tderrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDERR` reader - Time and/or Date Free Running Error"]
pub type TderrR = crate::BitReader<Tderrselect>;
impl TderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tderrselect {
        match self.bits {
            false => Tderrselect::Correct,
            true => Tderrselect::ErrTimedate,
        }
    }
    #[doc = "The internal free running counters are carrying valid values since the last read of the Status Register (RTC_SR)."]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == Tderrselect::Correct
    }
    #[doc = "The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    #[inline(always)]
    pub fn is_err_timedate(&self) -> bool {
        *self == Tderrselect::ErrTimedate
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> AckupdR {
        AckupdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> AlarmR {
        AlarmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TimevR {
        TimevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CalevR {
        CalevR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error"]
    #[inline(always)]
    pub fn tderr(&self) -> TderrR {
        TderrR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
