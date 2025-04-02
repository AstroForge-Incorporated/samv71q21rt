#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<Rxf1sSpec>;
#[doc = "Field `F1FL` reader - Receive FIFO 1 Fill Level"]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1GI` reader - Receive FIFO 1 Get Index"]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1PI` reader - Receive FIFO 1 Put Index"]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1F` reader - Receive FIFO 1 Fill Level"]
pub type F1fR = crate::BitReader;
#[doc = "Field `RF1L` reader - Receive FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Debug Message Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmsselect {
    #[doc = "0: Idle state, wait for reception of debug messages, DMA request is cleared."]
    Idle = 0,
    #[doc = "1: Debug message A received."]
    MsgA = 1,
    #[doc = "2: Debug messages A, B received."]
    MsgAb = 2,
    #[doc = "3: Debug messages A, B, C received, DMA request is set."]
    MsgAbc = 3,
}
impl From<Dmsselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmsselect {
    type Ux = u8;
}
impl crate::IsEnum for Dmsselect {}
#[doc = "Field `DMS` reader - Debug Message Status"]
pub type DmsR = crate::FieldReader<Dmsselect>;
impl DmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmsselect {
        match self.bits {
            0 => Dmsselect::Idle,
            1 => Dmsselect::MsgA,
            2 => Dmsselect::MsgAb,
            3 => Dmsselect::MsgAbc,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle state, wait for reception of debug messages, DMA request is cleared."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Dmsselect::Idle
    }
    #[doc = "Debug message A received."]
    #[inline(always)]
    pub fn is_msg_a(&self) -> bool {
        *self == Dmsselect::MsgA
    }
    #[doc = "Debug messages A, B received."]
    #[inline(always)]
    pub fn is_msg_ab(&self) -> bool {
        *self == Dmsselect::MsgAb
    }
    #[doc = "Debug messages A, B, C received, DMA request is set."]
    #[inline(always)]
    pub fn is_msg_abc(&self) -> bool {
        *self == Dmsselect::MsgAbc
    }
}
impl R {
    #[doc = "Bits 0:6 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Receive FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Receive FIFO 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1sSpec;
impl crate::RegisterSpec for Rxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for Rxf1sSpec {}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for Rxf1sSpec {}
