#[doc = "Register `DTOR` reader"]
pub type R = crate::R<DtorSpec>;
#[doc = "Register `DTOR` writer"]
pub type W = crate::W<DtorSpec>;
#[doc = "Field `DTOCYC` reader - Data Timeout Cycle Number"]
pub type DtocycR = crate::FieldReader;
#[doc = "Field `DTOCYC` writer - Data Timeout Cycle Number"]
pub type DtocycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtomulselect {
    #[doc = "0: DTOCYC"]
    _1 = 0,
    #[doc = "1: DTOCYC x 16"]
    _16 = 1,
    #[doc = "2: DTOCYC x 128"]
    _128 = 2,
    #[doc = "3: DTOCYC x 256"]
    _256 = 3,
    #[doc = "4: DTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: DTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: DTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: DTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<Dtomulselect> for u8 {
    #[inline(always)]
    fn from(variant: Dtomulselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtomulselect {
    type Ux = u8;
}
impl crate::IsEnum for Dtomulselect {}
#[doc = "Field `DTOMUL` reader - Data Timeout Multiplier"]
pub type DtomulR = crate::FieldReader<Dtomulselect>;
impl DtomulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtomulselect {
        match self.bits {
            0 => Dtomulselect::_1,
            1 => Dtomulselect::_16,
            2 => Dtomulselect::_128,
            3 => Dtomulselect::_256,
            4 => Dtomulselect::_1024,
            5 => Dtomulselect::_4096,
            6 => Dtomulselect::_65536,
            7 => Dtomulselect::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtomulselect::_1
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Dtomulselect::_16
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Dtomulselect::_128
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Dtomulselect::_256
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Dtomulselect::_1024
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == Dtomulselect::_4096
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == Dtomulselect::_65536
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == Dtomulselect::_1048576
    }
}
#[doc = "Field `DTOMUL` writer - Data Timeout Multiplier"]
pub type DtomulW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtomulselect, crate::Safe>;
impl<'a, REG> DtomulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomulselect::_1048576)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DtocycR {
        DtocycR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DtomulR {
        DtomulR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&mut self) -> DtocycW<DtorSpec> {
        DtocycW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&mut self) -> DtomulW<DtorSpec> {
        DtomulW::new(self, 4)
    }
}
#[doc = "Data Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtorSpec;
impl crate::RegisterSpec for DtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtor::R`](R) reader structure"]
impl crate::Readable for DtorSpec {}
#[doc = "`write(|w| ..)` method takes [`dtor::W`](W) writer structure"]
impl crate::Writable for DtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTOR to value 0"]
impl crate::Resettable for DtorSpec {}
