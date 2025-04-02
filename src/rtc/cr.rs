#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `UPDTIM` reader - Update Request Time Register"]
pub type UpdtimR = crate::BitReader;
#[doc = "Field `UPDTIM` writer - Update Request Time Register"]
pub type UpdtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDCAL` reader - Update Request Calendar Register"]
pub type UpdcalR = crate::BitReader;
#[doc = "Field `UPDCAL` writer - Update Request Calendar Register"]
pub type UpdcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Time Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timevselselect {
    #[doc = "0: Minute change"]
    Minute = 0,
    #[doc = "1: Hour change"]
    Hour = 1,
    #[doc = "2: Every day at midnight"]
    Midnight = 2,
    #[doc = "3: Every day at noon"]
    Noon = 3,
}
impl From<Timevselselect> for u8 {
    #[inline(always)]
    fn from(variant: Timevselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timevselselect {
    type Ux = u8;
}
impl crate::IsEnum for Timevselselect {}
#[doc = "Field `TIMEVSEL` reader - Time Event Selection"]
pub type TimevselR = crate::FieldReader<Timevselselect>;
impl TimevselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timevselselect {
        match self.bits {
            0 => Timevselselect::Minute,
            1 => Timevselselect::Hour,
            2 => Timevselselect::Midnight,
            3 => Timevselselect::Noon,
            _ => unreachable!(),
        }
    }
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == Timevselselect::Minute
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == Timevselselect::Hour
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == Timevselselect::Midnight
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == Timevselselect::Noon
    }
}
#[doc = "Field `TIMEVSEL` writer - Time Event Selection"]
pub type TimevselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timevselselect, crate::Safe>;
impl<'a, REG> TimevselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut crate::W<REG> {
        self.variant(Timevselselect::Minute)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut crate::W<REG> {
        self.variant(Timevselselect::Hour)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut crate::W<REG> {
        self.variant(Timevselselect::Midnight)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut crate::W<REG> {
        self.variant(Timevselselect::Noon)
    }
}
#[doc = "Calendar Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Calevselselect {
    #[doc = "0: Week change (every Monday at time 00:00:00)"]
    Week = 0,
    #[doc = "1: Month change (every 01 of each month at time 00:00:00)"]
    Month = 1,
    #[doc = "2: Year change (every January 1 at time 00:00:00)"]
    Year = 2,
}
impl From<Calevselselect> for u8 {
    #[inline(always)]
    fn from(variant: Calevselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Calevselselect {
    type Ux = u8;
}
impl crate::IsEnum for Calevselselect {}
#[doc = "Field `CALEVSEL` reader - Calendar Event Selection"]
pub type CalevselR = crate::FieldReader<Calevselselect>;
impl CalevselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Calevselselect> {
        match self.bits {
            0 => Some(Calevselselect::Week),
            1 => Some(Calevselselect::Month),
            2 => Some(Calevselselect::Year),
            _ => None,
        }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == Calevselselect::Week
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == Calevselselect::Month
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == Calevselselect::Year
    }
}
#[doc = "Field `CALEVSEL` writer - Calendar Event Selection"]
pub type CalevselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Calevselselect>;
impl<'a, REG> CalevselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut crate::W<REG> {
        self.variant(Calevselselect::Week)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut crate::W<REG> {
        self.variant(Calevselselect::Month)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut crate::W<REG> {
        self.variant(Calevselselect::Year)
    }
}
impl R {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UpdtimR {
        UpdtimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UpdcalR {
        UpdcalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TimevselR {
        TimevselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CalevselR {
        CalevselR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&mut self) -> UpdtimW<CrSpec> {
        UpdtimW::new(self, 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&mut self) -> UpdcalW<CrSpec> {
        UpdcalW::new(self, 1)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&mut self) -> TimevselW<CrSpec> {
        TimevselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&mut self) -> CalevselW<CrSpec> {
        CalevselW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
