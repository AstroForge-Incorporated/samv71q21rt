#[doc = "Register `MRCR` reader"]
pub type R = crate::R<MrcrSpec>;
#[doc = "Register `MRCR` writer"]
pub type W = crate::W<MrcrSpec>;
#[doc = "Field `RCB0` reader - Remap Command Bit for Master 0"]
pub type Rcb0R = crate::BitReader;
#[doc = "Field `RCB0` writer - Remap Command Bit for Master 0"]
pub type Rcb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB1` reader - Remap Command Bit for Master 1"]
pub type Rcb1R = crate::BitReader;
#[doc = "Field `RCB1` writer - Remap Command Bit for Master 1"]
pub type Rcb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB2` reader - Remap Command Bit for Master 2"]
pub type Rcb2R = crate::BitReader;
#[doc = "Field `RCB2` writer - Remap Command Bit for Master 2"]
pub type Rcb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB3` reader - Remap Command Bit for Master 3"]
pub type Rcb3R = crate::BitReader;
#[doc = "Field `RCB3` writer - Remap Command Bit for Master 3"]
pub type Rcb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB4` reader - Remap Command Bit for Master 4"]
pub type Rcb4R = crate::BitReader;
#[doc = "Field `RCB4` writer - Remap Command Bit for Master 4"]
pub type Rcb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB5` reader - Remap Command Bit for Master 5"]
pub type Rcb5R = crate::BitReader;
#[doc = "Field `RCB5` writer - Remap Command Bit for Master 5"]
pub type Rcb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB6` reader - Remap Command Bit for Master 6"]
pub type Rcb6R = crate::BitReader;
#[doc = "Field `RCB6` writer - Remap Command Bit for Master 6"]
pub type Rcb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB7` reader - Remap Command Bit for Master 7"]
pub type Rcb7R = crate::BitReader;
#[doc = "Field `RCB7` writer - Remap Command Bit for Master 7"]
pub type Rcb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB8` reader - Remap Command Bit for Master 8"]
pub type Rcb8R = crate::BitReader;
#[doc = "Field `RCB8` writer - Remap Command Bit for Master 8"]
pub type Rcb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB9` reader - Remap Command Bit for Master 9"]
pub type Rcb9R = crate::BitReader;
#[doc = "Field `RCB9` writer - Remap Command Bit for Master 9"]
pub type Rcb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB10` reader - Remap Command Bit for Master 10"]
pub type Rcb10R = crate::BitReader;
#[doc = "Field `RCB10` writer - Remap Command Bit for Master 10"]
pub type Rcb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB11` reader - Remap Command Bit for Master 11"]
pub type Rcb11R = crate::BitReader;
#[doc = "Field `RCB11` writer - Remap Command Bit for Master 11"]
pub type Rcb11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB12` reader - Remap Command Bit for Master 12"]
pub type Rcb12R = crate::BitReader;
#[doc = "Field `RCB12` writer - Remap Command Bit for Master 12"]
pub type Rcb12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> Rcb0R {
        Rcb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> Rcb1R {
        Rcb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> Rcb2R {
        Rcb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> Rcb3R {
        Rcb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> Rcb4R {
        Rcb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> Rcb5R {
        Rcb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&self) -> Rcb6R {
        Rcb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&self) -> Rcb7R {
        Rcb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&self) -> Rcb8R {
        Rcb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&self) -> Rcb9R {
        Rcb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&self) -> Rcb10R {
        Rcb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&self) -> Rcb11R {
        Rcb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&self) -> Rcb12R {
        Rcb12R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&mut self) -> Rcb0W<MrcrSpec> {
        Rcb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&mut self) -> Rcb1W<MrcrSpec> {
        Rcb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&mut self) -> Rcb2W<MrcrSpec> {
        Rcb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&mut self) -> Rcb3W<MrcrSpec> {
        Rcb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&mut self) -> Rcb4W<MrcrSpec> {
        Rcb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&mut self) -> Rcb5W<MrcrSpec> {
        Rcb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&mut self) -> Rcb6W<MrcrSpec> {
        Rcb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&mut self) -> Rcb7W<MrcrSpec> {
        Rcb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&mut self) -> Rcb8W<MrcrSpec> {
        Rcb8W::new(self, 8)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&mut self) -> Rcb9W<MrcrSpec> {
        Rcb9W::new(self, 9)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&mut self) -> Rcb10W<MrcrSpec> {
        Rcb10W::new(self, 10)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&mut self) -> Rcb11W<MrcrSpec> {
        Rcb11W::new(self, 11)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&mut self) -> Rcb12W<MrcrSpec> {
        Rcb12W::new(self, 12)
    }
}
#[doc = "Master Remap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrcrSpec;
impl crate::RegisterSpec for MrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcr::R`](R) reader structure"]
impl crate::Readable for MrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcr::W`](W) writer structure"]
impl crate::Writable for MrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MrcrSpec {}
