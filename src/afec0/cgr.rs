#[doc = "Register `CGR` reader"]
pub type R = crate::R<CgrSpec>;
#[doc = "Register `CGR` writer"]
pub type W = crate::W<CgrSpec>;
#[doc = "Field `GAIN0` reader - Gain for Channel 0"]
pub type Gain0R = crate::FieldReader;
#[doc = "Field `GAIN0` writer - Gain for Channel 0"]
pub type Gain0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN1` reader - Gain for Channel 1"]
pub type Gain1R = crate::FieldReader;
#[doc = "Field `GAIN1` writer - Gain for Channel 1"]
pub type Gain1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN2` reader - Gain for Channel 2"]
pub type Gain2R = crate::FieldReader;
#[doc = "Field `GAIN2` writer - Gain for Channel 2"]
pub type Gain2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN3` reader - Gain for Channel 3"]
pub type Gain3R = crate::FieldReader;
#[doc = "Field `GAIN3` writer - Gain for Channel 3"]
pub type Gain3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN4` reader - Gain for Channel 4"]
pub type Gain4R = crate::FieldReader;
#[doc = "Field `GAIN4` writer - Gain for Channel 4"]
pub type Gain4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN5` reader - Gain for Channel 5"]
pub type Gain5R = crate::FieldReader;
#[doc = "Field `GAIN5` writer - Gain for Channel 5"]
pub type Gain5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN6` reader - Gain for Channel 6"]
pub type Gain6R = crate::FieldReader;
#[doc = "Field `GAIN6` writer - Gain for Channel 6"]
pub type Gain6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN7` reader - Gain for Channel 7"]
pub type Gain7R = crate::FieldReader;
#[doc = "Field `GAIN7` writer - Gain for Channel 7"]
pub type Gain7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN8` reader - Gain for Channel 8"]
pub type Gain8R = crate::FieldReader;
#[doc = "Field `GAIN8` writer - Gain for Channel 8"]
pub type Gain8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN9` reader - Gain for Channel 9"]
pub type Gain9R = crate::FieldReader;
#[doc = "Field `GAIN9` writer - Gain for Channel 9"]
pub type Gain9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN10` reader - Gain for Channel 10"]
pub type Gain10R = crate::FieldReader;
#[doc = "Field `GAIN10` writer - Gain for Channel 10"]
pub type Gain10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAIN11` reader - Gain for Channel 11"]
pub type Gain11R = crate::FieldReader;
#[doc = "Field `GAIN11` writer - Gain for Channel 11"]
pub type Gain11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&self) -> Gain0R {
        Gain0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&self) -> Gain1R {
        Gain1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&self) -> Gain2R {
        Gain2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&self) -> Gain3R {
        Gain3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&self) -> Gain4R {
        Gain4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&self) -> Gain5R {
        Gain5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&self) -> Gain6R {
        Gain6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&self) -> Gain7R {
        Gain7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&self) -> Gain8R {
        Gain8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&self) -> Gain9R {
        Gain9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&self) -> Gain10R {
        Gain10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&self) -> Gain11R {
        Gain11R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> Gain0W<CgrSpec> {
        Gain0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> Gain1W<CgrSpec> {
        Gain1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> Gain2W<CgrSpec> {
        Gain2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&mut self) -> Gain3W<CgrSpec> {
        Gain3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&mut self) -> Gain4W<CgrSpec> {
        Gain4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&mut self) -> Gain5W<CgrSpec> {
        Gain5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&mut self) -> Gain6W<CgrSpec> {
        Gain6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&mut self) -> Gain7W<CgrSpec> {
        Gain7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&mut self) -> Gain8W<CgrSpec> {
        Gain8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&mut self) -> Gain9W<CgrSpec> {
        Gain9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&mut self) -> Gain10W<CgrSpec> {
        Gain10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&mut self) -> Gain11W<CgrSpec> {
        Gain11W::new(self, 22)
    }
}
#[doc = "AFEC Channel Gain Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgrSpec;
impl crate::RegisterSpec for CgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgr::R`](R) reader structure"]
impl crate::Readable for CgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cgr::W`](W) writer structure"]
impl crate::Writable for CgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGR to value 0"]
impl crate::Resettable for CgrSpec {}
