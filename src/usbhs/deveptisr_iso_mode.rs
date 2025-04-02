#[doc = "Register `DEVEPTISR_ISO_MODE[%s]` reader"]
pub type R = crate::R<DeveptisrIsoModeSpec>;
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub type TxiniR = crate::BitReader;
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub type RxoutiR = crate::BitReader;
#[doc = "Field `UNDERFI` reader - Underflow Interrupt"]
pub type UnderfiR = crate::BitReader;
#[doc = "Field `HBISOINERRI` reader - High Bandwidth Isochronous IN Underflow Error Interrupt"]
pub type HbisoinerriR = crate::BitReader;
#[doc = "Field `HBISOFLUSHI` reader - High Bandwidth Isochronous IN Flush Interrupt"]
pub type HbisoflushiR = crate::BitReader;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OverfiR = crate::BitReader;
#[doc = "Field `CRCERRI` reader - CRC Error Interrupt"]
pub type CrcerriR = crate::BitReader;
#[doc = "Field `SHORTPACKET` reader - Short Packet Interrupt"]
pub type ShortpacketR = crate::BitReader;
#[doc = "Data Toggle Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtseqselect {
    #[doc = "0: Data0 toggle sequence"]
    Data0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    Data1 = 1,
    #[doc = "2: Reserved for high-bandwidth isochronous endpoint"]
    Data2 = 2,
    #[doc = "3: Reserved for high-bandwidth isochronous endpoint"]
    Mdata = 3,
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
    pub const fn variant(&self) -> Dtseqselect {
        match self.bits {
            0 => Dtseqselect::Data0,
            1 => Dtseqselect::Data1,
            2 => Dtseqselect::Data2,
            3 => Dtseqselect::Mdata,
            _ => unreachable!(),
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
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == Dtseqselect::Data2
    }
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == Dtseqselect::Mdata
    }
}
#[doc = "Field `ERRORTRANS` reader - High-bandwidth Isochronous OUT Endpoint Transaction Error Interrupt"]
pub type ErrortransR = crate::BitReader;
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
#[doc = "Field `BYCT` reader - Byte Count"]
pub type ByctR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TxiniR {
        TxiniR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RxoutiR {
        RxoutiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfi(&self) -> UnderfiR {
        UnderfiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt"]
    #[inline(always)]
    pub fn hbisoinerri(&self) -> HbisoinerriR {
        HbisoinerriR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt"]
    #[inline(always)]
    pub fn hbisoflushi(&self) -> HbisoflushiR {
        HbisoflushiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OverfiR {
        OverfiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerri(&self) -> CrcerriR {
        CrcerriR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacket(&self) -> ShortpacketR {
        ShortpacketR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DtseqR {
        DtseqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - High-bandwidth Isochronous OUT Endpoint Transaction Error Interrupt"]
    #[inline(always)]
    pub fn errortrans(&self) -> ErrortransR {
        ErrortransR::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline(always)]
    pub fn byct(&self) -> ByctR {
        ByctR::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr_iso_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptisrIsoModeSpec;
impl crate::RegisterSpec for DeveptisrIsoModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptisr_iso_mode::R`](R) reader structure"]
impl crate::Readable for DeveptisrIsoModeSpec {}
#[doc = "`reset()` method sets DEVEPTISR_ISO_MODE[%s] to value 0"]
impl crate::Resettable for DeveptisrIsoModeSpec {}
