#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DbtpSpec>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DbtpSpec>;
#[doc = "Field `DSJW` reader - Data (Re) Synchronization Jump Width"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - Data (Re) Synchronization Jump Width"]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTSEG2` reader - Data Time Segment After Sample Point"]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data Time Segment After Sample Point"]
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data Time Segment Before Sample Point"]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data Time Segment Before Sample Point"]
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data Bit Rate Prescaler"]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data Bit Rate Prescaler"]
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transmitter Delay Compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcselect {
    #[doc = "0: Transmitter Delay Compensation disabled."]
    Disabled = 0,
    #[doc = "1: Transmitter Delay Compensation enabled."]
    Enabled = 1,
}
impl From<Tdcselect> for bool {
    #[inline(always)]
    fn from(variant: Tdcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transmitter Delay Compensation"]
pub type TdcR = crate::BitReader<Tdcselect>;
impl TdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdcselect {
        match self.bits {
            false => Tdcselect::Disabled,
            true => Tdcselect::Enabled,
        }
    }
    #[doc = "Transmitter Delay Compensation disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdcselect::Disabled
    }
    #[doc = "Transmitter Delay Compensation enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdcselect::Enabled
    }
}
#[doc = "Field `TDC` writer - Transmitter Delay Compensation"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG, Tdcselect>;
impl<'a, REG> TdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter Delay Compensation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcselect::Disabled)
    }
    #[doc = "Transmitter Delay Compensation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcselect::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - Data (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Data Time Segment After Sample Point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&mut self) -> DsjwW<DbtpSpec> {
        DsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data Time Segment After Sample Point"]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> Dtseg2W<DbtpSpec> {
        Dtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> Dtseg1W<DbtpSpec> {
        Dtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&mut self) -> DbrpW<DbtpSpec> {
        DbrpW::new(self, 16)
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TdcW<DbtpSpec> {
        TdcW::new(self, 23)
    }
}
#[doc = "Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbtpSpec;
impl crate::RegisterSpec for DbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DbtpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBTP to value 0"]
impl crate::Resettable for DbtpSpec {}
