#[doc = "Register `MADR` reader"]
pub type R = crate::R<MadrSpec>;
#[doc = "Register `MADR` writer"]
pub type W = crate::W<MadrSpec>;
#[doc = "Field `ADDR` reader - CTR or DBR Address"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - CTR or DBR Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Target Location Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbselect {
    #[doc = "0: Selects CTR"]
    Ctr = 0,
    #[doc = "1: Selects DBR"]
    Dbr = 1,
}
impl From<Tbselect> for bool {
    #[inline(always)]
    fn from(variant: Tbselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TB` reader - Target Location Bit"]
pub type TbR = crate::BitReader<Tbselect>;
impl TbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbselect {
        match self.bits {
            false => Tbselect::Ctr,
            true => Tbselect::Dbr,
        }
    }
    #[doc = "Selects CTR"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == Tbselect::Ctr
    }
    #[doc = "Selects DBR"]
    #[inline(always)]
    pub fn is_dbr(&self) -> bool {
        *self == Tbselect::Dbr
    }
}
#[doc = "Field `TB` writer - Target Location Bit"]
pub type TbW<'a, REG> = crate::BitWriter<'a, REG, Tbselect>;
impl<'a, REG> TbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects CTR"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(Tbselect::Ctr)
    }
    #[doc = "Selects DBR"]
    #[inline(always)]
    pub fn dbr(self) -> &'a mut crate::W<REG> {
        self.variant(Tbselect::Dbr)
    }
}
#[doc = "Field `WNR` reader - Write-Not-Read Selection"]
pub type WnrR = crate::BitReader;
#[doc = "Field `WNR` writer - Write-Not-Read Selection"]
pub type WnrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - CTR or DBR Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    pub fn tb(&self) -> TbR {
        TbR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    pub fn wnr(&self) -> WnrR {
        WnrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - CTR or DBR Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<MadrSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    pub fn tb(&mut self) -> TbW<MadrSpec> {
        TbW::new(self, 30)
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    pub fn wnr(&mut self) -> WnrW<MadrSpec> {
        WnrW::new(self, 31)
    }
}
#[doc = "MIF Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`madr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`madr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MadrSpec;
impl crate::RegisterSpec for MadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`madr::R`](R) reader structure"]
impl crate::Readable for MadrSpec {}
#[doc = "`write(|w| ..)` method takes [`madr::W`](W) writer structure"]
impl crate::Writable for MadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MADR to value 0"]
impl crate::Resettable for MadrSpec {}
