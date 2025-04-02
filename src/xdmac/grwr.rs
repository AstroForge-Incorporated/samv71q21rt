#[doc = "Register `GRWR` writer"]
pub type W = crate::W<GrwrSpec>;
#[doc = "Field `RWR0` writer - XDMAC Channel 0 Read Write Resume Bit"]
pub type Rwr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR1` writer - XDMAC Channel 1 Read Write Resume Bit"]
pub type Rwr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR2` writer - XDMAC Channel 2 Read Write Resume Bit"]
pub type Rwr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR3` writer - XDMAC Channel 3 Read Write Resume Bit"]
pub type Rwr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR4` writer - XDMAC Channel 4 Read Write Resume Bit"]
pub type Rwr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR5` writer - XDMAC Channel 5 Read Write Resume Bit"]
pub type Rwr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR6` writer - XDMAC Channel 6 Read Write Resume Bit"]
pub type Rwr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR7` writer - XDMAC Channel 7 Read Write Resume Bit"]
pub type Rwr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR8` writer - XDMAC Channel 8 Read Write Resume Bit"]
pub type Rwr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR9` writer - XDMAC Channel 9 Read Write Resume Bit"]
pub type Rwr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR10` writer - XDMAC Channel 10 Read Write Resume Bit"]
pub type Rwr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR11` writer - XDMAC Channel 11 Read Write Resume Bit"]
pub type Rwr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR12` writer - XDMAC Channel 12 Read Write Resume Bit"]
pub type Rwr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR13` writer - XDMAC Channel 13 Read Write Resume Bit"]
pub type Rwr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR14` writer - XDMAC Channel 14 Read Write Resume Bit"]
pub type Rwr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR15` writer - XDMAC Channel 15 Read Write Resume Bit"]
pub type Rwr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR16` writer - XDMAC Channel 16 Read Write Resume Bit"]
pub type Rwr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR17` writer - XDMAC Channel 17 Read Write Resume Bit"]
pub type Rwr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR18` writer - XDMAC Channel 18 Read Write Resume Bit"]
pub type Rwr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR19` writer - XDMAC Channel 19 Read Write Resume Bit"]
pub type Rwr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR20` writer - XDMAC Channel 20 Read Write Resume Bit"]
pub type Rwr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR21` writer - XDMAC Channel 21 Read Write Resume Bit"]
pub type Rwr21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR22` writer - XDMAC Channel 22 Read Write Resume Bit"]
pub type Rwr22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWR23` writer - XDMAC Channel 23 Read Write Resume Bit"]
pub type Rwr23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr0(&mut self) -> Rwr0W<GrwrSpec> {
        Rwr0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr1(&mut self) -> Rwr1W<GrwrSpec> {
        Rwr1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr2(&mut self) -> Rwr2W<GrwrSpec> {
        Rwr2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr3(&mut self) -> Rwr3W<GrwrSpec> {
        Rwr3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr4(&mut self) -> Rwr4W<GrwrSpec> {
        Rwr4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr5(&mut self) -> Rwr5W<GrwrSpec> {
        Rwr5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr6(&mut self) -> Rwr6W<GrwrSpec> {
        Rwr6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr7(&mut self) -> Rwr7W<GrwrSpec> {
        Rwr7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr8(&mut self) -> Rwr8W<GrwrSpec> {
        Rwr8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr9(&mut self) -> Rwr9W<GrwrSpec> {
        Rwr9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr10(&mut self) -> Rwr10W<GrwrSpec> {
        Rwr10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr11(&mut self) -> Rwr11W<GrwrSpec> {
        Rwr11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr12(&mut self) -> Rwr12W<GrwrSpec> {
        Rwr12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr13(&mut self) -> Rwr13W<GrwrSpec> {
        Rwr13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr14(&mut self) -> Rwr14W<GrwrSpec> {
        Rwr14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr15(&mut self) -> Rwr15W<GrwrSpec> {
        Rwr15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr16(&mut self) -> Rwr16W<GrwrSpec> {
        Rwr16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr17(&mut self) -> Rwr17W<GrwrSpec> {
        Rwr17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr18(&mut self) -> Rwr18W<GrwrSpec> {
        Rwr18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr19(&mut self) -> Rwr19W<GrwrSpec> {
        Rwr19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr20(&mut self) -> Rwr20W<GrwrSpec> {
        Rwr20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr21(&mut self) -> Rwr21W<GrwrSpec> {
        Rwr21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr22(&mut self) -> Rwr22W<GrwrSpec> {
        Rwr22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr23(&mut self) -> Rwr23W<GrwrSpec> {
        Rwr23W::new(self, 23)
    }
}
#[doc = "Global Channel Read Write Resume Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrwrSpec;
impl crate::RegisterSpec for GrwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`grwr::W`](W) writer structure"]
impl crate::Writable for GrwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRWR to value 0"]
impl crate::Resettable for GrwrSpec {}
