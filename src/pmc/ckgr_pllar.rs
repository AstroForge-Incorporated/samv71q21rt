#[doc = "Register `CKGR_PLLAR` reader"]
pub type R = crate::R<CkgrPllarSpec>;
#[doc = "Register `CKGR_PLLAR` writer"]
pub type W = crate::W<CkgrPllarSpec>;
#[doc = "PLLA Front End Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divaselect {
    #[doc = "0: Divider output is 0 and PLLA is disabled."]
    _0 = 0,
    #[doc = "1: Divider is bypassed (divide by 1) and PLLA is enabled."]
    Bypass = 1,
}
impl From<Divaselect> for u8 {
    #[inline(always)]
    fn from(variant: Divaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divaselect {
    type Ux = u8;
}
impl crate::IsEnum for Divaselect {}
#[doc = "Field `DIVA` reader - PLLA Front End Divider"]
pub type DivaR = crate::FieldReader<Divaselect>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divaselect> {
        match self.bits {
            0 => Some(Divaselect::_0),
            1 => Some(Divaselect::Bypass),
            _ => None,
        }
    }
    #[doc = "Divider output is 0 and PLLA is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Divaselect::_0
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Divaselect::Bypass
    }
}
#[doc = "Field `DIVA` writer - PLLA Front End Divider"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divaselect>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divider output is 0 and PLLA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Divaselect::_0)
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Divaselect::Bypass)
    }
}
#[doc = "Field `PLLACOUNT` reader - PLLA Counter"]
pub type PllacountR = crate::FieldReader;
#[doc = "Field `PLLACOUNT` writer - PLLA Counter"]
pub type PllacountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MULA` reader - PLLA Multiplier"]
pub type MulaR = crate::FieldReader<u16>;
#[doc = "Field `MULA` writer - PLLA Multiplier"]
pub type MulaW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&self) -> PllacountR {
        PllacountR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&self) -> MulaR {
        MulaR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<CkgrPllarSpec> {
        DivaW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&mut self) -> PllacountW<CkgrPllarSpec> {
        PllacountW::new(self, 8)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&mut self) -> MulaW<CkgrPllarSpec> {
        MulaW::new(self, 16)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> OneW<CkgrPllarSpec> {
        OneW::new(self, 29)
    }
}
#[doc = "PLLA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrPllarSpec;
impl crate::RegisterSpec for CkgrPllarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_pllar::R`](R) reader structure"]
impl crate::Readable for CkgrPllarSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_pllar::W`](W) writer structure"]
impl crate::Writable for CkgrPllarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKGR_PLLAR to value 0"]
impl crate::Resettable for CkgrPllarSpec {}
