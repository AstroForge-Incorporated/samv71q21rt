#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CmprSpec>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CmprSpec>;
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub type Val1R = crate::FieldReader;
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub type Val1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpmodeselect {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FlagOnly = 0,
    #[doc = "1: Comparison condition must be met to start reception."]
    StartCondition = 1,
}
impl From<Cmpmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Cmpmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CmpmodeR = crate::BitReader<Cmpmodeselect>;
impl CmpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmodeselect {
        match self.bits {
            false => Cmpmodeselect::FlagOnly,
            true => Cmpmodeselect::StartCondition,
        }
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == Cmpmodeselect::FlagOnly
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == Cmpmodeselect::StartCondition
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CmpmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmpmodeselect>;
impl<'a, REG> CmpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::FlagOnly)
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::StartCondition)
    }
}
#[doc = "Field `CMPPAR` reader - Compare Parity"]
pub type CmpparR = crate::BitReader;
#[doc = "Field `CMPPAR` writer - Compare Parity"]
pub type CmpparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub type Val2R = crate::FieldReader;
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub type Val2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> Val1R {
        Val1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CmpmodeR {
        CmpmodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&self) -> CmpparR {
        CmpparR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> Val2R {
        Val2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&mut self) -> Val1W<CmprSpec> {
        Val1W::new(self, 0)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CmpmodeW<CmprSpec> {
        CmpmodeW::new(self, 12)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&mut self) -> CmpparW<CmprSpec> {
        CmpparW::new(self, 14)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&mut self) -> Val2W<CmprSpec> {
        Val2W::new(self, 16)
    }
}
#[doc = "Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmprSpec;
impl crate::RegisterSpec for CmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CmprSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CmprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CmprSpec {}
