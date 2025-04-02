#[doc = "Register `DEVEPTCFG[%s]` reader"]
pub type R = crate::R<DeveptcfgSpec>;
#[doc = "Register `DEVEPTCFG[%s]` writer"]
pub type W = crate::W<DeveptcfgSpec>;
#[doc = "Field `ALLOC` reader - Endpoint Memory Allocate"]
pub type AllocR = crate::BitReader;
#[doc = "Field `ALLOC` writer - Endpoint Memory Allocate"]
pub type AllocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Epbkselect {
    #[doc = "0: Single-bank endpoint"]
    _1Bank = 0,
    #[doc = "1: Double-bank endpoint"]
    _2Bank = 1,
    #[doc = "2: Triple-bank endpoint"]
    _3Bank = 2,
}
impl From<Epbkselect> for u8 {
    #[inline(always)]
    fn from(variant: Epbkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Epbkselect {
    type Ux = u8;
}
impl crate::IsEnum for Epbkselect {}
#[doc = "Field `EPBK` reader - Endpoint Banks"]
pub type EpbkR = crate::FieldReader<Epbkselect>;
impl EpbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Epbkselect> {
        match self.bits {
            0 => Some(Epbkselect::_1Bank),
            1 => Some(Epbkselect::_2Bank),
            2 => Some(Epbkselect::_3Bank),
            _ => None,
        }
    }
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == Epbkselect::_1Bank
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == Epbkselect::_2Bank
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == Epbkselect::_3Bank
    }
}
#[doc = "Field `EPBK` writer - Endpoint Banks"]
pub type EpbkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Epbkselect>;
impl<'a, REG> EpbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Epbkselect::_1Bank)
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Epbkselect::_2Bank)
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Epbkselect::_3Bank)
    }
}
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Epsizeselect {
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
impl From<Epsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Epsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Epsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Epsizeselect {}
#[doc = "Field `EPSIZE` reader - Endpoint Size"]
pub type EpsizeR = crate::FieldReader<Epsizeselect>;
impl EpsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epsizeselect {
        match self.bits {
            0 => Epsizeselect::_8Byte,
            1 => Epsizeselect::_16Byte,
            2 => Epsizeselect::_32Byte,
            3 => Epsizeselect::_64Byte,
            4 => Epsizeselect::_128Byte,
            5 => Epsizeselect::_256Byte,
            6 => Epsizeselect::_512Byte,
            7 => Epsizeselect::_1024Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Epsizeselect::_8Byte
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Epsizeselect::_16Byte
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Epsizeselect::_32Byte
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == Epsizeselect::_64Byte
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == Epsizeselect::_128Byte
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == Epsizeselect::_256Byte
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == Epsizeselect::_512Byte
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == Epsizeselect::_1024Byte
    }
}
#[doc = "Field `EPSIZE` writer - Endpoint Size"]
pub type EpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Epsizeselect, crate::Safe>;
impl<'a, REG> EpsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_8Byte)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_16Byte)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_32Byte)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_64Byte)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_128Byte)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_256Byte)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_512Byte)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsizeselect::_1024Byte)
    }
}
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epdirselect {
    #[doc = "0: The endpoint direction is OUT."]
    Out = 0,
    #[doc = "1: The endpoint direction is IN (nor for control endpoints)."]
    In = 1,
}
impl From<Epdirselect> for bool {
    #[inline(always)]
    fn from(variant: Epdirselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EpdirR = crate::BitReader<Epdirselect>;
impl EpdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epdirselect {
        match self.bits {
            false => Epdirselect::Out,
            true => Epdirselect::In,
        }
    }
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Epdirselect::Out
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Epdirselect::In
    }
}
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG, Epdirselect>;
impl<'a, REG> EpdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Epdirselect::Out)
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Epdirselect::In)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AutoswR = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AutoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptypeselect {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous"]
    Iso = 1,
    #[doc = "2: Bulk"]
    Blk = 2,
    #[doc = "3: Interrupt"]
    Intrpt = 3,
}
impl From<Eptypeselect> for u8 {
    #[inline(always)]
    fn from(variant: Eptypeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptypeselect {
    type Ux = u8;
}
impl crate::IsEnum for Eptypeselect {}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader<Eptypeselect>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eptypeselect {
        match self.bits {
            0 => Eptypeselect::Ctrl,
            1 => Eptypeselect::Iso,
            2 => Eptypeselect::Blk,
            3 => Eptypeselect::Intrpt,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == Eptypeselect::Ctrl
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Eptypeselect::Iso
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == Eptypeselect::Blk
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == Eptypeselect::Intrpt
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eptypeselect, crate::Safe>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Eptypeselect::Ctrl)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Eptypeselect::Iso)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(Eptypeselect::Blk)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut crate::W<REG> {
        self.variant(Eptypeselect::Intrpt)
    }
}
#[doc = "Number of transactions per microframe for isochronous endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbtransselect {
    #[doc = "0: Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0Trans = 0,
    #[doc = "1: Default value: one transaction per microframe."]
    _1Trans = 1,
    #[doc = "2: Two transactions per microframe. This endpoint should be configured as double-bank."]
    _2Trans = 2,
    #[doc = "3: Three transactions per microframe. This endpoint should be configured as triple-bank."]
    _3Trans = 3,
}
impl From<Nbtransselect> for u8 {
    #[inline(always)]
    fn from(variant: Nbtransselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbtransselect {
    type Ux = u8;
}
impl crate::IsEnum for Nbtransselect {}
#[doc = "Field `NBTRANS` reader - Number of transactions per microframe for isochronous endpoint"]
pub type NbtransR = crate::FieldReader<Nbtransselect>;
impl NbtransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbtransselect {
        match self.bits {
            0 => Nbtransselect::_0Trans,
            1 => Nbtransselect::_1Trans,
            2 => Nbtransselect::_2Trans,
            3 => Nbtransselect::_3Trans,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn is_0_trans(&self) -> bool {
        *self == Nbtransselect::_0Trans
    }
    #[doc = "Default value: one transaction per microframe."]
    #[inline(always)]
    pub fn is_1_trans(&self) -> bool {
        *self == Nbtransselect::_1Trans
    }
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn is_2_trans(&self) -> bool {
        *self == Nbtransselect::_2Trans
    }
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn is_3_trans(&self) -> bool {
        *self == Nbtransselect::_3Trans
    }
}
#[doc = "Field `NBTRANS` writer - Number of transactions per microframe for isochronous endpoint"]
pub type NbtransW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nbtransselect, crate::Safe>;
impl<'a, REG> NbtransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn _0_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtransselect::_0Trans)
    }
    #[doc = "Default value: one transaction per microframe."]
    #[inline(always)]
    pub fn _1_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtransselect::_1Trans)
    }
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn _2_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtransselect::_2Trans)
    }
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn _3_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtransselect::_3Trans)
    }
}
impl R {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> AllocR {
        AllocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&self) -> EpbkR {
        EpbkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EpsizeR {
        EpsizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AutoswR {
        AutoswR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&self) -> NbtransR {
        NbtransR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&mut self) -> AllocW<DeveptcfgSpec> {
        AllocW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&mut self) -> EpbkW<DeveptcfgSpec> {
        EpbkW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&mut self) -> EpsizeW<DeveptcfgSpec> {
        EpsizeW::new(self, 4)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EpdirW<DeveptcfgSpec> {
        EpdirW::new(self, 8)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&mut self) -> AutoswW<DeveptcfgSpec> {
        AutoswW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<DeveptcfgSpec> {
        EptypeW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&mut self) -> NbtransW<DeveptcfgSpec> {
        NbtransW::new(self, 13)
    }
}
#[doc = "Device Endpoint Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeveptcfgSpec;
impl crate::RegisterSpec for DeveptcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptcfg::R`](R) reader structure"]
impl crate::Readable for DeveptcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`deveptcfg::W`](W) writer structure"]
impl crate::Writable for DeveptcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVEPTCFG[%s] to value 0"]
impl crate::Resettable for DeveptcfgSpec {}
