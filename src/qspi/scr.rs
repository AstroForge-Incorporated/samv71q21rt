#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCBR` reader - Serial Clock Baud Rate"]
pub type ScbrR = crate::FieldReader;
#[doc = "Field `SCBR` writer - Serial Clock Baud Rate"]
pub type ScbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYBS` reader - Delay Before QSCK"]
pub type DlybsR = crate::FieldReader;
#[doc = "Field `DLYBS` writer - Delay Before QSCK"]
pub type DlybsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> ScbrR {
        ScbrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before QSCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DlybsR {
        DlybsR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<ScrSpec> {
        CpolW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<ScrSpec> {
        CphaW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&mut self) -> ScbrW<ScrSpec> {
        ScbrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay Before QSCK"]
    #[inline(always)]
    pub fn dlybs(&mut self) -> DlybsW<ScrSpec> {
        DlybsW::new(self, 16)
    }
}
#[doc = "Serial Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
