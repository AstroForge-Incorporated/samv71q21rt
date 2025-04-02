#[doc = "Register `TEMPMR` reader"]
pub type R = crate::R<TempmrSpec>;
#[doc = "Register `TEMPMR` writer"]
pub type W = crate::W<TempmrSpec>;
#[doc = "Field `RTCT` reader - Temperature Sensor RTC Trigger Mode"]
pub type RtctR = crate::BitReader;
#[doc = "Field `RTCT` writer - Temperature Sensor RTC Trigger Mode"]
pub type RtctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Temperature Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tempcmpmodselect {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    Low = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    High = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    In = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    Out = 3,
}
impl From<Tempcmpmodselect> for u8 {
    #[inline(always)]
    fn from(variant: Tempcmpmodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tempcmpmodselect {
    type Ux = u8;
}
impl crate::IsEnum for Tempcmpmodselect {}
#[doc = "Field `TEMPCMPMOD` reader - Temperature Comparison Mode"]
pub type TempcmpmodR = crate::FieldReader<Tempcmpmodselect>;
impl TempcmpmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tempcmpmodselect {
        match self.bits {
            0 => Tempcmpmodselect::Low,
            1 => Tempcmpmodselect::High,
            2 => Tempcmpmodselect::In,
            3 => Tempcmpmodselect::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Tempcmpmodselect::Low
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Tempcmpmodselect::High
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Tempcmpmodselect::In
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Tempcmpmodselect::Out
    }
}
#[doc = "Field `TEMPCMPMOD` writer - Temperature Comparison Mode"]
pub type TempcmpmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tempcmpmodselect, crate::Safe>;
impl<'a, REG> TempcmpmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Tempcmpmodselect::Low)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Tempcmpmodselect::High)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Tempcmpmodselect::In)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Tempcmpmodselect::Out)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&self) -> RtctR {
        RtctR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&self) -> TempcmpmodR {
        TempcmpmodR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&mut self) -> RtctW<TempmrSpec> {
        RtctW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&mut self) -> TempcmpmodW<TempmrSpec> {
        TempcmpmodW::new(self, 4)
    }
}
#[doc = "AFEC Temperature Sensor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tempmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tempmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempmrSpec;
impl crate::RegisterSpec for TempmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempmr::R`](R) reader structure"]
impl crate::Readable for TempmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tempmr::W`](W) writer structure"]
impl crate::Writable for TempmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMPMR to value 0"]
impl crate::Resettable for TempmrSpec {}
