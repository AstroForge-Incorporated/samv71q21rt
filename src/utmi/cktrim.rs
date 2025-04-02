#[doc = "Register `CKTRIM` reader"]
pub type R = crate::R<CktrimSpec>;
#[doc = "Register `CKTRIM` writer"]
pub type W = crate::W<CktrimSpec>;
#[doc = "UTMI Reference Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freqselect {
    #[doc = "0: 12 MHz reference clock"]
    Xtal12 = 0,
    #[doc = "1: 16 MHz reference clock"]
    Xtal16 = 1,
}
impl From<Freqselect> for u8 {
    #[inline(always)]
    fn from(variant: Freqselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freqselect {
    type Ux = u8;
}
impl crate::IsEnum for Freqselect {}
#[doc = "Field `FREQ` reader - UTMI Reference Clock Frequency"]
pub type FreqR = crate::FieldReader<Freqselect>;
impl FreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freqselect> {
        match self.bits {
            0 => Some(Freqselect::Xtal12),
            1 => Some(Freqselect::Xtal16),
            _ => None,
        }
    }
    #[doc = "12 MHz reference clock"]
    #[inline(always)]
    pub fn is_xtal12(&self) -> bool {
        *self == Freqselect::Xtal12
    }
    #[doc = "16 MHz reference clock"]
    #[inline(always)]
    pub fn is_xtal16(&self) -> bool {
        *self == Freqselect::Xtal16
    }
}
#[doc = "Field `FREQ` writer - UTMI Reference Clock Frequency"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freqselect>;
impl<'a, REG> FreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12 MHz reference clock"]
    #[inline(always)]
    pub fn xtal12(self) -> &'a mut crate::W<REG> {
        self.variant(Freqselect::Xtal12)
    }
    #[doc = "16 MHz reference clock"]
    #[inline(always)]
    pub fn xtal16(self) -> &'a mut crate::W<REG> {
        self.variant(Freqselect::Xtal16)
    }
}
impl R {
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<CktrimSpec> {
        FreqW::new(self, 0)
    }
}
#[doc = "UTMI Clock Trimming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cktrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cktrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CktrimSpec;
impl crate::RegisterSpec for CktrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cktrim::R`](R) reader structure"]
impl crate::Readable for CktrimSpec {}
#[doc = "`write(|w| ..)` method takes [`cktrim::W`](W) writer structure"]
impl crate::Writable for CktrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKTRIM to value 0"]
impl crate::Resettable for CktrimSpec {}
