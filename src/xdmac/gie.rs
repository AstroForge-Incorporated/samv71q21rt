#[doc = "Register `GIE` writer"]
pub type W = crate::W<GieSpec>;
#[doc = "Field `IE0` writer - XDMAC Channel 0 Interrupt Enable Bit"]
pub type Ie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE1` writer - XDMAC Channel 1 Interrupt Enable Bit"]
pub type Ie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE2` writer - XDMAC Channel 2 Interrupt Enable Bit"]
pub type Ie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE3` writer - XDMAC Channel 3 Interrupt Enable Bit"]
pub type Ie3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE4` writer - XDMAC Channel 4 Interrupt Enable Bit"]
pub type Ie4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE5` writer - XDMAC Channel 5 Interrupt Enable Bit"]
pub type Ie5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE6` writer - XDMAC Channel 6 Interrupt Enable Bit"]
pub type Ie6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE7` writer - XDMAC Channel 7 Interrupt Enable Bit"]
pub type Ie7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE8` writer - XDMAC Channel 8 Interrupt Enable Bit"]
pub type Ie8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE9` writer - XDMAC Channel 9 Interrupt Enable Bit"]
pub type Ie9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE10` writer - XDMAC Channel 10 Interrupt Enable Bit"]
pub type Ie10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE11` writer - XDMAC Channel 11 Interrupt Enable Bit"]
pub type Ie11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE12` writer - XDMAC Channel 12 Interrupt Enable Bit"]
pub type Ie12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE13` writer - XDMAC Channel 13 Interrupt Enable Bit"]
pub type Ie13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE14` writer - XDMAC Channel 14 Interrupt Enable Bit"]
pub type Ie14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE15` writer - XDMAC Channel 15 Interrupt Enable Bit"]
pub type Ie15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE16` writer - XDMAC Channel 16 Interrupt Enable Bit"]
pub type Ie16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE17` writer - XDMAC Channel 17 Interrupt Enable Bit"]
pub type Ie17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE18` writer - XDMAC Channel 18 Interrupt Enable Bit"]
pub type Ie18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE19` writer - XDMAC Channel 19 Interrupt Enable Bit"]
pub type Ie19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE20` writer - XDMAC Channel 20 Interrupt Enable Bit"]
pub type Ie20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE21` writer - XDMAC Channel 21 Interrupt Enable Bit"]
pub type Ie21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE22` writer - XDMAC Channel 22 Interrupt Enable Bit"]
pub type Ie22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE23` writer - XDMAC Channel 23 Interrupt Enable Bit"]
pub type Ie23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie0(&mut self) -> Ie0W<GieSpec> {
        Ie0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie1(&mut self) -> Ie1W<GieSpec> {
        Ie1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie2(&mut self) -> Ie2W<GieSpec> {
        Ie2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie3(&mut self) -> Ie3W<GieSpec> {
        Ie3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie4(&mut self) -> Ie4W<GieSpec> {
        Ie4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie5(&mut self) -> Ie5W<GieSpec> {
        Ie5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie6(&mut self) -> Ie6W<GieSpec> {
        Ie6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie7(&mut self) -> Ie7W<GieSpec> {
        Ie7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie8(&mut self) -> Ie8W<GieSpec> {
        Ie8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie9(&mut self) -> Ie9W<GieSpec> {
        Ie9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie10(&mut self) -> Ie10W<GieSpec> {
        Ie10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie11(&mut self) -> Ie11W<GieSpec> {
        Ie11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie12(&mut self) -> Ie12W<GieSpec> {
        Ie12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie13(&mut self) -> Ie13W<GieSpec> {
        Ie13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie14(&mut self) -> Ie14W<GieSpec> {
        Ie14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie15(&mut self) -> Ie15W<GieSpec> {
        Ie15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie16(&mut self) -> Ie16W<GieSpec> {
        Ie16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie17(&mut self) -> Ie17W<GieSpec> {
        Ie17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie18(&mut self) -> Ie18W<GieSpec> {
        Ie18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie19(&mut self) -> Ie19W<GieSpec> {
        Ie19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie20(&mut self) -> Ie20W<GieSpec> {
        Ie20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie21(&mut self) -> Ie21W<GieSpec> {
        Ie21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie22(&mut self) -> Ie22W<GieSpec> {
        Ie22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie23(&mut self) -> Ie23W<GieSpec> {
        Ie23W::new(self, 23)
    }
}
#[doc = "Global Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gie::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GieSpec;
impl crate::RegisterSpec for GieSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gie::W`](W) writer structure"]
impl crate::Writable for GieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GIE to value 0"]
impl crate::Resettable for GieSpec {}
