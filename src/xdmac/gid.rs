#[doc = "Register `GID` writer"]
pub type W = crate::W<GidSpec>;
#[doc = "Field `ID0` writer - XDMAC Channel 0 Interrupt Disable Bit"]
pub type Id0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID1` writer - XDMAC Channel 1 Interrupt Disable Bit"]
pub type Id1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID2` writer - XDMAC Channel 2 Interrupt Disable Bit"]
pub type Id2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID3` writer - XDMAC Channel 3 Interrupt Disable Bit"]
pub type Id3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID4` writer - XDMAC Channel 4 Interrupt Disable Bit"]
pub type Id4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID5` writer - XDMAC Channel 5 Interrupt Disable Bit"]
pub type Id5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID6` writer - XDMAC Channel 6 Interrupt Disable Bit"]
pub type Id6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID7` writer - XDMAC Channel 7 Interrupt Disable Bit"]
pub type Id7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID8` writer - XDMAC Channel 8 Interrupt Disable Bit"]
pub type Id8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID9` writer - XDMAC Channel 9 Interrupt Disable Bit"]
pub type Id9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID10` writer - XDMAC Channel 10 Interrupt Disable Bit"]
pub type Id10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID11` writer - XDMAC Channel 11 Interrupt Disable Bit"]
pub type Id11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID12` writer - XDMAC Channel 12 Interrupt Disable Bit"]
pub type Id12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID13` writer - XDMAC Channel 13 Interrupt Disable Bit"]
pub type Id13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID14` writer - XDMAC Channel 14 Interrupt Disable Bit"]
pub type Id14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID15` writer - XDMAC Channel 15 Interrupt Disable Bit"]
pub type Id15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID16` writer - XDMAC Channel 16 Interrupt Disable Bit"]
pub type Id16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID17` writer - XDMAC Channel 17 Interrupt Disable Bit"]
pub type Id17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID18` writer - XDMAC Channel 18 Interrupt Disable Bit"]
pub type Id18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID19` writer - XDMAC Channel 19 Interrupt Disable Bit"]
pub type Id19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID20` writer - XDMAC Channel 20 Interrupt Disable Bit"]
pub type Id20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID21` writer - XDMAC Channel 21 Interrupt Disable Bit"]
pub type Id21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID22` writer - XDMAC Channel 22 Interrupt Disable Bit"]
pub type Id22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID23` writer - XDMAC Channel 23 Interrupt Disable Bit"]
pub type Id23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id0(&mut self) -> Id0W<GidSpec> {
        Id0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id1(&mut self) -> Id1W<GidSpec> {
        Id1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id2(&mut self) -> Id2W<GidSpec> {
        Id2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id3(&mut self) -> Id3W<GidSpec> {
        Id3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id4(&mut self) -> Id4W<GidSpec> {
        Id4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id5(&mut self) -> Id5W<GidSpec> {
        Id5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id6(&mut self) -> Id6W<GidSpec> {
        Id6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id7(&mut self) -> Id7W<GidSpec> {
        Id7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id8(&mut self) -> Id8W<GidSpec> {
        Id8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id9(&mut self) -> Id9W<GidSpec> {
        Id9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id10(&mut self) -> Id10W<GidSpec> {
        Id10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id11(&mut self) -> Id11W<GidSpec> {
        Id11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id12(&mut self) -> Id12W<GidSpec> {
        Id12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id13(&mut self) -> Id13W<GidSpec> {
        Id13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id14(&mut self) -> Id14W<GidSpec> {
        Id14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id15(&mut self) -> Id15W<GidSpec> {
        Id15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id16(&mut self) -> Id16W<GidSpec> {
        Id16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id17(&mut self) -> Id17W<GidSpec> {
        Id17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id18(&mut self) -> Id18W<GidSpec> {
        Id18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id19(&mut self) -> Id19W<GidSpec> {
        Id19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id20(&mut self) -> Id20W<GidSpec> {
        Id20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id21(&mut self) -> Id21W<GidSpec> {
        Id21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id22(&mut self) -> Id22W<GidSpec> {
        Id22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id23(&mut self) -> Id23W<GidSpec> {
        Id23W::new(self, 23)
    }
}
#[doc = "Global Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gid::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GidSpec;
impl crate::RegisterSpec for GidSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gid::W`](W) writer structure"]
impl crate::Writable for GidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GID to value 0"]
impl crate::Resettable for GidSpec {}
