#[doc = "Register `CECR` reader"]
pub type R = crate::R<CecrSpec>;
#[doc = "Register `CECR` writer"]
pub type W = crate::W<CecrSpec>;
#[doc = "Field `ECORR0` reader - Error Correction Enable for channel 0"]
pub type Ecorr0R = crate::BitReader;
#[doc = "Field `ECORR0` writer - Error Correction Enable for channel 0"]
pub type Ecorr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR1` reader - Error Correction Enable for channel 1"]
pub type Ecorr1R = crate::BitReader;
#[doc = "Field `ECORR1` writer - Error Correction Enable for channel 1"]
pub type Ecorr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR2` reader - Error Correction Enable for channel 2"]
pub type Ecorr2R = crate::BitReader;
#[doc = "Field `ECORR2` writer - Error Correction Enable for channel 2"]
pub type Ecorr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR3` reader - Error Correction Enable for channel 3"]
pub type Ecorr3R = crate::BitReader;
#[doc = "Field `ECORR3` writer - Error Correction Enable for channel 3"]
pub type Ecorr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR4` reader - Error Correction Enable for channel 4"]
pub type Ecorr4R = crate::BitReader;
#[doc = "Field `ECORR4` writer - Error Correction Enable for channel 4"]
pub type Ecorr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR5` reader - Error Correction Enable for channel 5"]
pub type Ecorr5R = crate::BitReader;
#[doc = "Field `ECORR5` writer - Error Correction Enable for channel 5"]
pub type Ecorr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR6` reader - Error Correction Enable for channel 6"]
pub type Ecorr6R = crate::BitReader;
#[doc = "Field `ECORR6` writer - Error Correction Enable for channel 6"]
pub type Ecorr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR7` reader - Error Correction Enable for channel 7"]
pub type Ecorr7R = crate::BitReader;
#[doc = "Field `ECORR7` writer - Error Correction Enable for channel 7"]
pub type Ecorr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR8` reader - Error Correction Enable for channel 8"]
pub type Ecorr8R = crate::BitReader;
#[doc = "Field `ECORR8` writer - Error Correction Enable for channel 8"]
pub type Ecorr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR9` reader - Error Correction Enable for channel 9"]
pub type Ecorr9R = crate::BitReader;
#[doc = "Field `ECORR9` writer - Error Correction Enable for channel 9"]
pub type Ecorr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR10` reader - Error Correction Enable for channel 10"]
pub type Ecorr10R = crate::BitReader;
#[doc = "Field `ECORR10` writer - Error Correction Enable for channel 10"]
pub type Ecorr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECORR11` reader - Error Correction Enable for channel 11"]
pub type Ecorr11R = crate::BitReader;
#[doc = "Field `ECORR11` writer - Error Correction Enable for channel 11"]
pub type Ecorr11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&self) -> Ecorr0R {
        Ecorr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&self) -> Ecorr1R {
        Ecorr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&self) -> Ecorr2R {
        Ecorr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&self) -> Ecorr3R {
        Ecorr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&self) -> Ecorr4R {
        Ecorr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&self) -> Ecorr5R {
        Ecorr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&self) -> Ecorr6R {
        Ecorr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&self) -> Ecorr7R {
        Ecorr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&self) -> Ecorr8R {
        Ecorr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&self) -> Ecorr9R {
        Ecorr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&self) -> Ecorr10R {
        Ecorr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&self) -> Ecorr11R {
        Ecorr11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&mut self) -> Ecorr0W<CecrSpec> {
        Ecorr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&mut self) -> Ecorr1W<CecrSpec> {
        Ecorr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&mut self) -> Ecorr2W<CecrSpec> {
        Ecorr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&mut self) -> Ecorr3W<CecrSpec> {
        Ecorr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&mut self) -> Ecorr4W<CecrSpec> {
        Ecorr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&mut self) -> Ecorr5W<CecrSpec> {
        Ecorr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&mut self) -> Ecorr6W<CecrSpec> {
        Ecorr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&mut self) -> Ecorr7W<CecrSpec> {
        Ecorr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&mut self) -> Ecorr8W<CecrSpec> {
        Ecorr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&mut self) -> Ecorr9W<CecrSpec> {
        Ecorr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&mut self) -> Ecorr10W<CecrSpec> {
        Ecorr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&mut self) -> Ecorr11W<CecrSpec> {
        Ecorr11W::new(self, 11)
    }
}
#[doc = "AFEC Channel Error Correction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecrSpec;
impl crate::RegisterSpec for CecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cecr::R`](R) reader structure"]
impl crate::Readable for CecrSpec {}
#[doc = "`write(|w| ..)` method takes [`cecr::W`](W) writer structure"]
impl crate::Writable for CecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CECR to value 0"]
impl crate::Resettable for CecrSpec {}
