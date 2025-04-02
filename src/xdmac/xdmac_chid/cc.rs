#[doc = "Register `CC` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "Channel x Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typeselect {
    #[doc = "0: Self-triggered mode (memory-to-memory transfer)."]
    MemTran = 0,
    #[doc = "1: Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    PerTran = 1,
}
impl From<Typeselect> for bool {
    #[inline(always)]
    fn from(variant: Typeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE` reader - Channel x Transfer Type"]
pub type TypeR = crate::BitReader<Typeselect>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typeselect {
        match self.bits {
            false => Typeselect::MemTran,
            true => Typeselect::PerTran,
        }
    }
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        *self == Typeselect::MemTran
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        *self == Typeselect::PerTran
    }
}
#[doc = "Field `TYPE` writer - Channel x Transfer Type"]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG, Typeselect>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut crate::W<REG> {
        self.variant(Typeselect::MemTran)
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut crate::W<REG> {
        self.variant(Typeselect::PerTran)
    }
}
#[doc = "Channel x Memory Burst Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mbsizeselect {
    #[doc = "0: The memory burst size is set to one."]
    Single = 0,
    #[doc = "1: The memory burst size is set to four."]
    Four = 1,
    #[doc = "2: The memory burst size is set to eight."]
    Eight = 2,
    #[doc = "3: The memory burst size is set to sixteen."]
    Sixteen = 3,
}
impl From<Mbsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Mbsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mbsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Mbsizeselect {}
#[doc = "Field `MBSIZE` reader - Channel x Memory Burst Size"]
pub type MbsizeR = crate::FieldReader<Mbsizeselect>;
impl MbsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbsizeselect {
        match self.bits {
            0 => Mbsizeselect::Single,
            1 => Mbsizeselect::Four,
            2 => Mbsizeselect::Eight,
            3 => Mbsizeselect::Sixteen,
            _ => unreachable!(),
        }
    }
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Mbsizeselect::Single
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Mbsizeselect::Four
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Mbsizeselect::Eight
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == Mbsizeselect::Sixteen
    }
}
#[doc = "Field `MBSIZE` writer - Channel x Memory Burst Size"]
pub type MbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mbsizeselect, crate::Safe>;
impl<'a, REG> MbsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Single)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Four)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Eight)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Sixteen)
    }
}
#[doc = "Channel x Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsyncselect {
    #[doc = "0: Peripheral-to-memory transfer."]
    Per2mem = 0,
    #[doc = "1: Memory-to-peripheral transfer."]
    Mem2per = 1,
}
impl From<Dsyncselect> for bool {
    #[inline(always)]
    fn from(variant: Dsyncselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSYNC` reader - Channel x Synchronization"]
pub type DsyncR = crate::BitReader<Dsyncselect>;
impl DsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsyncselect {
        match self.bits {
            false => Dsyncselect::Per2mem,
            true => Dsyncselect::Mem2per,
        }
    }
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        *self == Dsyncselect::Per2mem
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        *self == Dsyncselect::Mem2per
    }
}
#[doc = "Field `DSYNC` writer - Channel x Synchronization"]
pub type DsyncW<'a, REG> = crate::BitWriter<'a, REG, Dsyncselect>;
impl<'a, REG> DsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut crate::W<REG> {
        self.variant(Dsyncselect::Per2mem)
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut crate::W<REG> {
        self.variant(Dsyncselect::Mem2per)
    }
}
#[doc = "Channel x Software Request Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swreqselect {
    #[doc = "0: Hardware request line is connected to the peripheral request line."]
    HwrConnected = 0,
    #[doc = "1: Software request is connected to the peripheral request line."]
    SwrConnected = 1,
}
impl From<Swreqselect> for bool {
    #[inline(always)]
    fn from(variant: Swreqselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ` reader - Channel x Software Request Trigger"]
pub type SwreqR = crate::BitReader<Swreqselect>;
impl SwreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swreqselect {
        match self.bits {
            false => Swreqselect::HwrConnected,
            true => Swreqselect::SwrConnected,
        }
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        *self == Swreqselect::HwrConnected
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        *self == Swreqselect::SwrConnected
    }
}
#[doc = "Field `SWREQ` writer - Channel x Software Request Trigger"]
pub type SwreqW<'a, REG> = crate::BitWriter<'a, REG, Swreqselect>;
impl<'a, REG> SwreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut crate::W<REG> {
        self.variant(Swreqselect::HwrConnected)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut crate::W<REG> {
        self.variant(Swreqselect::SwrConnected)
    }
}
#[doc = "Channel x Fill Block of memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memsetselect {
    #[doc = "0: Memset is not activated."]
    NormalMode = 0,
    #[doc = "1: Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HwMode = 1,
}
impl From<Memsetselect> for bool {
    #[inline(always)]
    fn from(variant: Memsetselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMSET` reader - Channel x Fill Block of memory"]
pub type MemsetR = crate::BitReader<Memsetselect>;
impl MemsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memsetselect {
        match self.bits {
            false => Memsetselect::NormalMode,
            true => Memsetselect::HwMode,
        }
    }
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Memsetselect::NormalMode
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        *self == Memsetselect::HwMode
    }
}
#[doc = "Field `MEMSET` writer - Channel x Fill Block of memory"]
pub type MemsetW<'a, REG> = crate::BitWriter<'a, REG, Memsetselect>;
impl<'a, REG> MemsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Memsetselect::NormalMode)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Memsetselect::HwMode)
    }
}
#[doc = "Channel x Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csizeselect {
    #[doc = "0: 1 data transferred"]
    Chk1 = 0,
    #[doc = "1: 2 data transferred"]
    Chk2 = 1,
    #[doc = "2: 4 data transferred"]
    Chk4 = 2,
    #[doc = "3: 8 data transferred"]
    Chk8 = 3,
    #[doc = "4: 16 data transferred"]
    Chk16 = 4,
}
impl From<Csizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Csizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Csizeselect {}
#[doc = "Field `CSIZE` reader - Channel x Chunk Size"]
pub type CsizeR = crate::FieldReader<Csizeselect>;
impl CsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csizeselect> {
        match self.bits {
            0 => Some(Csizeselect::Chk1),
            1 => Some(Csizeselect::Chk2),
            2 => Some(Csizeselect::Chk4),
            3 => Some(Csizeselect::Chk8),
            4 => Some(Csizeselect::Chk16),
            _ => None,
        }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == Csizeselect::Chk1
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        *self == Csizeselect::Chk2
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == Csizeselect::Chk4
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == Csizeselect::Chk8
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == Csizeselect::Chk16
    }
}
#[doc = "Field `CSIZE` writer - Channel x Chunk Size"]
pub type CsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Csizeselect>;
impl<'a, REG> CsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk16)
    }
}
#[doc = "Channel x Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwidthselect {
    #[doc = "0: The data size is set to 8 bits"]
    Byte = 0,
    #[doc = "1: The data size is set to 16 bits"]
    Halfword = 1,
    #[doc = "2: The data size is set to 32 bits"]
    Word = 2,
}
impl From<Dwidthselect> for u8 {
    #[inline(always)]
    fn from(variant: Dwidthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwidthselect {
    type Ux = u8;
}
impl crate::IsEnum for Dwidthselect {}
#[doc = "Field `DWIDTH` reader - Channel x Data Width"]
pub type DwidthR = crate::FieldReader<Dwidthselect>;
impl DwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dwidthselect> {
        match self.bits {
            0 => Some(Dwidthselect::Byte),
            1 => Some(Dwidthselect::Halfword),
            2 => Some(Dwidthselect::Word),
            _ => None,
        }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dwidthselect::Byte
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == Dwidthselect::Halfword
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dwidthselect::Word
    }
}
#[doc = "Field `DWIDTH` writer - Channel x Data Width"]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dwidthselect>;
impl<'a, REG> DwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidthselect::Byte)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidthselect::Halfword)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidthselect::Word)
    }
}
#[doc = "Channel x Source Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sifselect {
    #[doc = "0: The data is read through the system bus interface 0."]
    AhbIf0 = 0,
    #[doc = "1: The data is read through the system bus interface 1."]
    AhbIf1 = 1,
}
impl From<Sifselect> for bool {
    #[inline(always)]
    fn from(variant: Sifselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIF` reader - Channel x Source Interface Identifier"]
pub type SifR = crate::BitReader<Sifselect>;
impl SifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sifselect {
        match self.bits {
            false => Sifselect::AhbIf0,
            true => Sifselect::AhbIf1,
        }
    }
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == Sifselect::AhbIf0
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == Sifselect::AhbIf1
    }
}
#[doc = "Field `SIF` writer - Channel x Source Interface Identifier"]
pub type SifW<'a, REG> = crate::BitWriter<'a, REG, Sifselect>;
impl<'a, REG> SifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut crate::W<REG> {
        self.variant(Sifselect::AhbIf0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut crate::W<REG> {
        self.variant(Sifselect::AhbIf1)
    }
}
#[doc = "Channel x Destination Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difselect {
    #[doc = "0: The data is written through the system bus interface 0."]
    AhbIf0 = 0,
    #[doc = "1: The data is written though the system bus interface 1."]
    AhbIf1 = 1,
}
impl From<Difselect> for bool {
    #[inline(always)]
    fn from(variant: Difselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIF` reader - Channel x Destination Interface Identifier"]
pub type DifR = crate::BitReader<Difselect>;
impl DifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Difselect {
        match self.bits {
            false => Difselect::AhbIf0,
            true => Difselect::AhbIf1,
        }
    }
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == Difselect::AhbIf0
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == Difselect::AhbIf1
    }
}
#[doc = "Field `DIF` writer - Channel x Destination Interface Identifier"]
pub type DifW<'a, REG> = crate::BitWriter<'a, REG, Difselect>;
impl<'a, REG> DifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut crate::W<REG> {
        self.variant(Difselect::AhbIf0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut crate::W<REG> {
        self.variant(Difselect::AhbIf1)
    }
}
#[doc = "Channel x Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Samselect {
    #[doc = "0: The address remains unchanged."]
    FixedAm = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    IncrementedAm = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UbsAm = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UbsDsAm = 3,
}
impl From<Samselect> for u8 {
    #[inline(always)]
    fn from(variant: Samselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Samselect {
    type Ux = u8;
}
impl crate::IsEnum for Samselect {}
#[doc = "Field `SAM` reader - Channel x Source Addressing Mode"]
pub type SamR = crate::FieldReader<Samselect>;
impl SamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samselect {
        match self.bits {
            0 => Samselect::FixedAm,
            1 => Samselect::IncrementedAm,
            2 => Samselect::UbsAm,
            3 => Samselect::UbsDsAm,
            _ => unreachable!(),
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == Samselect::FixedAm
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == Samselect::IncrementedAm
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == Samselect::UbsAm
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == Samselect::UbsDsAm
    }
}
#[doc = "Field `SAM` writer - Channel x Source Addressing Mode"]
pub type SamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Samselect, crate::Safe>;
impl<'a, REG> SamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::FixedAm)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::IncrementedAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::UbsAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::UbsDsAm)
    }
}
#[doc = "Channel x Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Damselect {
    #[doc = "0: The address remains unchanged."]
    FixedAm = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    IncrementedAm = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UbsAm = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    UbsDsAm = 3,
}
impl From<Damselect> for u8 {
    #[inline(always)]
    fn from(variant: Damselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Damselect {
    type Ux = u8;
}
impl crate::IsEnum for Damselect {}
#[doc = "Field `DAM` reader - Channel x Destination Addressing Mode"]
pub type DamR = crate::FieldReader<Damselect>;
impl DamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Damselect {
        match self.bits {
            0 => Damselect::FixedAm,
            1 => Damselect::IncrementedAm,
            2 => Damselect::UbsAm,
            3 => Damselect::UbsDsAm,
            _ => unreachable!(),
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == Damselect::FixedAm
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == Damselect::IncrementedAm
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == Damselect::UbsAm
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == Damselect::UbsDsAm
    }
}
#[doc = "Field `DAM` writer - Channel x Destination Addressing Mode"]
pub type DamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Damselect, crate::Safe>;
impl<'a, REG> DamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::FixedAm)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::IncrementedAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::UbsAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::UbsDsAm)
    }
}
#[doc = "Channel Initialization Terminated (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initdselect {
    #[doc = "0: Channel initialization is in progress."]
    InProgress = 0,
    #[doc = "1: Channel initialization is completed."]
    Terminated = 1,
}
impl From<Initdselect> for bool {
    #[inline(always)]
    fn from(variant: Initdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITD` reader - Channel Initialization Terminated (this bit is read-only)"]
pub type InitdR = crate::BitReader<Initdselect>;
impl InitdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initdselect {
        match self.bits {
            false => Initdselect::InProgress,
            true => Initdselect::Terminated,
        }
    }
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Initdselect::InProgress
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        *self == Initdselect::Terminated
    }
}
#[doc = "Field `INITD` writer - Channel Initialization Terminated (this bit is read-only)"]
pub type InitdW<'a, REG> = crate::BitWriter<'a, REG, Initdselect>;
impl<'a, REG> InitdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Initdselect::InProgress)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut crate::W<REG> {
        self.variant(Initdselect::Terminated)
    }
}
#[doc = "Read in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdipselect {
    #[doc = "0: No active read transaction on the bus."]
    Done = 0,
    #[doc = "1: A read transaction is in progress."]
    InProgress = 1,
}
impl From<Rdipselect> for bool {
    #[inline(always)]
    fn from(variant: Rdipselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIP` reader - Read in Progress (this bit is read-only)"]
pub type RdipR = crate::BitReader<Rdipselect>;
impl RdipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdipselect {
        match self.bits {
            false => Rdipselect::Done,
            true => Rdipselect::InProgress,
        }
    }
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Rdipselect::Done
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Rdipselect::InProgress
    }
}
#[doc = "Field `RDIP` writer - Read in Progress (this bit is read-only)"]
pub type RdipW<'a, REG> = crate::BitWriter<'a, REG, Rdipselect>;
impl<'a, REG> RdipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(Rdipselect::Done)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Rdipselect::InProgress)
    }
}
#[doc = "Write in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wripselect {
    #[doc = "0: No active write transaction on the bus."]
    Done = 0,
    #[doc = "1: A write transaction is in progress."]
    InProgress = 1,
}
impl From<Wripselect> for bool {
    #[inline(always)]
    fn from(variant: Wripselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRIP` reader - Write in Progress (this bit is read-only)"]
pub type WripR = crate::BitReader<Wripselect>;
impl WripR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wripselect {
        match self.bits {
            false => Wripselect::Done,
            true => Wripselect::InProgress,
        }
    }
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Wripselect::Done
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Wripselect::InProgress
    }
}
#[doc = "Field `WRIP` writer - Write in Progress (this bit is read-only)"]
pub type WripW<'a, REG> = crate::BitWriter<'a, REG, Wripselect>;
impl<'a, REG> WripW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(Wripselect::Done)
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Wripselect::InProgress)
    }
}
#[doc = "Channel x Peripheral Hardware Request Line Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Peridselect {
    #[doc = "0: HSMCI"]
    Hsmci = 0,
    #[doc = "1: SPI0_TX"]
    Spi0Tx = 1,
    #[doc = "2: SPI0_RX"]
    Spi0Rx = 2,
    #[doc = "3: SPI1_TX"]
    Spi1Tx = 3,
    #[doc = "4: SPI1_RX"]
    Spi1Rx = 4,
    #[doc = "5: QSPI_TX"]
    QspiTx = 5,
    #[doc = "6: QSPI_RX"]
    QspiRx = 6,
    #[doc = "7: USART0_TX"]
    Usart0Tx = 7,
    #[doc = "8: USART0_RX"]
    Usart0Rx = 8,
    #[doc = "9: USART1_TX"]
    Usart1Tx = 9,
    #[doc = "10: USART1_RX"]
    Usart1Rx = 10,
    #[doc = "11: USART2_TX"]
    Usart2Tx = 11,
    #[doc = "12: USART2_RX"]
    Usart2Rx = 12,
    #[doc = "13: PWM0"]
    Pwm0 = 13,
    #[doc = "14: TWIHS0_TX"]
    Twihs0Tx = 14,
    #[doc = "15: TWIHS0_RX"]
    Twihs0Rx = 15,
    #[doc = "16: TWIHS1_TX"]
    Twihs1Tx = 16,
    #[doc = "17: TWIHS1_RX"]
    Twihs1Rx = 17,
    #[doc = "18: TWIHS2_TX"]
    Twihs2Tx = 18,
    #[doc = "19: TWIHS2_RX"]
    Twihs2Rx = 19,
    #[doc = "20: UART0_TX"]
    Uart0Tx = 20,
    #[doc = "21: UART0_RX"]
    Uart0Rx = 21,
    #[doc = "22: UART1_TX"]
    Uart1Tx = 22,
    #[doc = "23: UART1_RX"]
    Uart1Rx = 23,
    #[doc = "24: UART2_TX"]
    Uart2Tx = 24,
    #[doc = "25: UART2_RX"]
    Uart2Rx = 25,
    #[doc = "26: UART3_TX"]
    Uart3Tx = 26,
    #[doc = "27: UART3_RX"]
    Uart3Rx = 27,
    #[doc = "28: UART4_TX"]
    Uart4Tx = 28,
    #[doc = "29: UART4_RX"]
    Uart4Rx = 29,
    #[doc = "30: DACC0"]
    Dacc0 = 30,
    #[doc = "31: DACC1"]
    Dacc1 = 31,
    #[doc = "32: SSC_TX"]
    SscTx = 32,
    #[doc = "33: SSC_RX"]
    SscRx = 33,
    #[doc = "34: PIOA"]
    Pioa = 34,
    #[doc = "35: AFEC0"]
    Afec0 = 35,
    #[doc = "36: AFEC1"]
    Afec1 = 36,
    #[doc = "37: AES_TX"]
    AesTx = 37,
    #[doc = "38: AES_RX"]
    AesRx = 38,
    #[doc = "39: PWM1"]
    Pwm1 = 39,
    #[doc = "40: TC0"]
    Tc0 = 40,
    #[doc = "41: TC3"]
    Tc3 = 41,
    #[doc = "42: TC6"]
    Tc6 = 42,
    #[doc = "43: TC9"]
    Tc9 = 43,
    #[doc = "44: I2SC0_TX_LEFT"]
    I2sc0TxLeft = 44,
    #[doc = "45: I2SC0_RX_LEFT"]
    I2sc0RxLeft = 45,
    #[doc = "46: I2SC1_TX_LEFT"]
    I2sc1TxLeft = 46,
    #[doc = "47: I2SC1_RX_LEFT"]
    I2sc1RxLeft = 47,
    #[doc = "48: I2SC0_TX_RIGHT"]
    I2sc0TxRight = 48,
    #[doc = "49: I2SC0_RX_RIGHT"]
    I2sc0RxRight = 49,
    #[doc = "50: I2SC1_TX_RIGHT"]
    I2sc1TxRight = 50,
    #[doc = "51: I2SC1_RX_RIGHT"]
    I2sc1RxRight = 51,
}
impl From<Peridselect> for u8 {
    #[inline(always)]
    fn from(variant: Peridselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peridselect {
    type Ux = u8;
}
impl crate::IsEnum for Peridselect {}
#[doc = "Field `PERID` reader - Channel x Peripheral Hardware Request Line Identifier"]
pub type PeridR = crate::FieldReader<Peridselect>;
impl PeridR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Peridselect> {
        match self.bits {
            0 => Some(Peridselect::Hsmci),
            1 => Some(Peridselect::Spi0Tx),
            2 => Some(Peridselect::Spi0Rx),
            3 => Some(Peridselect::Spi1Tx),
            4 => Some(Peridselect::Spi1Rx),
            5 => Some(Peridselect::QspiTx),
            6 => Some(Peridselect::QspiRx),
            7 => Some(Peridselect::Usart0Tx),
            8 => Some(Peridselect::Usart0Rx),
            9 => Some(Peridselect::Usart1Tx),
            10 => Some(Peridselect::Usart1Rx),
            11 => Some(Peridselect::Usart2Tx),
            12 => Some(Peridselect::Usart2Rx),
            13 => Some(Peridselect::Pwm0),
            14 => Some(Peridselect::Twihs0Tx),
            15 => Some(Peridselect::Twihs0Rx),
            16 => Some(Peridselect::Twihs1Tx),
            17 => Some(Peridselect::Twihs1Rx),
            18 => Some(Peridselect::Twihs2Tx),
            19 => Some(Peridselect::Twihs2Rx),
            20 => Some(Peridselect::Uart0Tx),
            21 => Some(Peridselect::Uart0Rx),
            22 => Some(Peridselect::Uart1Tx),
            23 => Some(Peridselect::Uart1Rx),
            24 => Some(Peridselect::Uart2Tx),
            25 => Some(Peridselect::Uart2Rx),
            26 => Some(Peridselect::Uart3Tx),
            27 => Some(Peridselect::Uart3Rx),
            28 => Some(Peridselect::Uart4Tx),
            29 => Some(Peridselect::Uart4Rx),
            30 => Some(Peridselect::Dacc0),
            31 => Some(Peridselect::Dacc1),
            32 => Some(Peridselect::SscTx),
            33 => Some(Peridselect::SscRx),
            34 => Some(Peridselect::Pioa),
            35 => Some(Peridselect::Afec0),
            36 => Some(Peridselect::Afec1),
            37 => Some(Peridselect::AesTx),
            38 => Some(Peridselect::AesRx),
            39 => Some(Peridselect::Pwm1),
            40 => Some(Peridselect::Tc0),
            41 => Some(Peridselect::Tc3),
            42 => Some(Peridselect::Tc6),
            43 => Some(Peridselect::Tc9),
            44 => Some(Peridselect::I2sc0TxLeft),
            45 => Some(Peridselect::I2sc0RxLeft),
            46 => Some(Peridselect::I2sc1TxLeft),
            47 => Some(Peridselect::I2sc1RxLeft),
            48 => Some(Peridselect::I2sc0TxRight),
            49 => Some(Peridselect::I2sc0RxRight),
            50 => Some(Peridselect::I2sc1TxRight),
            51 => Some(Peridselect::I2sc1RxRight),
            _ => None,
        }
    }
    #[doc = "HSMCI"]
    #[inline(always)]
    pub fn is_hsmci(&self) -> bool {
        *self == Peridselect::Hsmci
    }
    #[doc = "SPI0_TX"]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        *self == Peridselect::Spi0Tx
    }
    #[doc = "SPI0_RX"]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        *self == Peridselect::Spi0Rx
    }
    #[doc = "SPI1_TX"]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == Peridselect::Spi1Tx
    }
    #[doc = "SPI1_RX"]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == Peridselect::Spi1Rx
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == Peridselect::QspiTx
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == Peridselect::QspiRx
    }
    #[doc = "USART0_TX"]
    #[inline(always)]
    pub fn is_usart0_tx(&self) -> bool {
        *self == Peridselect::Usart0Tx
    }
    #[doc = "USART0_RX"]
    #[inline(always)]
    pub fn is_usart0_rx(&self) -> bool {
        *self == Peridselect::Usart0Rx
    }
    #[doc = "USART1_TX"]
    #[inline(always)]
    pub fn is_usart1_tx(&self) -> bool {
        *self == Peridselect::Usart1Tx
    }
    #[doc = "USART1_RX"]
    #[inline(always)]
    pub fn is_usart1_rx(&self) -> bool {
        *self == Peridselect::Usart1Rx
    }
    #[doc = "USART2_TX"]
    #[inline(always)]
    pub fn is_usart2_tx(&self) -> bool {
        *self == Peridselect::Usart2Tx
    }
    #[doc = "USART2_RX"]
    #[inline(always)]
    pub fn is_usart2_rx(&self) -> bool {
        *self == Peridselect::Usart2Rx
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == Peridselect::Pwm0
    }
    #[doc = "TWIHS0_TX"]
    #[inline(always)]
    pub fn is_twihs0_tx(&self) -> bool {
        *self == Peridselect::Twihs0Tx
    }
    #[doc = "TWIHS0_RX"]
    #[inline(always)]
    pub fn is_twihs0_rx(&self) -> bool {
        *self == Peridselect::Twihs0Rx
    }
    #[doc = "TWIHS1_TX"]
    #[inline(always)]
    pub fn is_twihs1_tx(&self) -> bool {
        *self == Peridselect::Twihs1Tx
    }
    #[doc = "TWIHS1_RX"]
    #[inline(always)]
    pub fn is_twihs1_rx(&self) -> bool {
        *self == Peridselect::Twihs1Rx
    }
    #[doc = "TWIHS2_TX"]
    #[inline(always)]
    pub fn is_twihs2_tx(&self) -> bool {
        *self == Peridselect::Twihs2Tx
    }
    #[doc = "TWIHS2_RX"]
    #[inline(always)]
    pub fn is_twihs2_rx(&self) -> bool {
        *self == Peridselect::Twihs2Rx
    }
    #[doc = "UART0_TX"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == Peridselect::Uart0Tx
    }
    #[doc = "UART0_RX"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == Peridselect::Uart0Rx
    }
    #[doc = "UART1_TX"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == Peridselect::Uart1Tx
    }
    #[doc = "UART1_RX"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == Peridselect::Uart1Rx
    }
    #[doc = "UART2_TX"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == Peridselect::Uart2Tx
    }
    #[doc = "UART2_RX"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == Peridselect::Uart2Rx
    }
    #[doc = "UART3_TX"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == Peridselect::Uart3Tx
    }
    #[doc = "UART3_RX"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == Peridselect::Uart3Rx
    }
    #[doc = "UART4_TX"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == Peridselect::Uart4Tx
    }
    #[doc = "UART4_RX"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == Peridselect::Uart4Rx
    }
    #[doc = "DACC0"]
    #[inline(always)]
    pub fn is_dacc0(&self) -> bool {
        *self == Peridselect::Dacc0
    }
    #[doc = "DACC1"]
    #[inline(always)]
    pub fn is_dacc1(&self) -> bool {
        *self == Peridselect::Dacc1
    }
    #[doc = "SSC_TX"]
    #[inline(always)]
    pub fn is_ssc_tx(&self) -> bool {
        *self == Peridselect::SscTx
    }
    #[doc = "SSC_RX"]
    #[inline(always)]
    pub fn is_ssc_rx(&self) -> bool {
        *self == Peridselect::SscRx
    }
    #[doc = "PIOA"]
    #[inline(always)]
    pub fn is_pioa(&self) -> bool {
        *self == Peridselect::Pioa
    }
    #[doc = "AFEC0"]
    #[inline(always)]
    pub fn is_afec0(&self) -> bool {
        *self == Peridselect::Afec0
    }
    #[doc = "AFEC1"]
    #[inline(always)]
    pub fn is_afec1(&self) -> bool {
        *self == Peridselect::Afec1
    }
    #[doc = "AES_TX"]
    #[inline(always)]
    pub fn is_aes_tx(&self) -> bool {
        *self == Peridselect::AesTx
    }
    #[doc = "AES_RX"]
    #[inline(always)]
    pub fn is_aes_rx(&self) -> bool {
        *self == Peridselect::AesRx
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Peridselect::Pwm1
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == Peridselect::Tc0
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn is_tc3(&self) -> bool {
        *self == Peridselect::Tc3
    }
    #[doc = "TC6"]
    #[inline(always)]
    pub fn is_tc6(&self) -> bool {
        *self == Peridselect::Tc6
    }
    #[doc = "TC9"]
    #[inline(always)]
    pub fn is_tc9(&self) -> bool {
        *self == Peridselect::Tc9
    }
    #[doc = "I2SC0_TX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc0_tx_left(&self) -> bool {
        *self == Peridselect::I2sc0TxLeft
    }
    #[doc = "I2SC0_RX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc0_rx_left(&self) -> bool {
        *self == Peridselect::I2sc0RxLeft
    }
    #[doc = "I2SC1_TX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc1_tx_left(&self) -> bool {
        *self == Peridselect::I2sc1TxLeft
    }
    #[doc = "I2SC1_RX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc1_rx_left(&self) -> bool {
        *self == Peridselect::I2sc1RxLeft
    }
    #[doc = "I2SC0_TX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc0_tx_right(&self) -> bool {
        *self == Peridselect::I2sc0TxRight
    }
    #[doc = "I2SC0_RX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc0_rx_right(&self) -> bool {
        *self == Peridselect::I2sc0RxRight
    }
    #[doc = "I2SC1_TX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc1_tx_right(&self) -> bool {
        *self == Peridselect::I2sc1TxRight
    }
    #[doc = "I2SC1_RX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc1_rx_right(&self) -> bool {
        *self == Peridselect::I2sc1RxRight
    }
}
#[doc = "Field `PERID` writer - Channel x Peripheral Hardware Request Line Identifier"]
pub type PeridW<'a, REG> = crate::FieldWriter<'a, REG, 7, Peridselect>;
impl<'a, REG> PeridW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSMCI"]
    #[inline(always)]
    pub fn hsmci(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Hsmci)
    }
    #[doc = "SPI0_TX"]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Spi0Tx)
    }
    #[doc = "SPI0_RX"]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Spi0Rx)
    }
    #[doc = "SPI1_TX"]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Spi1Tx)
    }
    #[doc = "SPI1_RX"]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Spi1Rx)
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::QspiTx)
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::QspiRx)
    }
    #[doc = "USART0_TX"]
    #[inline(always)]
    pub fn usart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Usart0Tx)
    }
    #[doc = "USART0_RX"]
    #[inline(always)]
    pub fn usart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Usart0Rx)
    }
    #[doc = "USART1_TX"]
    #[inline(always)]
    pub fn usart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Usart1Tx)
    }
    #[doc = "USART1_RX"]
    #[inline(always)]
    pub fn usart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Usart1Rx)
    }
    #[doc = "USART2_TX"]
    #[inline(always)]
    pub fn usart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Usart2Tx)
    }
    #[doc = "USART2_RX"]
    #[inline(always)]
    pub fn usart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Usart2Rx)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pwm0)
    }
    #[doc = "TWIHS0_TX"]
    #[inline(always)]
    pub fn twihs0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Twihs0Tx)
    }
    #[doc = "TWIHS0_RX"]
    #[inline(always)]
    pub fn twihs0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Twihs0Rx)
    }
    #[doc = "TWIHS1_TX"]
    #[inline(always)]
    pub fn twihs1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Twihs1Tx)
    }
    #[doc = "TWIHS1_RX"]
    #[inline(always)]
    pub fn twihs1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Twihs1Rx)
    }
    #[doc = "TWIHS2_TX"]
    #[inline(always)]
    pub fn twihs2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Twihs2Tx)
    }
    #[doc = "TWIHS2_RX"]
    #[inline(always)]
    pub fn twihs2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Twihs2Rx)
    }
    #[doc = "UART0_TX"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart0Tx)
    }
    #[doc = "UART0_RX"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart0Rx)
    }
    #[doc = "UART1_TX"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart1Tx)
    }
    #[doc = "UART1_RX"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart1Rx)
    }
    #[doc = "UART2_TX"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart2Tx)
    }
    #[doc = "UART2_RX"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart2Rx)
    }
    #[doc = "UART3_TX"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart3Tx)
    }
    #[doc = "UART3_RX"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart3Rx)
    }
    #[doc = "UART4_TX"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart4Tx)
    }
    #[doc = "UART4_RX"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Uart4Rx)
    }
    #[doc = "DACC0"]
    #[inline(always)]
    pub fn dacc0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Dacc0)
    }
    #[doc = "DACC1"]
    #[inline(always)]
    pub fn dacc1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Dacc1)
    }
    #[doc = "SSC_TX"]
    #[inline(always)]
    pub fn ssc_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::SscTx)
    }
    #[doc = "SSC_RX"]
    #[inline(always)]
    pub fn ssc_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::SscRx)
    }
    #[doc = "PIOA"]
    #[inline(always)]
    pub fn pioa(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pioa)
    }
    #[doc = "AFEC0"]
    #[inline(always)]
    pub fn afec0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Afec0)
    }
    #[doc = "AFEC1"]
    #[inline(always)]
    pub fn afec1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Afec1)
    }
    #[doc = "AES_TX"]
    #[inline(always)]
    pub fn aes_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::AesTx)
    }
    #[doc = "AES_RX"]
    #[inline(always)]
    pub fn aes_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::AesRx)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pwm1)
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc0)
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn tc3(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc3)
    }
    #[doc = "TC6"]
    #[inline(always)]
    pub fn tc6(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc6)
    }
    #[doc = "TC9"]
    #[inline(always)]
    pub fn tc9(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc9)
    }
    #[doc = "I2SC0_TX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_tx_left(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc0TxLeft)
    }
    #[doc = "I2SC0_RX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_rx_left(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc0RxLeft)
    }
    #[doc = "I2SC1_TX_LEFT"]
    #[inline(always)]
    pub fn i2sc1_tx_left(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc1TxLeft)
    }
    #[doc = "I2SC1_RX_LEFT"]
    #[inline(always)]
    pub fn i2sc1_rx_left(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc1RxLeft)
    }
    #[doc = "I2SC0_TX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_tx_right(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc0TxRight)
    }
    #[doc = "I2SC0_RX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_rx_right(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc0RxRight)
    }
    #[doc = "I2SC1_TX_RIGHT"]
    #[inline(always)]
    pub fn i2sc1_tx_right(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc1TxRight)
    }
    #[doc = "I2SC1_RX_RIGHT"]
    #[inline(always)]
    pub fn i2sc1_rx_right(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::I2sc1RxRight)
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MbsizeR {
        MbsizeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DsyncR {
        DsyncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SwreqR {
        SwreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&self) -> MemsetR {
        MemsetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CsizeR {
        CsizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SifR {
        SifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DifR {
        DifR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DamR {
        DamR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> InitdR {
        InitdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RdipR {
        RdipR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WripR {
        WripR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PeridR {
        PeridR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<CcSpec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&mut self) -> MbsizeW<CcSpec> {
        MbsizeW::new(self, 1)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&mut self) -> DsyncW<CcSpec> {
        DsyncW::new(self, 4)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<CcSpec> {
        SwreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&mut self) -> MemsetW<CcSpec> {
        MemsetW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&mut self) -> CsizeW<CcSpec> {
        CsizeW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<CcSpec> {
        DwidthW::new(self, 11)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&mut self) -> SifW<CcSpec> {
        SifW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&mut self) -> DifW<CcSpec> {
        DifW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&mut self) -> SamW<CcSpec> {
        SamW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&mut self) -> DamW<CcSpec> {
        DamW::new(self, 18)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&mut self) -> InitdW<CcSpec> {
        InitdW::new(self, 21)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&mut self) -> RdipW<CcSpec> {
        RdipW::new(self, 22)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&mut self) -> WripW<CcSpec> {
        WripW::new(self, 23)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PeridW<CcSpec> {
        PeridW::new(self, 24)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CcSpec {}
