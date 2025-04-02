#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Number of Column Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncselect {
    #[doc = "0: 8 column bits"]
    Col8 = 0,
    #[doc = "1: 9 column bits"]
    Col9 = 1,
    #[doc = "2: 10 column bits"]
    Col10 = 2,
    #[doc = "3: 11 column bits"]
    Col11 = 3,
}
impl From<Ncselect> for u8 {
    #[inline(always)]
    fn from(variant: Ncselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncselect {
    type Ux = u8;
}
impl crate::IsEnum for Ncselect {}
#[doc = "Field `NC` reader - Number of Column Bits"]
pub type NcR = crate::FieldReader<Ncselect>;
impl NcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncselect {
        match self.bits {
            0 => Ncselect::Col8,
            1 => Ncselect::Col9,
            2 => Ncselect::Col10,
            3 => Ncselect::Col11,
            _ => unreachable!(),
        }
    }
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn is_col8(&self) -> bool {
        *self == Ncselect::Col8
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn is_col9(&self) -> bool {
        *self == Ncselect::Col9
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn is_col10(&self) -> bool {
        *self == Ncselect::Col10
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn is_col11(&self) -> bool {
        *self == Ncselect::Col11
    }
}
#[doc = "Field `NC` writer - Number of Column Bits"]
pub type NcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ncselect, crate::Safe>;
impl<'a, REG> NcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn col8(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col8)
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn col9(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col9)
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn col10(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col10)
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn col11(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col11)
    }
}
#[doc = "Number of Row Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nrselect {
    #[doc = "0: 11 row bits"]
    Row11 = 0,
    #[doc = "1: 12 row bits"]
    Row12 = 1,
    #[doc = "2: 13 row bits"]
    Row13 = 2,
}
impl From<Nrselect> for u8 {
    #[inline(always)]
    fn from(variant: Nrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nrselect {
    type Ux = u8;
}
impl crate::IsEnum for Nrselect {}
#[doc = "Field `NR` reader - Number of Row Bits"]
pub type NrR = crate::FieldReader<Nrselect>;
impl NrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nrselect> {
        match self.bits {
            0 => Some(Nrselect::Row11),
            1 => Some(Nrselect::Row12),
            2 => Some(Nrselect::Row13),
            _ => None,
        }
    }
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn is_row11(&self) -> bool {
        *self == Nrselect::Row11
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn is_row12(&self) -> bool {
        *self == Nrselect::Row12
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn is_row13(&self) -> bool {
        *self == Nrselect::Row13
    }
}
#[doc = "Field `NR` writer - Number of Row Bits"]
pub type NrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nrselect>;
impl<'a, REG> NrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn row11(self) -> &'a mut crate::W<REG> {
        self.variant(Nrselect::Row11)
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn row12(self) -> &'a mut crate::W<REG> {
        self.variant(Nrselect::Row12)
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn row13(self) -> &'a mut crate::W<REG> {
        self.variant(Nrselect::Row13)
    }
}
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nbselect {
    #[doc = "0: 2 banks"]
    Bank2 = 0,
    #[doc = "1: 4 banks"]
    Bank4 = 1,
}
impl From<Nbselect> for bool {
    #[inline(always)]
    fn from(variant: Nbselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB` reader - Number of Banks"]
pub type NbR = crate::BitReader<Nbselect>;
impl NbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbselect {
        match self.bits {
            false => Nbselect::Bank2,
            true => Nbselect::Bank4,
        }
    }
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Nbselect::Bank2
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == Nbselect::Bank4
    }
}
#[doc = "Field `NB` writer - Number of Banks"]
pub type NbW<'a, REG> = crate::BitWriter<'a, REG, Nbselect>;
impl<'a, REG> NbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(Nbselect::Bank2)
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut crate::W<REG> {
        self.variant(Nbselect::Bank4)
    }
}
#[doc = "CAS Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Casselect {
    #[doc = "1: 1 cycle CAS latency"]
    Latency1 = 1,
    #[doc = "2: 2 cycle CAS latency"]
    Latency2 = 2,
    #[doc = "3: 3 cycle CAS latency"]
    Latency3 = 3,
}
impl From<Casselect> for u8 {
    #[inline(always)]
    fn from(variant: Casselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Casselect {
    type Ux = u8;
}
impl crate::IsEnum for Casselect {}
#[doc = "Field `CAS` reader - CAS Latency"]
pub type CasR = crate::FieldReader<Casselect>;
impl CasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Casselect> {
        match self.bits {
            1 => Some(Casselect::Latency1),
            2 => Some(Casselect::Latency2),
            3 => Some(Casselect::Latency3),
            _ => None,
        }
    }
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn is_latency1(&self) -> bool {
        *self == Casselect::Latency1
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn is_latency2(&self) -> bool {
        *self == Casselect::Latency2
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn is_latency3(&self) -> bool {
        *self == Casselect::Latency3
    }
}
#[doc = "Field `CAS` writer - CAS Latency"]
pub type CasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Casselect>;
impl<'a, REG> CasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn latency1(self) -> &'a mut crate::W<REG> {
        self.variant(Casselect::Latency1)
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn latency2(self) -> &'a mut crate::W<REG> {
        self.variant(Casselect::Latency2)
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn latency3(self) -> &'a mut crate::W<REG> {
        self.variant(Casselect::Latency3)
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::BitReader;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWR` reader - Write Recovery Delay"]
pub type TwrR = crate::FieldReader;
#[doc = "Field `TWR` writer - Write Recovery Delay"]
pub type TwrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRC_TRFC` reader - Row Cycle Delay and Row Refresh Cycle"]
pub type TrcTrfcR = crate::FieldReader;
#[doc = "Field `TRC_TRFC` writer - Row Cycle Delay and Row Refresh Cycle"]
pub type TrcTrfcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRP` reader - Row Precharge Delay"]
pub type TrpR = crate::FieldReader;
#[doc = "Field `TRP` writer - Row Precharge Delay"]
pub type TrpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCD` reader - Row to Column Delay"]
pub type TrcdR = crate::FieldReader;
#[doc = "Field `TRCD` writer - Row to Column Delay"]
pub type TrcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRAS` reader - Active to Precharge Delay"]
pub type TrasR = crate::FieldReader;
#[doc = "Field `TRAS` writer - Active to Precharge Delay"]
pub type TrasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSR` reader - Exit Self-Refresh to Active Delay"]
pub type TxsrR = crate::FieldReader;
#[doc = "Field `TXSR` writer - Exit Self-Refresh to Active Delay"]
pub type TxsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NcR {
        NcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NrR {
        NrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NbR {
        NbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TwrR {
        TwrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&self) -> TrcTrfcR {
        TrcTrfcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TrcdR {
        TrcdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TrasR {
        TrasR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Exit Self-Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TxsrR {
        TxsrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> NcW<CrSpec> {
        NcW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> NrW<CrSpec> {
        NrW::new(self, 2)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> NbW<CrSpec> {
        NbW::new(self, 4)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> CasW<CrSpec> {
        CasW::new(self, 5)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DbwW<CrSpec> {
        DbwW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> TwrW<CrSpec> {
        TwrW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&mut self) -> TrcTrfcW<CrSpec> {
        TrcTrfcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> TrpW<CrSpec> {
        TrpW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TrcdW<CrSpec> {
        TrcdW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&mut self) -> TrasW<CrSpec> {
        TrasW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Exit Self-Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&mut self) -> TxsrW<CrSpec> {
        TxsrW::new(self, 28)
    }
}
#[doc = "SDRAMC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
