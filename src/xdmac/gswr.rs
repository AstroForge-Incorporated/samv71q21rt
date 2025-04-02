#[doc = "Register `GSWR` writer"]
pub type W = crate::W<GswrSpec>;
#[doc = "Field `SWREQ0` writer - XDMAC Channel 0 Software Request Bit"]
pub type Swreq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ1` writer - XDMAC Channel 1 Software Request Bit"]
pub type Swreq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ2` writer - XDMAC Channel 2 Software Request Bit"]
pub type Swreq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ3` writer - XDMAC Channel 3 Software Request Bit"]
pub type Swreq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ4` writer - XDMAC Channel 4 Software Request Bit"]
pub type Swreq4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ5` writer - XDMAC Channel 5 Software Request Bit"]
pub type Swreq5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ6` writer - XDMAC Channel 6 Software Request Bit"]
pub type Swreq6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ7` writer - XDMAC Channel 7 Software Request Bit"]
pub type Swreq7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ8` writer - XDMAC Channel 8 Software Request Bit"]
pub type Swreq8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ9` writer - XDMAC Channel 9 Software Request Bit"]
pub type Swreq9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ10` writer - XDMAC Channel 10 Software Request Bit"]
pub type Swreq10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ11` writer - XDMAC Channel 11 Software Request Bit"]
pub type Swreq11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ12` writer - XDMAC Channel 12 Software Request Bit"]
pub type Swreq12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ13` writer - XDMAC Channel 13 Software Request Bit"]
pub type Swreq13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ14` writer - XDMAC Channel 14 Software Request Bit"]
pub type Swreq14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ15` writer - XDMAC Channel 15 Software Request Bit"]
pub type Swreq15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ16` writer - XDMAC Channel 16 Software Request Bit"]
pub type Swreq16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ17` writer - XDMAC Channel 17 Software Request Bit"]
pub type Swreq17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ18` writer - XDMAC Channel 18 Software Request Bit"]
pub type Swreq18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ19` writer - XDMAC Channel 19 Software Request Bit"]
pub type Swreq19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ20` writer - XDMAC Channel 20 Software Request Bit"]
pub type Swreq20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ21` writer - XDMAC Channel 21 Software Request Bit"]
pub type Swreq21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ22` writer - XDMAC Channel 22 Software Request Bit"]
pub type Swreq22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQ23` writer - XDMAC Channel 23 Software Request Bit"]
pub type Swreq23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Bit"]
    #[inline(always)]
    pub fn swreq0(&mut self) -> Swreq0W<GswrSpec> {
        Swreq0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Bit"]
    #[inline(always)]
    pub fn swreq1(&mut self) -> Swreq1W<GswrSpec> {
        Swreq1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Bit"]
    #[inline(always)]
    pub fn swreq2(&mut self) -> Swreq2W<GswrSpec> {
        Swreq2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Bit"]
    #[inline(always)]
    pub fn swreq3(&mut self) -> Swreq3W<GswrSpec> {
        Swreq3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Bit"]
    #[inline(always)]
    pub fn swreq4(&mut self) -> Swreq4W<GswrSpec> {
        Swreq4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Bit"]
    #[inline(always)]
    pub fn swreq5(&mut self) -> Swreq5W<GswrSpec> {
        Swreq5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Bit"]
    #[inline(always)]
    pub fn swreq6(&mut self) -> Swreq6W<GswrSpec> {
        Swreq6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Bit"]
    #[inline(always)]
    pub fn swreq7(&mut self) -> Swreq7W<GswrSpec> {
        Swreq7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Bit"]
    #[inline(always)]
    pub fn swreq8(&mut self) -> Swreq8W<GswrSpec> {
        Swreq8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Bit"]
    #[inline(always)]
    pub fn swreq9(&mut self) -> Swreq9W<GswrSpec> {
        Swreq9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Bit"]
    #[inline(always)]
    pub fn swreq10(&mut self) -> Swreq10W<GswrSpec> {
        Swreq10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Bit"]
    #[inline(always)]
    pub fn swreq11(&mut self) -> Swreq11W<GswrSpec> {
        Swreq11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Bit"]
    #[inline(always)]
    pub fn swreq12(&mut self) -> Swreq12W<GswrSpec> {
        Swreq12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Bit"]
    #[inline(always)]
    pub fn swreq13(&mut self) -> Swreq13W<GswrSpec> {
        Swreq13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Bit"]
    #[inline(always)]
    pub fn swreq14(&mut self) -> Swreq14W<GswrSpec> {
        Swreq14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Bit"]
    #[inline(always)]
    pub fn swreq15(&mut self) -> Swreq15W<GswrSpec> {
        Swreq15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Bit"]
    #[inline(always)]
    pub fn swreq16(&mut self) -> Swreq16W<GswrSpec> {
        Swreq16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Bit"]
    #[inline(always)]
    pub fn swreq17(&mut self) -> Swreq17W<GswrSpec> {
        Swreq17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Bit"]
    #[inline(always)]
    pub fn swreq18(&mut self) -> Swreq18W<GswrSpec> {
        Swreq18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Bit"]
    #[inline(always)]
    pub fn swreq19(&mut self) -> Swreq19W<GswrSpec> {
        Swreq19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Bit"]
    #[inline(always)]
    pub fn swreq20(&mut self) -> Swreq20W<GswrSpec> {
        Swreq20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Bit"]
    #[inline(always)]
    pub fn swreq21(&mut self) -> Swreq21W<GswrSpec> {
        Swreq21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Bit"]
    #[inline(always)]
    pub fn swreq22(&mut self) -> Swreq22W<GswrSpec> {
        Swreq22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Bit"]
    #[inline(always)]
    pub fn swreq23(&mut self) -> Swreq23W<GswrSpec> {
        Swreq23W::new(self, 23)
    }
}
#[doc = "Global Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gswr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GswrSpec;
impl crate::RegisterSpec for GswrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gswr::W`](W) writer structure"]
impl crate::Writable for GswrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSWR to value 0"]
impl crate::Resettable for GswrSpec {}
