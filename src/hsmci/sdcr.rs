#[doc = "Register `SDCR` reader"]
pub type R = crate::R<SdcrSpec>;
#[doc = "Register `SDCR` writer"]
pub type W = crate::W<SdcrSpec>;
#[doc = "SDCard/SDIO Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdcselselect {
    #[doc = "0: Slot A is selected."]
    Slota = 0,
}
impl From<Sdcselselect> for u8 {
    #[inline(always)]
    fn from(variant: Sdcselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdcselselect {
    type Ux = u8;
}
impl crate::IsEnum for Sdcselselect {}
#[doc = "Field `SDCSEL` reader - SDCard/SDIO Slot"]
pub type SdcselR = crate::FieldReader<Sdcselselect>;
impl SdcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdcselselect> {
        match self.bits {
            0 => Some(Sdcselselect::Slota),
            _ => None,
        }
    }
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn is_slota(&self) -> bool {
        *self == Sdcselselect::Slota
    }
}
#[doc = "Field `SDCSEL` writer - SDCard/SDIO Slot"]
pub type SdcselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdcselselect>;
impl<'a, REG> SdcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn slota(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcselselect::Slota)
    }
}
#[doc = "SDCard/SDIO Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdcbusselect {
    #[doc = "0: 1 bit"]
    _1 = 0,
    #[doc = "2: 4 bits"]
    _4 = 2,
    #[doc = "3: 8 bits"]
    _8 = 3,
}
impl From<Sdcbusselect> for u8 {
    #[inline(always)]
    fn from(variant: Sdcbusselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdcbusselect {
    type Ux = u8;
}
impl crate::IsEnum for Sdcbusselect {}
#[doc = "Field `SDCBUS` reader - SDCard/SDIO Bus Width"]
pub type SdcbusR = crate::FieldReader<Sdcbusselect>;
impl SdcbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdcbusselect> {
        match self.bits {
            0 => Some(Sdcbusselect::_1),
            2 => Some(Sdcbusselect::_4),
            3 => Some(Sdcbusselect::_8),
            _ => None,
        }
    }
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcbusselect::_1
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Sdcbusselect::_4
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Sdcbusselect::_8
    }
}
#[doc = "Field `SDCBUS` writer - SDCard/SDIO Bus Width"]
pub type SdcbusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdcbusselect>;
impl<'a, REG> SdcbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcbusselect::_1)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcbusselect::_4)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcbusselect::_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&self) -> SdcselR {
        SdcselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&self) -> SdcbusR {
        SdcbusR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&mut self) -> SdcselW<SdcrSpec> {
        SdcselW::new(self, 0)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&mut self) -> SdcbusW<SdcrSpec> {
        SdcbusW::new(self, 6)
    }
}
#[doc = "SD/SDIO Card Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcrSpec;
impl crate::RegisterSpec for SdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcr::R`](R) reader structure"]
impl crate::Readable for SdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdcr::W`](W) writer structure"]
impl crate::Writable for SdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDCR to value 0"]
impl crate::Resettable for SdcrSpec {}
