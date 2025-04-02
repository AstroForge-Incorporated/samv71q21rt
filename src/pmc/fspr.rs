#[doc = "Register `FSPR` reader"]
pub type R = crate::R<FsprSpec>;
#[doc = "Register `FSPR` writer"]
pub type W = crate::W<FsprSpec>;
#[doc = "Field `FSTP0` reader - Fast Startup Input Polarity 0"]
pub type Fstp0R = crate::BitReader;
#[doc = "Field `FSTP0` writer - Fast Startup Input Polarity 0"]
pub type Fstp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP1` reader - Fast Startup Input Polarity 1"]
pub type Fstp1R = crate::BitReader;
#[doc = "Field `FSTP1` writer - Fast Startup Input Polarity 1"]
pub type Fstp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP2` reader - Fast Startup Input Polarity 2"]
pub type Fstp2R = crate::BitReader;
#[doc = "Field `FSTP2` writer - Fast Startup Input Polarity 2"]
pub type Fstp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP3` reader - Fast Startup Input Polarity 3"]
pub type Fstp3R = crate::BitReader;
#[doc = "Field `FSTP3` writer - Fast Startup Input Polarity 3"]
pub type Fstp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP4` reader - Fast Startup Input Polarity 4"]
pub type Fstp4R = crate::BitReader;
#[doc = "Field `FSTP4` writer - Fast Startup Input Polarity 4"]
pub type Fstp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP5` reader - Fast Startup Input Polarity 5"]
pub type Fstp5R = crate::BitReader;
#[doc = "Field `FSTP5` writer - Fast Startup Input Polarity 5"]
pub type Fstp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP6` reader - Fast Startup Input Polarity 6"]
pub type Fstp6R = crate::BitReader;
#[doc = "Field `FSTP6` writer - Fast Startup Input Polarity 6"]
pub type Fstp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP7` reader - Fast Startup Input Polarity 7"]
pub type Fstp7R = crate::BitReader;
#[doc = "Field `FSTP7` writer - Fast Startup Input Polarity 7"]
pub type Fstp7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP8` reader - Fast Startup Input Polarity 8"]
pub type Fstp8R = crate::BitReader;
#[doc = "Field `FSTP8` writer - Fast Startup Input Polarity 8"]
pub type Fstp8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP9` reader - Fast Startup Input Polarity 9"]
pub type Fstp9R = crate::BitReader;
#[doc = "Field `FSTP9` writer - Fast Startup Input Polarity 9"]
pub type Fstp9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP10` reader - Fast Startup Input Polarity 10"]
pub type Fstp10R = crate::BitReader;
#[doc = "Field `FSTP10` writer - Fast Startup Input Polarity 10"]
pub type Fstp10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP11` reader - Fast Startup Input Polarity 11"]
pub type Fstp11R = crate::BitReader;
#[doc = "Field `FSTP11` writer - Fast Startup Input Polarity 11"]
pub type Fstp11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP12` reader - Fast Startup Input Polarity 12"]
pub type Fstp12R = crate::BitReader;
#[doc = "Field `FSTP12` writer - Fast Startup Input Polarity 12"]
pub type Fstp12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP13` reader - Fast Startup Input Polarity 13"]
pub type Fstp13R = crate::BitReader;
#[doc = "Field `FSTP13` writer - Fast Startup Input Polarity 13"]
pub type Fstp13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP14` reader - Fast Startup Input Polarity 14"]
pub type Fstp14R = crate::BitReader;
#[doc = "Field `FSTP14` writer - Fast Startup Input Polarity 14"]
pub type Fstp14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP15` reader - Fast Startup Input Polarity 15"]
pub type Fstp15R = crate::BitReader;
#[doc = "Field `FSTP15` writer - Fast Startup Input Polarity 15"]
pub type Fstp15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&self) -> Fstp0R {
        Fstp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&self) -> Fstp1R {
        Fstp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&self) -> Fstp2R {
        Fstp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&self) -> Fstp3R {
        Fstp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&self) -> Fstp4R {
        Fstp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&self) -> Fstp5R {
        Fstp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&self) -> Fstp6R {
        Fstp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&self) -> Fstp7R {
        Fstp7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&self) -> Fstp8R {
        Fstp8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&self) -> Fstp9R {
        Fstp9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&self) -> Fstp10R {
        Fstp10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&self) -> Fstp11R {
        Fstp11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&self) -> Fstp12R {
        Fstp12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&self) -> Fstp13R {
        Fstp13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&self) -> Fstp14R {
        Fstp14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&self) -> Fstp15R {
        Fstp15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&mut self) -> Fstp0W<FsprSpec> {
        Fstp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&mut self) -> Fstp1W<FsprSpec> {
        Fstp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&mut self) -> Fstp2W<FsprSpec> {
        Fstp2W::new(self, 2)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&mut self) -> Fstp3W<FsprSpec> {
        Fstp3W::new(self, 3)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&mut self) -> Fstp4W<FsprSpec> {
        Fstp4W::new(self, 4)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&mut self) -> Fstp5W<FsprSpec> {
        Fstp5W::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&mut self) -> Fstp6W<FsprSpec> {
        Fstp6W::new(self, 6)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&mut self) -> Fstp7W<FsprSpec> {
        Fstp7W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&mut self) -> Fstp8W<FsprSpec> {
        Fstp8W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&mut self) -> Fstp9W<FsprSpec> {
        Fstp9W::new(self, 9)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&mut self) -> Fstp10W<FsprSpec> {
        Fstp10W::new(self, 10)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&mut self) -> Fstp11W<FsprSpec> {
        Fstp11W::new(self, 11)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&mut self) -> Fstp12W<FsprSpec> {
        Fstp12W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&mut self) -> Fstp13W<FsprSpec> {
        Fstp13W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&mut self) -> Fstp14W<FsprSpec> {
        Fstp14W::new(self, 14)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&mut self) -> Fstp15W<FsprSpec> {
        Fstp15W::new(self, 15)
    }
}
#[doc = "Fast Startup Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fspr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fspr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsprSpec;
impl crate::RegisterSpec for FsprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fspr::R`](R) reader structure"]
impl crate::Readable for FsprSpec {}
#[doc = "`write(|w| ..)` method takes [`fspr::W`](W) writer structure"]
impl crate::Writable for FsprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSPR to value 0"]
impl crate::Resettable for FsprSpec {}
