#[doc = "Register `GRWS` writer"]
pub type W = crate::W<GrwsSpec>;
#[doc = "Field `RWS0` writer - XDMAC Channel 0 Read Write Suspend Bit"]
pub type Rws0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS1` writer - XDMAC Channel 1 Read Write Suspend Bit"]
pub type Rws1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS2` writer - XDMAC Channel 2 Read Write Suspend Bit"]
pub type Rws2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS3` writer - XDMAC Channel 3 Read Write Suspend Bit"]
pub type Rws3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS4` writer - XDMAC Channel 4 Read Write Suspend Bit"]
pub type Rws4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS5` writer - XDMAC Channel 5 Read Write Suspend Bit"]
pub type Rws5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS6` writer - XDMAC Channel 6 Read Write Suspend Bit"]
pub type Rws6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS7` writer - XDMAC Channel 7 Read Write Suspend Bit"]
pub type Rws7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS8` writer - XDMAC Channel 8 Read Write Suspend Bit"]
pub type Rws8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS9` writer - XDMAC Channel 9 Read Write Suspend Bit"]
pub type Rws9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS10` writer - XDMAC Channel 10 Read Write Suspend Bit"]
pub type Rws10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS11` writer - XDMAC Channel 11 Read Write Suspend Bit"]
pub type Rws11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS12` writer - XDMAC Channel 12 Read Write Suspend Bit"]
pub type Rws12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS13` writer - XDMAC Channel 13 Read Write Suspend Bit"]
pub type Rws13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS14` writer - XDMAC Channel 14 Read Write Suspend Bit"]
pub type Rws14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS15` writer - XDMAC Channel 15 Read Write Suspend Bit"]
pub type Rws15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS16` writer - XDMAC Channel 16 Read Write Suspend Bit"]
pub type Rws16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS17` writer - XDMAC Channel 17 Read Write Suspend Bit"]
pub type Rws17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS18` writer - XDMAC Channel 18 Read Write Suspend Bit"]
pub type Rws18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS19` writer - XDMAC Channel 19 Read Write Suspend Bit"]
pub type Rws19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS20` writer - XDMAC Channel 20 Read Write Suspend Bit"]
pub type Rws20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS21` writer - XDMAC Channel 21 Read Write Suspend Bit"]
pub type Rws21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS22` writer - XDMAC Channel 22 Read Write Suspend Bit"]
pub type Rws22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWS23` writer - XDMAC Channel 23 Read Write Suspend Bit"]
pub type Rws23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws0(&mut self) -> Rws0W<GrwsSpec> {
        Rws0W::new(self, 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws1(&mut self) -> Rws1W<GrwsSpec> {
        Rws1W::new(self, 1)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws2(&mut self) -> Rws2W<GrwsSpec> {
        Rws2W::new(self, 2)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws3(&mut self) -> Rws3W<GrwsSpec> {
        Rws3W::new(self, 3)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws4(&mut self) -> Rws4W<GrwsSpec> {
        Rws4W::new(self, 4)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws5(&mut self) -> Rws5W<GrwsSpec> {
        Rws5W::new(self, 5)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws6(&mut self) -> Rws6W<GrwsSpec> {
        Rws6W::new(self, 6)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws7(&mut self) -> Rws7W<GrwsSpec> {
        Rws7W::new(self, 7)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws8(&mut self) -> Rws8W<GrwsSpec> {
        Rws8W::new(self, 8)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws9(&mut self) -> Rws9W<GrwsSpec> {
        Rws9W::new(self, 9)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws10(&mut self) -> Rws10W<GrwsSpec> {
        Rws10W::new(self, 10)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws11(&mut self) -> Rws11W<GrwsSpec> {
        Rws11W::new(self, 11)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws12(&mut self) -> Rws12W<GrwsSpec> {
        Rws12W::new(self, 12)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws13(&mut self) -> Rws13W<GrwsSpec> {
        Rws13W::new(self, 13)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws14(&mut self) -> Rws14W<GrwsSpec> {
        Rws14W::new(self, 14)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws15(&mut self) -> Rws15W<GrwsSpec> {
        Rws15W::new(self, 15)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws16(&mut self) -> Rws16W<GrwsSpec> {
        Rws16W::new(self, 16)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws17(&mut self) -> Rws17W<GrwsSpec> {
        Rws17W::new(self, 17)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws18(&mut self) -> Rws18W<GrwsSpec> {
        Rws18W::new(self, 18)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws19(&mut self) -> Rws19W<GrwsSpec> {
        Rws19W::new(self, 19)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws20(&mut self) -> Rws20W<GrwsSpec> {
        Rws20W::new(self, 20)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws21(&mut self) -> Rws21W<GrwsSpec> {
        Rws21W::new(self, 21)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws22(&mut self) -> Rws22W<GrwsSpec> {
        Rws22W::new(self, 22)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws23(&mut self) -> Rws23W<GrwsSpec> {
        Rws23W::new(self, 23)
    }
}
#[doc = "Global Channel Read Write Suspend Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrwsSpec;
impl crate::RegisterSpec for GrwsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`grws::W`](W) writer structure"]
impl crate::Writable for GrwsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRWS to value 0"]
impl crate::Resettable for GrwsSpec {}
