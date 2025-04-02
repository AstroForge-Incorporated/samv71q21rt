#[doc = "Register `ETRG1` reader"]
pub type R = crate::R<Etrg1Spec>;
#[doc = "Register `ETRG1` writer"]
pub type W = crate::W<Etrg1Spec>;
#[doc = "Field `MAXCNT` reader - Maximum Counter value"]
pub type MaxcntR = crate::FieldReader<u32>;
#[doc = "Field `MAXCNT` writer - Maximum Counter value"]
pub type MaxcntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "External Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgmodeselect {
    #[doc = "0: External trigger is not enabled."]
    Off = 0,
    #[doc = "1: External PWM Reset Mode"]
    Mode1 = 1,
    #[doc = "2: External PWM Start Mode"]
    Mode2 = 2,
    #[doc = "3: Cycle-by-cycle Duty Mode"]
    Mode3 = 3,
}
impl From<Trgmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Trgmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Trgmodeselect {}
#[doc = "Field `TRGMODE` reader - External Trigger Mode"]
pub type TrgmodeR = crate::FieldReader<Trgmodeselect>;
impl TrgmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgmodeselect {
        match self.bits {
            0 => Trgmodeselect::Off,
            1 => Trgmodeselect::Mode1,
            2 => Trgmodeselect::Mode2,
            3 => Trgmodeselect::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Trgmodeselect::Off
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Trgmodeselect::Mode1
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Trgmodeselect::Mode2
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Trgmodeselect::Mode3
    }
}
#[doc = "Field `TRGMODE` writer - External Trigger Mode"]
pub type TrgmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trgmodeselect, crate::Safe>;
impl<'a, REG> TrgmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Trgmodeselect::Off)
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgmodeselect::Mode1)
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgmodeselect::Mode2)
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgmodeselect::Mode3)
    }
}
#[doc = "Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgedgeselect {
    #[doc = "0: TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    FallingZero = 0,
    #[doc = "1: TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    RisingOne = 1,
}
impl From<Trgedgeselect> for bool {
    #[inline(always)]
    fn from(variant: Trgedgeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGEDGE` reader - Edge Selection"]
pub type TrgedgeR = crate::BitReader<Trgedgeselect>;
impl TrgedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgedgeselect {
        match self.bits {
            false => Trgedgeselect::FallingZero,
            true => Trgedgeselect::RisingOne,
        }
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn is_falling_zero(&self) -> bool {
        *self == Trgedgeselect::FallingZero
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn is_rising_one(&self) -> bool {
        *self == Trgedgeselect::RisingOne
    }
}
#[doc = "Field `TRGEDGE` writer - Edge Selection"]
pub type TrgedgeW<'a, REG> = crate::BitWriter<'a, REG, Trgedgeselect>;
impl<'a, REG> TrgedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn falling_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Trgedgeselect::FallingZero)
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn rising_one(self) -> &'a mut crate::W<REG> {
        self.variant(Trgedgeselect::RisingOne)
    }
}
#[doc = "Field `TRGFILT` reader - Filtered input"]
pub type TrgfiltR = crate::BitReader;
#[doc = "Field `TRGFILT` writer - Filtered input"]
pub type TrgfiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub type TrgsrcR = crate::BitReader;
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub type TrgsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFEN` reader - Recoverable Fault Enable"]
pub type RfenR = crate::BitReader;
#[doc = "Field `RFEN` writer - Recoverable Fault Enable"]
pub type RfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MaxcntR {
        MaxcntR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&self) -> TrgmodeR {
        TrgmodeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&self) -> TrgedgeR {
        TrgedgeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&self) -> TrgfiltR {
        TrgfiltR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TrgsrcR {
        TrgsrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RfenR {
        RfenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MaxcntW<Etrg1Spec> {
        MaxcntW::new(self, 0)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&mut self) -> TrgmodeW<Etrg1Spec> {
        TrgmodeW::new(self, 24)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&mut self) -> TrgedgeW<Etrg1Spec> {
        TrgedgeW::new(self, 28)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&mut self) -> TrgfiltW<Etrg1Spec> {
        TrgfiltW::new(self, 29)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TrgsrcW<Etrg1Spec> {
        TrgsrcW::new(self, 30)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RfenW<Etrg1Spec> {
        RfenW::new(self, 31)
    }
}
#[doc = "PWM External Trigger Register (trg_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`etrg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etrg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etrg1Spec;
impl crate::RegisterSpec for Etrg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etrg1::R`](R) reader structure"]
impl crate::Readable for Etrg1Spec {}
#[doc = "`write(|w| ..)` method takes [`etrg1::W`](W) writer structure"]
impl crate::Writable for Etrg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETRG1 to value 0"]
impl crate::Resettable for Etrg1Spec {}
