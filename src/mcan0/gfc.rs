#[doc = "Register `GFC` reader"]
pub type R = crate::R<GfcSpec>;
#[doc = "Register `GFC` writer"]
pub type W = crate::W<GfcSpec>;
#[doc = "Reject Remote Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrfeselect {
    #[doc = "0: Filter remote frames with 29-bit extended IDs."]
    Filter = 0,
    #[doc = "1: Reject all remote frames with 29-bit extended IDs."]
    Reject = 1,
}
impl From<Rrfeselect> for bool {
    #[inline(always)]
    fn from(variant: Rrfeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RrfeR = crate::BitReader<Rrfeselect>;
impl RrfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrfeselect {
        match self.bits {
            false => Rrfeselect::Filter,
            true => Rrfeselect::Reject,
        }
    }
    #[doc = "Filter remote frames with 29-bit extended IDs."]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == Rrfeselect::Filter
    }
    #[doc = "Reject all remote frames with 29-bit extended IDs."]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == Rrfeselect::Reject
    }
}
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG, Rrfeselect>;
impl<'a, REG> RrfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter remote frames with 29-bit extended IDs."]
    #[inline(always)]
    pub fn filter(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfeselect::Filter)
    }
    #[doc = "Reject all remote frames with 29-bit extended IDs."]
    #[inline(always)]
    pub fn reject(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfeselect::Reject)
    }
}
#[doc = "Reject Remote Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrfsselect {
    #[doc = "0: Filter remote frames with 11-bit standard IDs."]
    Filter = 0,
    #[doc = "1: Reject all remote frames with 11-bit standard IDs."]
    Reject = 1,
}
impl From<Rrfsselect> for bool {
    #[inline(always)]
    fn from(variant: Rrfsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RrfsR = crate::BitReader<Rrfsselect>;
impl RrfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrfsselect {
        match self.bits {
            false => Rrfsselect::Filter,
            true => Rrfsselect::Reject,
        }
    }
    #[doc = "Filter remote frames with 11-bit standard IDs."]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == Rrfsselect::Filter
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs."]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == Rrfsselect::Reject
    }
}
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG, Rrfsselect>;
impl<'a, REG> RrfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter remote frames with 11-bit standard IDs."]
    #[inline(always)]
    pub fn filter(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfsselect::Filter)
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs."]
    #[inline(always)]
    pub fn reject(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfsselect::Reject)
    }
}
#[doc = "Accept Non-matching Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anfeselect {
    #[doc = "0: Accept in Rx FIFO 0"]
    RxFifo0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RxFifo1 = 1,
}
impl From<Anfeselect> for u8 {
    #[inline(always)]
    fn from(variant: Anfeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anfeselect {
    type Ux = u8;
}
impl crate::IsEnum for Anfeselect {}
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type AnfeR = crate::FieldReader<Anfeselect>;
impl AnfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anfeselect> {
        match self.bits {
            0 => Some(Anfeselect::RxFifo0),
            1 => Some(Anfeselect::RxFifo1),
            _ => None,
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rx_fifo_0(&self) -> bool {
        *self == Anfeselect::RxFifo0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rx_fifo_1(&self) -> bool {
        *self == Anfeselect::RxFifo1
    }
}
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anfeselect>;
impl<'a, REG> AnfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rx_fifo_0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfeselect::RxFifo0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rx_fifo_1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfeselect::RxFifo1)
    }
}
#[doc = "Accept Non-matching Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anfsselect {
    #[doc = "0: Accept in Rx FIFO 0"]
    RxFifo0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RxFifo1 = 1,
}
impl From<Anfsselect> for u8 {
    #[inline(always)]
    fn from(variant: Anfsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anfsselect {
    type Ux = u8;
}
impl crate::IsEnum for Anfsselect {}
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type AnfsR = crate::FieldReader<Anfsselect>;
impl AnfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anfsselect> {
        match self.bits {
            0 => Some(Anfsselect::RxFifo0),
            1 => Some(Anfsselect::RxFifo1),
            _ => None,
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rx_fifo_0(&self) -> bool {
        *self == Anfsselect::RxFifo0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rx_fifo_1(&self) -> bool {
        *self == Anfsselect::RxFifo1
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anfsselect>;
impl<'a, REG> AnfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rx_fifo_0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfsselect::RxFifo0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rx_fifo_1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfsselect::RxFifo1)
    }
}
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RrfeW<GfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RrfsW<GfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&mut self) -> AnfeW<GfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&mut self) -> AnfsW<GfcSpec> {
        AnfsW::new(self, 4)
    }
}
#[doc = "Global Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfcSpec;
impl crate::RegisterSpec for GfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfc::R`](R) reader structure"]
impl crate::Readable for GfcSpec {}
#[doc = "`write(|w| ..)` method takes [`gfc::W`](W) writer structure"]
impl crate::Writable for GfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GfcSpec {}
