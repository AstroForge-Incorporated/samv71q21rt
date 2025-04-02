#[doc = "Register `SHMR` reader"]
pub type R = crate::R<ShmrSpec>;
#[doc = "Register `SHMR` writer"]
pub type W = crate::W<ShmrSpec>;
#[doc = "Field `DUAL0` reader - Dual Sample & Hold for channel 0"]
pub type Dual0R = crate::BitReader;
#[doc = "Field `DUAL0` writer - Dual Sample & Hold for channel 0"]
pub type Dual0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL1` reader - Dual Sample & Hold for channel 1"]
pub type Dual1R = crate::BitReader;
#[doc = "Field `DUAL1` writer - Dual Sample & Hold for channel 1"]
pub type Dual1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL2` reader - Dual Sample & Hold for channel 2"]
pub type Dual2R = crate::BitReader;
#[doc = "Field `DUAL2` writer - Dual Sample & Hold for channel 2"]
pub type Dual2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL3` reader - Dual Sample & Hold for channel 3"]
pub type Dual3R = crate::BitReader;
#[doc = "Field `DUAL3` writer - Dual Sample & Hold for channel 3"]
pub type Dual3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL4` reader - Dual Sample & Hold for channel 4"]
pub type Dual4R = crate::BitReader;
#[doc = "Field `DUAL4` writer - Dual Sample & Hold for channel 4"]
pub type Dual4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL5` reader - Dual Sample & Hold for channel 5"]
pub type Dual5R = crate::BitReader;
#[doc = "Field `DUAL5` writer - Dual Sample & Hold for channel 5"]
pub type Dual5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL6` reader - Dual Sample & Hold for channel 6"]
pub type Dual6R = crate::BitReader;
#[doc = "Field `DUAL6` writer - Dual Sample & Hold for channel 6"]
pub type Dual6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL7` reader - Dual Sample & Hold for channel 7"]
pub type Dual7R = crate::BitReader;
#[doc = "Field `DUAL7` writer - Dual Sample & Hold for channel 7"]
pub type Dual7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL8` reader - Dual Sample & Hold for channel 8"]
pub type Dual8R = crate::BitReader;
#[doc = "Field `DUAL8` writer - Dual Sample & Hold for channel 8"]
pub type Dual8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL9` reader - Dual Sample & Hold for channel 9"]
pub type Dual9R = crate::BitReader;
#[doc = "Field `DUAL9` writer - Dual Sample & Hold for channel 9"]
pub type Dual9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL10` reader - Dual Sample & Hold for channel 10"]
pub type Dual10R = crate::BitReader;
#[doc = "Field `DUAL10` writer - Dual Sample & Hold for channel 10"]
pub type Dual10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL11` reader - Dual Sample & Hold for channel 11"]
pub type Dual11R = crate::BitReader;
#[doc = "Field `DUAL11` writer - Dual Sample & Hold for channel 11"]
pub type Dual11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&self) -> Dual0R {
        Dual0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&self) -> Dual1R {
        Dual1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&self) -> Dual2R {
        Dual2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&self) -> Dual3R {
        Dual3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&self) -> Dual4R {
        Dual4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&self) -> Dual5R {
        Dual5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&self) -> Dual6R {
        Dual6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&self) -> Dual7R {
        Dual7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&self) -> Dual8R {
        Dual8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&self) -> Dual9R {
        Dual9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&self) -> Dual10R {
        Dual10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&self) -> Dual11R {
        Dual11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&mut self) -> Dual0W<ShmrSpec> {
        Dual0W::new(self, 0)
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&mut self) -> Dual1W<ShmrSpec> {
        Dual1W::new(self, 1)
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&mut self) -> Dual2W<ShmrSpec> {
        Dual2W::new(self, 2)
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&mut self) -> Dual3W<ShmrSpec> {
        Dual3W::new(self, 3)
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&mut self) -> Dual4W<ShmrSpec> {
        Dual4W::new(self, 4)
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&mut self) -> Dual5W<ShmrSpec> {
        Dual5W::new(self, 5)
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&mut self) -> Dual6W<ShmrSpec> {
        Dual6W::new(self, 6)
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&mut self) -> Dual7W<ShmrSpec> {
        Dual7W::new(self, 7)
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&mut self) -> Dual8W<ShmrSpec> {
        Dual8W::new(self, 8)
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&mut self) -> Dual9W<ShmrSpec> {
        Dual9W::new(self, 9)
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&mut self) -> Dual10W<ShmrSpec> {
        Dual10W::new(self, 10)
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&mut self) -> Dual11W<ShmrSpec> {
        Dual11W::new(self, 11)
    }
}
#[doc = "AFEC Sample & Hold Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShmrSpec;
impl crate::RegisterSpec for ShmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shmr::R`](R) reader structure"]
impl crate::Readable for ShmrSpec {}
#[doc = "`write(|w| ..)` method takes [`shmr::W`](W) writer structure"]
impl crate::Writable for ShmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHMR to value 0"]
impl crate::Resettable for ShmrSpec {}
