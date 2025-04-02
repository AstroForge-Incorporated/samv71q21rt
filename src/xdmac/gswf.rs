#[doc = "Register `GSWF` writer"]
pub type W = crate::W<GswfSpec>;
#[doc = "Field `SWF0` writer - XDMAC Channel 0 Software Flush Request Bit"]
pub type Swf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF1` writer - XDMAC Channel 1 Software Flush Request Bit"]
pub type Swf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF2` writer - XDMAC Channel 2 Software Flush Request Bit"]
pub type Swf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF3` writer - XDMAC Channel 3 Software Flush Request Bit"]
pub type Swf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF4` writer - XDMAC Channel 4 Software Flush Request Bit"]
pub type Swf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF5` writer - XDMAC Channel 5 Software Flush Request Bit"]
pub type Swf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF6` writer - XDMAC Channel 6 Software Flush Request Bit"]
pub type Swf6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF7` writer - XDMAC Channel 7 Software Flush Request Bit"]
pub type Swf7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF8` writer - XDMAC Channel 8 Software Flush Request Bit"]
pub type Swf8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF9` writer - XDMAC Channel 9 Software Flush Request Bit"]
pub type Swf9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF10` writer - XDMAC Channel 10 Software Flush Request Bit"]
pub type Swf10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF11` writer - XDMAC Channel 11 Software Flush Request Bit"]
pub type Swf11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF12` writer - XDMAC Channel 12 Software Flush Request Bit"]
pub type Swf12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF13` writer - XDMAC Channel 13 Software Flush Request Bit"]
pub type Swf13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF14` writer - XDMAC Channel 14 Software Flush Request Bit"]
pub type Swf14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF15` writer - XDMAC Channel 15 Software Flush Request Bit"]
pub type Swf15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF16` writer - XDMAC Channel 16 Software Flush Request Bit"]
pub type Swf16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF17` writer - XDMAC Channel 17 Software Flush Request Bit"]
pub type Swf17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF18` writer - XDMAC Channel 18 Software Flush Request Bit"]
pub type Swf18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF19` writer - XDMAC Channel 19 Software Flush Request Bit"]
pub type Swf19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF20` writer - XDMAC Channel 20 Software Flush Request Bit"]
pub type Swf20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF21` writer - XDMAC Channel 21 Software Flush Request Bit"]
pub type Swf21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF22` writer - XDMAC Channel 22 Software Flush Request Bit"]
pub type Swf22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWF23` writer - XDMAC Channel 23 Software Flush Request Bit"]
pub type Swf23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf0(&mut self) -> Swf0W<GswfSpec> {
        Swf0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf1(&mut self) -> Swf1W<GswfSpec> {
        Swf1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf2(&mut self) -> Swf2W<GswfSpec> {
        Swf2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf3(&mut self) -> Swf3W<GswfSpec> {
        Swf3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf4(&mut self) -> Swf4W<GswfSpec> {
        Swf4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf5(&mut self) -> Swf5W<GswfSpec> {
        Swf5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf6(&mut self) -> Swf6W<GswfSpec> {
        Swf6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf7(&mut self) -> Swf7W<GswfSpec> {
        Swf7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf8(&mut self) -> Swf8W<GswfSpec> {
        Swf8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf9(&mut self) -> Swf9W<GswfSpec> {
        Swf9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf10(&mut self) -> Swf10W<GswfSpec> {
        Swf10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf11(&mut self) -> Swf11W<GswfSpec> {
        Swf11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf12(&mut self) -> Swf12W<GswfSpec> {
        Swf12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf13(&mut self) -> Swf13W<GswfSpec> {
        Swf13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf14(&mut self) -> Swf14W<GswfSpec> {
        Swf14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf15(&mut self) -> Swf15W<GswfSpec> {
        Swf15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf16(&mut self) -> Swf16W<GswfSpec> {
        Swf16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf17(&mut self) -> Swf17W<GswfSpec> {
        Swf17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf18(&mut self) -> Swf18W<GswfSpec> {
        Swf18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf19(&mut self) -> Swf19W<GswfSpec> {
        Swf19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf20(&mut self) -> Swf20W<GswfSpec> {
        Swf20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf21(&mut self) -> Swf21W<GswfSpec> {
        Swf21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf22(&mut self) -> Swf22W<GswfSpec> {
        Swf22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf23(&mut self) -> Swf23W<GswfSpec> {
        Swf23W::new(self, 23)
    }
}
#[doc = "Global Channel Software Flush Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gswf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GswfSpec;
impl crate::RegisterSpec for GswfSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gswf::W`](W) writer structure"]
impl crate::Writable for GswfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSWF to value 0"]
impl crate::Resettable for GswfSpec {}
