#[doc = "Register `GWAC` reader"]
pub type R = crate::R<GwacSpec>;
#[doc = "Register `GWAC` writer"]
pub type W = crate::W<GwacSpec>;
#[doc = "Field `PW0` reader - Pool Weight 0"]
pub type Pw0R = crate::FieldReader;
#[doc = "Field `PW0` writer - Pool Weight 0"]
pub type Pw0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PW1` reader - Pool Weight 1"]
pub type Pw1R = crate::FieldReader;
#[doc = "Field `PW1` writer - Pool Weight 1"]
pub type Pw1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PW2` reader - Pool Weight 2"]
pub type Pw2R = crate::FieldReader;
#[doc = "Field `PW2` writer - Pool Weight 2"]
pub type Pw2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PW3` reader - Pool Weight 3"]
pub type Pw3R = crate::FieldReader;
#[doc = "Field `PW3` writer - Pool Weight 3"]
pub type Pw3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&self) -> Pw0R {
        Pw0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&self) -> Pw1R {
        Pw1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&self) -> Pw2R {
        Pw2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&self) -> Pw3R {
        Pw3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&mut self) -> Pw0W<GwacSpec> {
        Pw0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&mut self) -> Pw1W<GwacSpec> {
        Pw1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&mut self) -> Pw2W<GwacSpec> {
        Pw2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&mut self) -> Pw3W<GwacSpec> {
        Pw3W::new(self, 12)
    }
}
#[doc = "Global Weighted Arbiter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gwac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gwac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GwacSpec;
impl crate::RegisterSpec for GwacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gwac::R`](R) reader structure"]
impl crate::Readable for GwacSpec {}
#[doc = "`write(|w| ..)` method takes [`gwac::W`](W) writer structure"]
impl crate::Writable for GwacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GWAC to value 0"]
impl crate::Resettable for GwacSpec {}
