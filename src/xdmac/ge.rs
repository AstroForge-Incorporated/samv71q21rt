#[doc = "Register `GE` writer"]
pub type W = crate::W<GeSpec>;
#[doc = "Field `EN0` writer - XDMAC Channel 0 Enable Bit"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1` writer - XDMAC Channel 1 Enable Bit"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` writer - XDMAC Channel 2 Enable Bit"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN3` writer - XDMAC Channel 3 Enable Bit"]
pub type En3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN4` writer - XDMAC Channel 4 Enable Bit"]
pub type En4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN5` writer - XDMAC Channel 5 Enable Bit"]
pub type En5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN6` writer - XDMAC Channel 6 Enable Bit"]
pub type En6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN7` writer - XDMAC Channel 7 Enable Bit"]
pub type En7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN8` writer - XDMAC Channel 8 Enable Bit"]
pub type En8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN9` writer - XDMAC Channel 9 Enable Bit"]
pub type En9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN10` writer - XDMAC Channel 10 Enable Bit"]
pub type En10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN11` writer - XDMAC Channel 11 Enable Bit"]
pub type En11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN12` writer - XDMAC Channel 12 Enable Bit"]
pub type En12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN13` writer - XDMAC Channel 13 Enable Bit"]
pub type En13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN14` writer - XDMAC Channel 14 Enable Bit"]
pub type En14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN15` writer - XDMAC Channel 15 Enable Bit"]
pub type En15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN16` writer - XDMAC Channel 16 Enable Bit"]
pub type En16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN17` writer - XDMAC Channel 17 Enable Bit"]
pub type En17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN18` writer - XDMAC Channel 18 Enable Bit"]
pub type En18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN19` writer - XDMAC Channel 19 Enable Bit"]
pub type En19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN20` writer - XDMAC Channel 20 Enable Bit"]
pub type En20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN21` writer - XDMAC Channel 21 Enable Bit"]
pub type En21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN22` writer - XDMAC Channel 22 Enable Bit"]
pub type En22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN23` writer - XDMAC Channel 23 Enable Bit"]
pub type En23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Enable Bit"]
    #[inline(always)]
    pub fn en0(&mut self) -> En0W<GeSpec> {
        En0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Enable Bit"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<GeSpec> {
        En1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Enable Bit"]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<GeSpec> {
        En2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Enable Bit"]
    #[inline(always)]
    pub fn en3(&mut self) -> En3W<GeSpec> {
        En3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Enable Bit"]
    #[inline(always)]
    pub fn en4(&mut self) -> En4W<GeSpec> {
        En4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Enable Bit"]
    #[inline(always)]
    pub fn en5(&mut self) -> En5W<GeSpec> {
        En5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Enable Bit"]
    #[inline(always)]
    pub fn en6(&mut self) -> En6W<GeSpec> {
        En6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Enable Bit"]
    #[inline(always)]
    pub fn en7(&mut self) -> En7W<GeSpec> {
        En7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Enable Bit"]
    #[inline(always)]
    pub fn en8(&mut self) -> En8W<GeSpec> {
        En8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Enable Bit"]
    #[inline(always)]
    pub fn en9(&mut self) -> En9W<GeSpec> {
        En9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Enable Bit"]
    #[inline(always)]
    pub fn en10(&mut self) -> En10W<GeSpec> {
        En10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Enable Bit"]
    #[inline(always)]
    pub fn en11(&mut self) -> En11W<GeSpec> {
        En11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Enable Bit"]
    #[inline(always)]
    pub fn en12(&mut self) -> En12W<GeSpec> {
        En12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Enable Bit"]
    #[inline(always)]
    pub fn en13(&mut self) -> En13W<GeSpec> {
        En13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Enable Bit"]
    #[inline(always)]
    pub fn en14(&mut self) -> En14W<GeSpec> {
        En14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Enable Bit"]
    #[inline(always)]
    pub fn en15(&mut self) -> En15W<GeSpec> {
        En15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Enable Bit"]
    #[inline(always)]
    pub fn en16(&mut self) -> En16W<GeSpec> {
        En16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Enable Bit"]
    #[inline(always)]
    pub fn en17(&mut self) -> En17W<GeSpec> {
        En17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Enable Bit"]
    #[inline(always)]
    pub fn en18(&mut self) -> En18W<GeSpec> {
        En18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Enable Bit"]
    #[inline(always)]
    pub fn en19(&mut self) -> En19W<GeSpec> {
        En19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Enable Bit"]
    #[inline(always)]
    pub fn en20(&mut self) -> En20W<GeSpec> {
        En20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Enable Bit"]
    #[inline(always)]
    pub fn en21(&mut self) -> En21W<GeSpec> {
        En21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Enable Bit"]
    #[inline(always)]
    pub fn en22(&mut self) -> En22W<GeSpec> {
        En22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Enable Bit"]
    #[inline(always)]
    pub fn en23(&mut self) -> En23W<GeSpec> {
        En23W::new(self, 23)
    }
}
#[doc = "Global Channel Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ge::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GeSpec;
impl crate::RegisterSpec for GeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ge::W`](W) writer structure"]
impl crate::Writable for GeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GE to value 0"]
impl crate::Resettable for GeSpec {}
