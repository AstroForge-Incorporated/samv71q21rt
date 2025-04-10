#[doc = "Register `HSTPIPCFG_CTRL_BULK_MODE[%s]` reader"]
pub type R = crate::R<HstpipcfgCtrlBulkModeSpec>;
#[doc = "Register `HSTPIPCFG_CTRL_BULK_MODE[%s]` writer"]
pub type W = crate::W<HstpipcfgCtrlBulkModeSpec>;
#[doc = "Field `ALLOC` reader - Pipe Memory Allocate"]
pub type AllocR = crate::BitReader;
#[doc = "Field `ALLOC` writer - Pipe Memory Allocate"]
pub type AllocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pipe Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbkselect {
    #[doc = "0: Single-bank pipe"]
    _1Bank = 0,
    #[doc = "1: Double-bank pipe"]
    _2Bank = 1,
    #[doc = "2: Triple-bank pipe"]
    _3Bank = 2,
}
impl From<Pbkselect> for u8 {
    #[inline(always)]
    fn from(variant: Pbkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pbkselect {
    type Ux = u8;
}
impl crate::IsEnum for Pbkselect {}
#[doc = "Field `PBK` reader - Pipe Banks"]
pub type PbkR = crate::FieldReader<Pbkselect>;
impl PbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pbkselect> {
        match self.bits {
            0 => Some(Pbkselect::_1Bank),
            1 => Some(Pbkselect::_2Bank),
            2 => Some(Pbkselect::_3Bank),
            _ => None,
        }
    }
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == Pbkselect::_1Bank
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == Pbkselect::_2Bank
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == Pbkselect::_3Bank
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub type PbkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pbkselect>;
impl<'a, REG> PbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Pbkselect::_1Bank)
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Pbkselect::_2Bank)
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Pbkselect::_3Bank)
    }
}
#[doc = "Pipe Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psizeselect {
    #[doc = "0: 8 bytes"]
    _8Byte = 0,
    #[doc = "1: 16 bytes"]
    _16Byte = 1,
    #[doc = "2: 32 bytes"]
    _32Byte = 2,
    #[doc = "3: 64 bytes"]
    _64Byte = 3,
    #[doc = "4: 128 bytes"]
    _128Byte = 4,
    #[doc = "5: 256 bytes"]
    _256Byte = 5,
    #[doc = "6: 512 bytes"]
    _512Byte = 6,
    #[doc = "7: 1024 bytes"]
    _1024Byte = 7,
}
impl From<Psizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Psizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Psizeselect {}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub type PsizeR = crate::FieldReader<Psizeselect>;
impl PsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psizeselect {
        match self.bits {
            0 => Psizeselect::_8Byte,
            1 => Psizeselect::_16Byte,
            2 => Psizeselect::_32Byte,
            3 => Psizeselect::_64Byte,
            4 => Psizeselect::_128Byte,
            5 => Psizeselect::_256Byte,
            6 => Psizeselect::_512Byte,
            7 => Psizeselect::_1024Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Psizeselect::_8Byte
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Psizeselect::_16Byte
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Psizeselect::_32Byte
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == Psizeselect::_64Byte
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == Psizeselect::_128Byte
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == Psizeselect::_256Byte
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == Psizeselect::_512Byte
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == Psizeselect::_1024Byte
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Psizeselect, crate::Safe>;
impl<'a, REG> PsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_8Byte)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_16Byte)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_32Byte)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_64Byte)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_128Byte)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_256Byte)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_512Byte)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psizeselect::_1024Byte)
    }
}
#[doc = "Pipe Token\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ptokenselect {
    #[doc = "0: SETUP"]
    Setup = 0,
    #[doc = "1: IN"]
    In = 1,
    #[doc = "2: OUT"]
    Out = 2,
}
impl From<Ptokenselect> for u8 {
    #[inline(always)]
    fn from(variant: Ptokenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptokenselect {
    type Ux = u8;
}
impl crate::IsEnum for Ptokenselect {}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PtokenR = crate::FieldReader<Ptokenselect>;
impl PtokenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ptokenselect> {
        match self.bits {
            0 => Some(Ptokenselect::Setup),
            1 => Some(Ptokenselect::In),
            2 => Some(Ptokenselect::Out),
            _ => None,
        }
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == Ptokenselect::Setup
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Ptokenselect::In
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Ptokenselect::Out
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PtokenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ptokenselect>;
impl<'a, REG> PtokenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut crate::W<REG> {
        self.variant(Ptokenselect::Setup)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Ptokenselect::In)
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Ptokenselect::Out)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AutoswR = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AutoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pipe Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ptypeselect {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous"]
    Iso = 1,
    #[doc = "2: Bulk"]
    Blk = 2,
    #[doc = "3: Interrupt"]
    Intrpt = 3,
}
impl From<Ptypeselect> for u8 {
    #[inline(always)]
    fn from(variant: Ptypeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptypeselect {
    type Ux = u8;
}
impl crate::IsEnum for Ptypeselect {}
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PtypeR = crate::FieldReader<Ptypeselect>;
impl PtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptypeselect {
        match self.bits {
            0 => Ptypeselect::Ctrl,
            1 => Ptypeselect::Iso,
            2 => Ptypeselect::Blk,
            3 => Ptypeselect::Intrpt,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == Ptypeselect::Ctrl
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Ptypeselect::Iso
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == Ptypeselect::Blk
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == Ptypeselect::Intrpt
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ptypeselect, crate::Safe>;
impl<'a, REG> PtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Ptypeselect::Ctrl)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Ptypeselect::Iso)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(Ptypeselect::Blk)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut crate::W<REG> {
        self.variant(Ptypeselect::Intrpt)
    }
}
#[doc = "Field `PEPNUM` reader - Pipe Endpoint Number"]
pub type PepnumR = crate::FieldReader;
#[doc = "Field `PEPNUM` writer - Pipe Endpoint Number"]
pub type PepnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PINGEN` reader - Ping Enable"]
pub type PingenR = crate::BitReader;
#[doc = "Field `PINGEN` writer - Ping Enable"]
pub type PingenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BINTERVAL` reader - bInterval Parameter for the Bulk-Out/Ping Transaction"]
pub type BintervalR = crate::FieldReader;
#[doc = "Field `BINTERVAL` writer - bInterval Parameter for the Bulk-Out/Ping Transaction"]
pub type BintervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> AllocR {
        AllocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PbkR {
        PbkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PtokenR {
        PtokenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AutoswR {
        AutoswR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PtypeR {
        PtypeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&self) -> PepnumR {
        PepnumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&self) -> PingenR {
        PingenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - bInterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&self) -> BintervalR {
        BintervalR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&mut self) -> AllocW<HstpipcfgCtrlBulkModeSpec> {
        AllocW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&mut self) -> PbkW<HstpipcfgCtrlBulkModeSpec> {
        PbkW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PsizeW<HstpipcfgCtrlBulkModeSpec> {
        PsizeW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PtokenW<HstpipcfgCtrlBulkModeSpec> {
        PtokenW::new(self, 8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&mut self) -> AutoswW<HstpipcfgCtrlBulkModeSpec> {
        AutoswW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PtypeW<HstpipcfgCtrlBulkModeSpec> {
        PtypeW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&mut self) -> PepnumW<HstpipcfgCtrlBulkModeSpec> {
        PepnumW::new(self, 16)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&mut self) -> PingenW<HstpipcfgCtrlBulkModeSpec> {
        PingenW::new(self, 20)
    }
    #[doc = "Bits 24:31 - bInterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&mut self) -> BintervalW<HstpipcfgCtrlBulkModeSpec> {
        BintervalW::new(self, 24)
    }
}
#[doc = "Host Pipe Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipcfg_ctrl_bulk_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipcfg_ctrl_bulk_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstpipcfgCtrlBulkModeSpec;
impl crate::RegisterSpec for HstpipcfgCtrlBulkModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipcfg_ctrl_bulk_mode::R`](R) reader structure"]
impl crate::Readable for HstpipcfgCtrlBulkModeSpec {}
#[doc = "`write(|w| ..)` method takes [`hstpipcfg_ctrl_bulk_mode::W`](W) writer structure"]
impl crate::Writable for HstpipcfgCtrlBulkModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPIPCFG_CTRL_BULK_MODE[%s] to value 0"]
impl crate::Resettable for HstpipcfgCtrlBulkModeSpec {}
