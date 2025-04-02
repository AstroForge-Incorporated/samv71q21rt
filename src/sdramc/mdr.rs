#[doc = "Register `MDR` reader"]
pub type R = crate::R<MdrSpec>;
#[doc = "Register `MDR` writer"]
pub type W = crate::W<MdrSpec>;
#[doc = "Memory Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdselect {
    #[doc = "0: SDRAM"]
    Sdram = 0,
    #[doc = "1: Low-power SDRAM"]
    Lpsdram = 1,
}
impl From<Mdselect> for u8 {
    #[inline(always)]
    fn from(variant: Mdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdselect {
    type Ux = u8;
}
impl crate::IsEnum for Mdselect {}
#[doc = "Field `MD` reader - Memory Device Type"]
pub type MdR = crate::FieldReader<Mdselect>;
impl MdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mdselect> {
        match self.bits {
            0 => Some(Mdselect::Sdram),
            1 => Some(Mdselect::Lpsdram),
            _ => None,
        }
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == Mdselect::Sdram
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn is_lpsdram(&self) -> bool {
        *self == Mdselect::Lpsdram
    }
}
#[doc = "Field `MD` writer - Memory Device Type"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mdselect>;
impl<'a, REG> MdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut crate::W<REG> {
        self.variant(Mdselect::Sdram)
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn lpsdram(self) -> &'a mut crate::W<REG> {
        self.variant(Mdselect::Lpsdram)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<MdrSpec> {
        MdW::new(self, 0)
    }
}
#[doc = "SDRAMC Memory Device Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdrSpec;
impl crate::RegisterSpec for MdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdr::R`](R) reader structure"]
impl crate::Readable for MdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdr::W`](W) writer structure"]
impl crate::Writable for MdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MdrSpec {}
