#[doc = "Register `GD` writer"]
pub type W = crate::W<GdSpec>;
#[doc = "Field `DI0` writer - XDMAC Channel 0 Disable Bit"]
pub type Di0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI1` writer - XDMAC Channel 1 Disable Bit"]
pub type Di1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI2` writer - XDMAC Channel 2 Disable Bit"]
pub type Di2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI3` writer - XDMAC Channel 3 Disable Bit"]
pub type Di3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI4` writer - XDMAC Channel 4 Disable Bit"]
pub type Di4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI5` writer - XDMAC Channel 5 Disable Bit"]
pub type Di5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI6` writer - XDMAC Channel 6 Disable Bit"]
pub type Di6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI7` writer - XDMAC Channel 7 Disable Bit"]
pub type Di7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI8` writer - XDMAC Channel 8 Disable Bit"]
pub type Di8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI9` writer - XDMAC Channel 9 Disable Bit"]
pub type Di9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI10` writer - XDMAC Channel 10 Disable Bit"]
pub type Di10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI11` writer - XDMAC Channel 11 Disable Bit"]
pub type Di11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI12` writer - XDMAC Channel 12 Disable Bit"]
pub type Di12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI13` writer - XDMAC Channel 13 Disable Bit"]
pub type Di13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI14` writer - XDMAC Channel 14 Disable Bit"]
pub type Di14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI15` writer - XDMAC Channel 15 Disable Bit"]
pub type Di15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI16` writer - XDMAC Channel 16 Disable Bit"]
pub type Di16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI17` writer - XDMAC Channel 17 Disable Bit"]
pub type Di17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI18` writer - XDMAC Channel 18 Disable Bit"]
pub type Di18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI19` writer - XDMAC Channel 19 Disable Bit"]
pub type Di19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI20` writer - XDMAC Channel 20 Disable Bit"]
pub type Di20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI21` writer - XDMAC Channel 21 Disable Bit"]
pub type Di21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI22` writer - XDMAC Channel 22 Disable Bit"]
pub type Di22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI23` writer - XDMAC Channel 23 Disable Bit"]
pub type Di23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Disable Bit"]
    #[inline(always)]
    pub fn di0(&mut self) -> Di0W<GdSpec> {
        Di0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Disable Bit"]
    #[inline(always)]
    pub fn di1(&mut self) -> Di1W<GdSpec> {
        Di1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Disable Bit"]
    #[inline(always)]
    pub fn di2(&mut self) -> Di2W<GdSpec> {
        Di2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Disable Bit"]
    #[inline(always)]
    pub fn di3(&mut self) -> Di3W<GdSpec> {
        Di3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Disable Bit"]
    #[inline(always)]
    pub fn di4(&mut self) -> Di4W<GdSpec> {
        Di4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Disable Bit"]
    #[inline(always)]
    pub fn di5(&mut self) -> Di5W<GdSpec> {
        Di5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Disable Bit"]
    #[inline(always)]
    pub fn di6(&mut self) -> Di6W<GdSpec> {
        Di6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Disable Bit"]
    #[inline(always)]
    pub fn di7(&mut self) -> Di7W<GdSpec> {
        Di7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Disable Bit"]
    #[inline(always)]
    pub fn di8(&mut self) -> Di8W<GdSpec> {
        Di8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Disable Bit"]
    #[inline(always)]
    pub fn di9(&mut self) -> Di9W<GdSpec> {
        Di9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Disable Bit"]
    #[inline(always)]
    pub fn di10(&mut self) -> Di10W<GdSpec> {
        Di10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Disable Bit"]
    #[inline(always)]
    pub fn di11(&mut self) -> Di11W<GdSpec> {
        Di11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Disable Bit"]
    #[inline(always)]
    pub fn di12(&mut self) -> Di12W<GdSpec> {
        Di12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Disable Bit"]
    #[inline(always)]
    pub fn di13(&mut self) -> Di13W<GdSpec> {
        Di13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Disable Bit"]
    #[inline(always)]
    pub fn di14(&mut self) -> Di14W<GdSpec> {
        Di14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Disable Bit"]
    #[inline(always)]
    pub fn di15(&mut self) -> Di15W<GdSpec> {
        Di15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Disable Bit"]
    #[inline(always)]
    pub fn di16(&mut self) -> Di16W<GdSpec> {
        Di16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Disable Bit"]
    #[inline(always)]
    pub fn di17(&mut self) -> Di17W<GdSpec> {
        Di17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Disable Bit"]
    #[inline(always)]
    pub fn di18(&mut self) -> Di18W<GdSpec> {
        Di18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Disable Bit"]
    #[inline(always)]
    pub fn di19(&mut self) -> Di19W<GdSpec> {
        Di19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Disable Bit"]
    #[inline(always)]
    pub fn di20(&mut self) -> Di20W<GdSpec> {
        Di20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Disable Bit"]
    #[inline(always)]
    pub fn di21(&mut self) -> Di21W<GdSpec> {
        Di21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Disable Bit"]
    #[inline(always)]
    pub fn di22(&mut self) -> Di22W<GdSpec> {
        Di22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Disable Bit"]
    #[inline(always)]
    pub fn di23(&mut self) -> Di23W<GdSpec> {
        Di23W::new(self, 23)
    }
}
#[doc = "Global Channel Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdSpec;
impl crate::RegisterSpec for GdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gd::W`](W) writer structure"]
impl crate::Writable for GdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GD to value 0"]
impl crate::Resettable for GdSpec {}
