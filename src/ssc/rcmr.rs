#[doc = "Register `RCMR` reader"]
pub type R = crate::R<RcmrSpec>;
#[doc = "Register `RCMR` writer"]
pub type W = crate::W<RcmrSpec>;
#[doc = "Receive Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cksselect {
    #[doc = "0: Divided Clock"]
    Mck = 0,
    #[doc = "1: TK Clock signal"]
    Tk = 1,
    #[doc = "2: RK pin"]
    Rk = 2,
}
impl From<Cksselect> for u8 {
    #[inline(always)]
    fn from(variant: Cksselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cksselect {
    type Ux = u8;
}
impl crate::IsEnum for Cksselect {}
#[doc = "Field `CKS` reader - Receive Clock Selection"]
pub type CksR = crate::FieldReader<Cksselect>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cksselect> {
        match self.bits {
            0 => Some(Cksselect::Mck),
            1 => Some(Cksselect::Tk),
            2 => Some(Cksselect::Rk),
            _ => None,
        }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cksselect::Mck
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == Cksselect::Tk
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == Cksselect::Rk
    }
}
#[doc = "Field `CKS` writer - Receive Clock Selection"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cksselect>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Cksselect::Mck)
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut crate::W<REG> {
        self.variant(Cksselect::Tk)
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut crate::W<REG> {
        self.variant(Cksselect::Rk)
    }
}
#[doc = "Receive Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckoselect {
    #[doc = "0: None, RK pin is an input"]
    None = 0,
    #[doc = "1: Continuous Receive Clock, RK pin is an output"]
    Continuous = 1,
    #[doc = "2: Receive Clock only during data transfers, RK pin is an output"]
    Transfer = 2,
}
impl From<Ckoselect> for u8 {
    #[inline(always)]
    fn from(variant: Ckoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckoselect {
    type Ux = u8;
}
impl crate::IsEnum for Ckoselect {}
#[doc = "Field `CKO` reader - Receive Clock Output Mode Selection"]
pub type CkoR = crate::FieldReader<Ckoselect>;
impl CkoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckoselect> {
        match self.bits {
            0 => Some(Ckoselect::None),
            1 => Some(Ckoselect::Continuous),
            2 => Some(Ckoselect::Transfer),
            _ => None,
        }
    }
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ckoselect::None
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ckoselect::Continuous
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Ckoselect::Transfer
    }
}
#[doc = "Field `CKO` writer - Receive Clock Output Mode Selection"]
pub type CkoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckoselect>;
impl<'a, REG> CkoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoselect::None)
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoselect::Continuous)
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoselect::Transfer)
    }
}
#[doc = "Field `CKI` reader - Receive Clock Inversion"]
pub type CkiR = crate::BitReader;
#[doc = "Field `CKI` writer - Receive Clock Inversion"]
pub type CkiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckgselect {
    #[doc = "0: None"]
    Continuous = 0,
    #[doc = "1: Receive Clock enabled only if RF Low"]
    EnRfLow = 1,
    #[doc = "2: Receive Clock enabled only if RF High"]
    EnRfHigh = 2,
}
impl From<Ckgselect> for u8 {
    #[inline(always)]
    fn from(variant: Ckgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckgselect {
    type Ux = u8;
}
impl crate::IsEnum for Ckgselect {}
#[doc = "Field `CKG` reader - Receive Clock Gating Selection"]
pub type CkgR = crate::FieldReader<Ckgselect>;
impl CkgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckgselect> {
        match self.bits {
            0 => Some(Ckgselect::Continuous),
            1 => Some(Ckgselect::EnRfLow),
            2 => Some(Ckgselect::EnRfHigh),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ckgselect::Continuous
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn is_en_rf_low(&self) -> bool {
        *self == Ckgselect::EnRfLow
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn is_en_rf_high(&self) -> bool {
        *self == Ckgselect::EnRfHigh
    }
}
#[doc = "Field `CKG` writer - Receive Clock Gating Selection"]
pub type CkgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckgselect>;
impl<'a, REG> CkgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Ckgselect::Continuous)
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn en_rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ckgselect::EnRfLow)
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn en_rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ckgselect::EnRfHigh)
    }
}
#[doc = "Receive Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startselect {
    #[doc = "0: Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    Continuous = 0,
    #[doc = "1: Transmit start"]
    Transmit = 1,
    #[doc = "2: Detection of a low level on RF signal"]
    RfLow = 2,
    #[doc = "3: Detection of a high level on RF signal"]
    RfHigh = 3,
    #[doc = "4: Detection of a falling edge on RF signal"]
    RfFalling = 4,
    #[doc = "5: Detection of a rising edge on RF signal"]
    RfRising = 5,
    #[doc = "6: Detection of any level change on RF signal"]
    RfLevel = 6,
    #[doc = "7: Detection of any edge on RF signal"]
    RfEdge = 7,
    #[doc = "8: Compare 0"]
    Cmp0 = 8,
}
impl From<Startselect> for u8 {
    #[inline(always)]
    fn from(variant: Startselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startselect {
    type Ux = u8;
}
impl crate::IsEnum for Startselect {}
#[doc = "Field `START` reader - Receive Start Selection"]
pub type StartR = crate::FieldReader<Startselect>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startselect> {
        match self.bits {
            0 => Some(Startselect::Continuous),
            1 => Some(Startselect::Transmit),
            2 => Some(Startselect::RfLow),
            3 => Some(Startselect::RfHigh),
            4 => Some(Startselect::RfFalling),
            5 => Some(Startselect::RfRising),
            6 => Some(Startselect::RfLevel),
            7 => Some(Startselect::RfEdge),
            8 => Some(Startselect::Cmp0),
            _ => None,
        }
    }
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Startselect::Continuous
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == Startselect::Transmit
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn is_rf_low(&self) -> bool {
        *self == Startselect::RfLow
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn is_rf_high(&self) -> bool {
        *self == Startselect::RfHigh
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_falling(&self) -> bool {
        *self == Startselect::RfFalling
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_rising(&self) -> bool {
        *self == Startselect::RfRising
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn is_rf_level(&self) -> bool {
        *self == Startselect::RfLevel
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn is_rf_edge(&self) -> bool {
        *self == Startselect::RfEdge
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn is_cmp_0(&self) -> bool {
        *self == Startselect::Cmp0
    }
}
#[doc = "Field `START` writer - Receive Start Selection"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startselect>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Continuous)
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Transmit)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn rf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::RfLow)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn rf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::RfHigh)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn rf_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::RfFalling)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn rf_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::RfRising)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn rf_level(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::RfLevel)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn rf_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::RfEdge)
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn cmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Cmp0)
    }
}
#[doc = "Field `STOP` reader - Receive Stop Selection"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Receive Stop Selection"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTDLY` reader - Receive Start Delay"]
pub type SttdlyR = crate::FieldReader;
#[doc = "Field `STTDLY` writer - Receive Start Delay"]
pub type SttdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERIOD` reader - Receive Period Divider Selection"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Receive Period Divider Selection"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CkoR {
        CkoR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CkiR {
        CkiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CkgR {
        CkgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> SttdlyR {
        SttdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<RcmrSpec> {
        CksW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> CkoW<RcmrSpec> {
        CkoW::new(self, 2)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> CkiW<RcmrSpec> {
        CkiW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> CkgW<RcmrSpec> {
        CkgW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<RcmrSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<RcmrSpec> {
        StopW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> SttdlyW<RcmrSpec> {
        SttdlyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<RcmrSpec> {
        PeriodW::new(self, 24)
    }
}
#[doc = "Receive Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcmrSpec;
impl crate::RegisterSpec for RcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcmr::R`](R) reader structure"]
impl crate::Readable for RcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcmr::W`](W) writer structure"]
impl crate::Writable for RcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCMR to value 0"]
impl crate::Resettable for RcmrSpec {}
