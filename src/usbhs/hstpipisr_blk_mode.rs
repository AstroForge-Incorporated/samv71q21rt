#[doc = "Register `HSTPIPISR_BLK_MODE[%s]` reader"]
pub type R = crate::R<HstpipisrBlkModeSpec>;
#[doc = "Field `RXINI` reader - Received IN Data Interrupt"]
pub type RxiniR = crate::BitReader;
#[doc = "Field `TXOUTI` reader - Transmitted OUT Data Interrupt"]
pub type TxoutiR = crate::BitReader;
#[doc = "Field `TXSTPI` reader - Transmitted SETUP Interrupt"]
pub type TxstpiR = crate::BitReader;
#[doc = "Field `PERRI` reader - Pipe Error Interrupt"]
pub type PerriR = crate::BitReader;
#[doc = "Field `NAKEDI` reader - NAKed Interrupt"]
pub type NakediR = crate::BitReader;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OverfiR = crate::BitReader;
#[doc = "Field `RXSTALLDI` reader - Received STALLed Interrupt"]
pub type RxstalldiR = crate::BitReader;
#[doc = "Field `SHORTPACKETI` reader - Short Packet Interrupt"]
pub type ShortpacketiR = crate::BitReader;
#[doc = "Data Toggle Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtseqselect {
    #[doc = "0: Data0 toggle sequence"]
    Data0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    Data1 = 1,
}
impl From<Dtseqselect> for u8 {
    #[inline(always)]
    fn from(variant: Dtseqselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtseqselect {
    type Ux = u8;
}
impl crate::IsEnum for Dtseqselect {}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DtseqR = crate::FieldReader<Dtseqselect>;
impl DtseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtseqselect> {
        match self.bits {
            0 => Some(Dtseqselect::Data0),
            1 => Some(Dtseqselect::Data1),
            _ => None,
        }
    }
    #[doc = "Data0 toggle sequence"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Dtseqselect::Data0
    }
    #[doc = "Data1 toggle sequence"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Dtseqselect::Data1
    }
}
#[doc = "Number of Busy Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbusybkselect {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0Busy = 0,
    #[doc = "1: 1 busy bank"]
    _1Busy = 1,
    #[doc = "2: 2 busy banks"]
    _2Busy = 2,
    #[doc = "3: 3 busy banks"]
    _3Busy = 3,
}
impl From<Nbusybkselect> for u8 {
    #[inline(always)]
    fn from(variant: Nbusybkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbusybkselect {
    type Ux = u8;
}
impl crate::IsEnum for Nbusybkselect {}
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub type NbusybkR = crate::FieldReader<Nbusybkselect>;
impl NbusybkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbusybkselect {
        match self.bits {
            0 => Nbusybkselect::_0Busy,
            1 => Nbusybkselect::_1Busy,
            2 => Nbusybkselect::_2Busy,
            3 => Nbusybkselect::_3Busy,
            _ => unreachable!(),
        }
    }
    #[doc = "0 busy bank (all banks free)"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        *self == Nbusybkselect::_0Busy
    }
    #[doc = "1 busy bank"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        *self == Nbusybkselect::_1Busy
    }
    #[doc = "2 busy banks"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        *self == Nbusybkselect::_2Busy
    }
    #[doc = "3 busy banks"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        *self == Nbusybkselect::_3Busy
    }
}
#[doc = "Current Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Currbkselect {
    #[doc = "0: Current bank is bank0"]
    Bank0 = 0,
    #[doc = "1: Current bank is bank1"]
    Bank1 = 1,
    #[doc = "2: Current bank is bank2"]
    Bank2 = 2,
}
impl From<Currbkselect> for u8 {
    #[inline(always)]
    fn from(variant: Currbkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Currbkselect {
    type Ux = u8;
}
impl crate::IsEnum for Currbkselect {}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CurrbkR = crate::FieldReader<Currbkselect>;
impl CurrbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Currbkselect> {
        match self.bits {
            0 => Some(Currbkselect::Bank0),
            1 => Some(Currbkselect::Bank1),
            2 => Some(Currbkselect::Bank2),
            _ => None,
        }
    }
    #[doc = "Current bank is bank0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Currbkselect::Bank0
    }
    #[doc = "Current bank is bank1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Currbkselect::Bank1
    }
    #[doc = "Current bank is bank2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Currbkselect::Bank2
    }
}
#[doc = "Field `RWALL` reader - Read/Write Allowed"]
pub type RwallR = crate::BitReader;
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub type CfgokR = crate::BitReader;
#[doc = "Field `PBYCT` reader - Pipe Byte Count"]
pub type PbyctR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt"]
    #[inline(always)]
    pub fn rxini(&self) -> RxiniR {
        RxiniR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt"]
    #[inline(always)]
    pub fn txouti(&self) -> TxoutiR {
        TxoutiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt"]
    #[inline(always)]
    pub fn txstpi(&self) -> TxstpiR {
        TxstpiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt"]
    #[inline(always)]
    pub fn perri(&self) -> PerriR {
        PerriR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt"]
    #[inline(always)]
    pub fn nakedi(&self) -> NakediR {
        NakediR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OverfiR {
        OverfiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt"]
    #[inline(always)]
    pub fn rxstalldi(&self) -> RxstalldiR {
        RxstalldiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacketi(&self) -> ShortpacketiR {
        ShortpacketiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DtseqR {
        DtseqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NbusybkR {
        NbusybkR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CurrbkR {
        CurrbkR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RwallR {
        RwallR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CfgokR {
        CfgokR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Pipe Byte Count"]
    #[inline(always)]
    pub fn pbyct(&self) -> PbyctR {
        PbyctR::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Host Pipe Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipisr_blk_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipisrBlkModeSpec;
impl crate::RegisterSpec for HstpipisrBlkModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipisr_blk_mode::R`](R) reader structure"]
impl crate::Readable for HstpipisrBlkModeSpec {}
#[doc = "`reset()` method sets HSTPIPISR_BLK_MODE[%s] to value 0"]
impl crate::Resettable for HstpipisrBlkModeSpec {}
