#[doc = "Register `DIFFR` reader"]
pub type R = crate::R<DiffrSpec>;
#[doc = "Register `DIFFR` writer"]
pub type W = crate::W<DiffrSpec>;
#[doc = "Field `DIFF0` reader - Differential inputs for channel 0"]
pub type Diff0R = crate::BitReader;
#[doc = "Field `DIFF0` writer - Differential inputs for channel 0"]
pub type Diff0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF1` reader - Differential inputs for channel 1"]
pub type Diff1R = crate::BitReader;
#[doc = "Field `DIFF1` writer - Differential inputs for channel 1"]
pub type Diff1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF2` reader - Differential inputs for channel 2"]
pub type Diff2R = crate::BitReader;
#[doc = "Field `DIFF2` writer - Differential inputs for channel 2"]
pub type Diff2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF3` reader - Differential inputs for channel 3"]
pub type Diff3R = crate::BitReader;
#[doc = "Field `DIFF3` writer - Differential inputs for channel 3"]
pub type Diff3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF4` reader - Differential inputs for channel 4"]
pub type Diff4R = crate::BitReader;
#[doc = "Field `DIFF4` writer - Differential inputs for channel 4"]
pub type Diff4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF5` reader - Differential inputs for channel 5"]
pub type Diff5R = crate::BitReader;
#[doc = "Field `DIFF5` writer - Differential inputs for channel 5"]
pub type Diff5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF6` reader - Differential inputs for channel 6"]
pub type Diff6R = crate::BitReader;
#[doc = "Field `DIFF6` writer - Differential inputs for channel 6"]
pub type Diff6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF7` reader - Differential inputs for channel 7"]
pub type Diff7R = crate::BitReader;
#[doc = "Field `DIFF7` writer - Differential inputs for channel 7"]
pub type Diff7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF8` reader - Differential inputs for channel 8"]
pub type Diff8R = crate::BitReader;
#[doc = "Field `DIFF8` writer - Differential inputs for channel 8"]
pub type Diff8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF9` reader - Differential inputs for channel 9"]
pub type Diff9R = crate::BitReader;
#[doc = "Field `DIFF9` writer - Differential inputs for channel 9"]
pub type Diff9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF10` reader - Differential inputs for channel 10"]
pub type Diff10R = crate::BitReader;
#[doc = "Field `DIFF10` writer - Differential inputs for channel 10"]
pub type Diff10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF11` reader - Differential inputs for channel 11"]
pub type Diff11R = crate::BitReader;
#[doc = "Field `DIFF11` writer - Differential inputs for channel 11"]
pub type Diff11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&self) -> Diff0R {
        Diff0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&self) -> Diff1R {
        Diff1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&self) -> Diff2R {
        Diff2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&self) -> Diff3R {
        Diff3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&self) -> Diff4R {
        Diff4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&self) -> Diff5R {
        Diff5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&self) -> Diff6R {
        Diff6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&self) -> Diff7R {
        Diff7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&self) -> Diff8R {
        Diff8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&self) -> Diff9R {
        Diff9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&self) -> Diff10R {
        Diff10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&self) -> Diff11R {
        Diff11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&mut self) -> Diff0W<DiffrSpec> {
        Diff0W::new(self, 0)
    }
    #[doc = "Bit 1 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&mut self) -> Diff1W<DiffrSpec> {
        Diff1W::new(self, 1)
    }
    #[doc = "Bit 2 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&mut self) -> Diff2W<DiffrSpec> {
        Diff2W::new(self, 2)
    }
    #[doc = "Bit 3 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&mut self) -> Diff3W<DiffrSpec> {
        Diff3W::new(self, 3)
    }
    #[doc = "Bit 4 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&mut self) -> Diff4W<DiffrSpec> {
        Diff4W::new(self, 4)
    }
    #[doc = "Bit 5 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&mut self) -> Diff5W<DiffrSpec> {
        Diff5W::new(self, 5)
    }
    #[doc = "Bit 6 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&mut self) -> Diff6W<DiffrSpec> {
        Diff6W::new(self, 6)
    }
    #[doc = "Bit 7 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&mut self) -> Diff7W<DiffrSpec> {
        Diff7W::new(self, 7)
    }
    #[doc = "Bit 8 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&mut self) -> Diff8W<DiffrSpec> {
        Diff8W::new(self, 8)
    }
    #[doc = "Bit 9 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&mut self) -> Diff9W<DiffrSpec> {
        Diff9W::new(self, 9)
    }
    #[doc = "Bit 10 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&mut self) -> Diff10W<DiffrSpec> {
        Diff10W::new(self, 10)
    }
    #[doc = "Bit 11 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&mut self) -> Diff11W<DiffrSpec> {
        Diff11W::new(self, 11)
    }
}
#[doc = "AFEC Channel Differential Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiffrSpec;
impl crate::RegisterSpec for DiffrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diffr::R`](R) reader structure"]
impl crate::Readable for DiffrSpec {}
#[doc = "`write(|w| ..)` method takes [`diffr::W`](W) writer structure"]
impl crate::Writable for DiffrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIFFR to value 0"]
impl crate::Resettable for DiffrSpec {}
