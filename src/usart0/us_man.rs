#[doc = "Register `US_MAN` reader"]
pub type R = crate::R<UsManSpec>;
#[doc = "Register `US_MAN` writer"]
pub type W = crate::W<UsManSpec>;
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub type TxPlR = crate::FieldReader;
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub type TxPlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxPpselect {
    #[doc = "0: The preamble is composed of '1's"]
    AllOne = 0,
    #[doc = "1: The preamble is composed of '0's"]
    AllZero = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZeroOne = 2,
    #[doc = "3: The preamble is composed of '10's"]
    OneZero = 3,
}
impl From<TxPpselect> for u8 {
    #[inline(always)]
    fn from(variant: TxPpselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxPpselect {
    type Ux = u8;
}
impl crate::IsEnum for TxPpselect {}
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub type TxPpR = crate::FieldReader<TxPpselect>;
impl TxPpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxPpselect {
        match self.bits {
            0 => TxPpselect::AllOne,
            1 => TxPpselect::AllZero,
            2 => TxPpselect::ZeroOne,
            3 => TxPpselect::OneZero,
            _ => unreachable!(),
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == TxPpselect::AllOne
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == TxPpselect::AllZero
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == TxPpselect::ZeroOne
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == TxPpselect::OneZero
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub type TxPpW<'a, REG> = crate::FieldWriter<'a, REG, 2, TxPpselect, crate::Safe>;
impl<'a, REG> TxPpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut crate::W<REG> {
        self.variant(TxPpselect::AllOne)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TxPpselect::AllZero)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut crate::W<REG> {
        self.variant(TxPpselect::ZeroOne)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TxPpselect::OneZero)
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub type TxMpolR = crate::BitReader;
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub type TxMpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub type RxPlR = crate::FieldReader;
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub type RxPlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxPpselect {
    #[doc = "0: The preamble is composed of '1's"]
    AllOne = 0,
    #[doc = "1: The preamble is composed of '0's"]
    AllZero = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZeroOne = 2,
    #[doc = "3: The preamble is composed of '10's"]
    OneZero = 3,
}
impl From<RxPpselect> for u8 {
    #[inline(always)]
    fn from(variant: RxPpselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxPpselect {
    type Ux = u8;
}
impl crate::IsEnum for RxPpselect {}
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub type RxPpR = crate::FieldReader<RxPpselect>;
impl RxPpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxPpselect {
        match self.bits {
            0 => RxPpselect::AllOne,
            1 => RxPpselect::AllZero,
            2 => RxPpselect::ZeroOne,
            3 => RxPpselect::OneZero,
            _ => unreachable!(),
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == RxPpselect::AllOne
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == RxPpselect::AllZero
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == RxPpselect::ZeroOne
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == RxPpselect::OneZero
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub type RxPpW<'a, REG> = crate::FieldWriter<'a, REG, 2, RxPpselect, crate::Safe>;
impl<'a, REG> RxPpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut crate::W<REG> {
        self.variant(RxPpselect::AllOne)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut crate::W<REG> {
        self.variant(RxPpselect::AllZero)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut crate::W<REG> {
        self.variant(RxPpselect::ZeroOne)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut crate::W<REG> {
        self.variant(RxPpselect::OneZero)
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub type RxMpolR = crate::BitReader;
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub type RxMpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIFT` reader - Drift Compensation"]
pub type DriftR = crate::BitReader;
#[doc = "Field `DRIFT` writer - Drift Compensation"]
pub type DriftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIDLEV` reader - Receiver Idle Value"]
pub type RxidlevR = crate::BitReader;
#[doc = "Field `RXIDLEV` writer - Receiver Idle Value"]
pub type RxidlevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TxPlR {
        TxPlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TxPpR {
        TxPpR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TxMpolR {
        TxMpolR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RxPlR {
        RxPlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RxPpR {
        RxPpR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RxMpolR {
        RxMpolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DriftR {
        DriftR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&self) -> RxidlevR {
        RxidlevR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&mut self) -> TxPlW<UsManSpec> {
        TxPlW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&mut self) -> TxPpW<UsManSpec> {
        TxPpW::new(self, 8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&mut self) -> TxMpolW<UsManSpec> {
        TxMpolW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&mut self) -> RxPlW<UsManSpec> {
        RxPlW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&mut self) -> RxPpW<UsManSpec> {
        RxPpW::new(self, 24)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&mut self) -> RxMpolW<UsManSpec> {
        RxMpolW::new(self, 28)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> OneW<UsManSpec> {
        OneW::new(self, 29)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&mut self) -> DriftW<UsManSpec> {
        DriftW::new(self, 30)
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&mut self) -> RxidlevW<UsManSpec> {
        RxidlevW::new(self, 31)
    }
}
#[doc = "Manchester Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`us_man::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`us_man::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsManSpec;
impl crate::RegisterSpec for UsManSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_man::R`](R) reader structure"]
impl crate::Readable for UsManSpec {}
#[doc = "`write(|w| ..)` method takes [`us_man::W`](W) writer structure"]
impl crate::Writable for UsManSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets US_MAN to value 0"]
impl crate::Resettable for UsManSpec {}
