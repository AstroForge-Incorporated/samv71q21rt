#[doc = "Register `TCMR` reader"]
pub type R = crate::R<TcmrSpec>;
#[doc = "Register `TCMR` writer"]
pub type W = crate::W<TcmrSpec>;
#[doc = "Transmit Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cksselect {
    #[doc = "0: Divided Clock"]
    Mck = 0,
    #[doc = "1: RK Clock signal"]
    Rk = 1,
    #[doc = "2: TK pin"]
    Tk = 2,
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
#[doc = "Field `CKS` reader - Transmit Clock Selection"]
pub type CksR = crate::FieldReader<Cksselect>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cksselect> {
        match self.bits {
            0 => Some(Cksselect::Mck),
            1 => Some(Cksselect::Rk),
            2 => Some(Cksselect::Tk),
            _ => None,
        }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cksselect::Mck
    }
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == Cksselect::Rk
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == Cksselect::Tk
    }
}
#[doc = "Field `CKS` writer - Transmit Clock Selection"]
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
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut crate::W<REG> {
        self.variant(Cksselect::Rk)
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut crate::W<REG> {
        self.variant(Cksselect::Tk)
    }
}
#[doc = "Transmit Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckoselect {
    #[doc = "0: None, TK pin is an input"]
    None = 0,
    #[doc = "1: Continuous Transmit Clock, TK pin is an output"]
    Continuous = 1,
    #[doc = "2: Transmit Clock only during data transfers, TK pin is an output"]
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
#[doc = "Field `CKO` reader - Transmit Clock Output Mode Selection"]
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
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ckoselect::None
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ckoselect::Continuous
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Ckoselect::Transfer
    }
}
#[doc = "Field `CKO` writer - Transmit Clock Output Mode Selection"]
pub type CkoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckoselect>;
impl<'a, REG> CkoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoselect::None)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoselect::Continuous)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoselect::Transfer)
    }
}
#[doc = "Field `CKI` reader - Transmit Clock Inversion"]
pub type CkiR = crate::BitReader;
#[doc = "Field `CKI` writer - Transmit Clock Inversion"]
pub type CkiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckgselect {
    #[doc = "0: None"]
    Continuous = 0,
    #[doc = "1: Transmit Clock enabled only if TF Low"]
    EnTfLow = 1,
    #[doc = "2: Transmit Clock enabled only if TF High"]
    EnTfHigh = 2,
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
#[doc = "Field `CKG` reader - Transmit Clock Gating Selection"]
pub type CkgR = crate::FieldReader<Ckgselect>;
impl CkgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckgselect> {
        match self.bits {
            0 => Some(Ckgselect::Continuous),
            1 => Some(Ckgselect::EnTfLow),
            2 => Some(Ckgselect::EnTfHigh),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Ckgselect::Continuous
    }
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn is_en_tf_low(&self) -> bool {
        *self == Ckgselect::EnTfLow
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn is_en_tf_high(&self) -> bool {
        *self == Ckgselect::EnTfHigh
    }
}
#[doc = "Field `CKG` writer - Transmit Clock Gating Selection"]
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
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn en_tf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ckgselect::EnTfLow)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn en_tf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ckgselect::EnTfHigh)
    }
}
#[doc = "Transmit Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startselect {
    #[doc = "0: Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    Continuous = 0,
    #[doc = "1: Receive start"]
    Receive = 1,
    #[doc = "2: Detection of a low level on TF signal"]
    TfLow = 2,
    #[doc = "3: Detection of a high level on TF signal"]
    TfHigh = 3,
    #[doc = "4: Detection of a falling edge on TF signal"]
    TfFalling = 4,
    #[doc = "5: Detection of a rising edge on TF signal"]
    TfRising = 5,
    #[doc = "6: Detection of any level change on TF signal"]
    TfLevel = 6,
    #[doc = "7: Detection of any edge on TF signal"]
    TfEdge = 7,
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
#[doc = "Field `START` reader - Transmit Start Selection"]
pub type StartR = crate::FieldReader<Startselect>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startselect> {
        match self.bits {
            0 => Some(Startselect::Continuous),
            1 => Some(Startselect::Receive),
            2 => Some(Startselect::TfLow),
            3 => Some(Startselect::TfHigh),
            4 => Some(Startselect::TfFalling),
            5 => Some(Startselect::TfRising),
            6 => Some(Startselect::TfLevel),
            7 => Some(Startselect::TfEdge),
            _ => None,
        }
    }
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Startselect::Continuous
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == Startselect::Receive
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn is_tf_low(&self) -> bool {
        *self == Startselect::TfLow
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn is_tf_high(&self) -> bool {
        *self == Startselect::TfHigh
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn is_tf_falling(&self) -> bool {
        *self == Startselect::TfFalling
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn is_tf_rising(&self) -> bool {
        *self == Startselect::TfRising
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn is_tf_level(&self) -> bool {
        *self == Startselect::TfLevel
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn is_tf_edge(&self) -> bool {
        *self == Startselect::TfEdge
    }
}
#[doc = "Field `START` writer - Transmit Start Selection"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startselect>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Continuous)
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Receive)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn tf_low(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::TfLow)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn tf_high(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::TfHigh)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn tf_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::TfFalling)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn tf_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::TfRising)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn tf_level(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::TfLevel)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn tf_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::TfEdge)
    }
}
#[doc = "Field `STTDLY` reader - Transmit Start Delay"]
pub type SttdlyR = crate::FieldReader;
#[doc = "Field `STTDLY` writer - Transmit Start Delay"]
pub type SttdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERIOD` reader - Transmit Period Divider Selection"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - Transmit Period Divider Selection"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CkoR {
        CkoR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CkiR {
        CkiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CkgR {
        CkgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> SttdlyR {
        SttdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<TcmrSpec> {
        CksW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> CkoW<TcmrSpec> {
        CkoW::new(self, 2)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> CkiW<TcmrSpec> {
        CkiW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> CkgW<TcmrSpec> {
        CkgW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<TcmrSpec> {
        StartW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> SttdlyW<TcmrSpec> {
        SttdlyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<TcmrSpec> {
        PeriodW::new(self, 24)
    }
}
#[doc = "Transmit Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmrSpec;
impl crate::RegisterSpec for TcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcmr::R`](R) reader structure"]
impl crate::Readable for TcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcmr::W`](W) writer structure"]
impl crate::Writable for TcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TcmrSpec {}
