#[doc = "Register `GSWS` reader"]
pub type R = crate::R<GswsSpec>;
#[doc = "Field `SWRS0` reader - XDMAC Channel 0 Software Request Status Bit"]
pub type Swrs0R = crate::BitReader;
#[doc = "Field `SWRS1` reader - XDMAC Channel 1 Software Request Status Bit"]
pub type Swrs1R = crate::BitReader;
#[doc = "Field `SWRS2` reader - XDMAC Channel 2 Software Request Status Bit"]
pub type Swrs2R = crate::BitReader;
#[doc = "Field `SWRS3` reader - XDMAC Channel 3 Software Request Status Bit"]
pub type Swrs3R = crate::BitReader;
#[doc = "Field `SWRS4` reader - XDMAC Channel 4 Software Request Status Bit"]
pub type Swrs4R = crate::BitReader;
#[doc = "Field `SWRS5` reader - XDMAC Channel 5 Software Request Status Bit"]
pub type Swrs5R = crate::BitReader;
#[doc = "Field `SWRS6` reader - XDMAC Channel 6 Software Request Status Bit"]
pub type Swrs6R = crate::BitReader;
#[doc = "Field `SWRS7` reader - XDMAC Channel 7 Software Request Status Bit"]
pub type Swrs7R = crate::BitReader;
#[doc = "Field `SWRS8` reader - XDMAC Channel 8 Software Request Status Bit"]
pub type Swrs8R = crate::BitReader;
#[doc = "Field `SWRS9` reader - XDMAC Channel 9 Software Request Status Bit"]
pub type Swrs9R = crate::BitReader;
#[doc = "Field `SWRS10` reader - XDMAC Channel 10 Software Request Status Bit"]
pub type Swrs10R = crate::BitReader;
#[doc = "Field `SWRS11` reader - XDMAC Channel 11 Software Request Status Bit"]
pub type Swrs11R = crate::BitReader;
#[doc = "Field `SWRS12` reader - XDMAC Channel 12 Software Request Status Bit"]
pub type Swrs12R = crate::BitReader;
#[doc = "Field `SWRS13` reader - XDMAC Channel 13 Software Request Status Bit"]
pub type Swrs13R = crate::BitReader;
#[doc = "Field `SWRS14` reader - XDMAC Channel 14 Software Request Status Bit"]
pub type Swrs14R = crate::BitReader;
#[doc = "Field `SWRS15` reader - XDMAC Channel 15 Software Request Status Bit"]
pub type Swrs15R = crate::BitReader;
#[doc = "Field `SWRS16` reader - XDMAC Channel 16 Software Request Status Bit"]
pub type Swrs16R = crate::BitReader;
#[doc = "Field `SWRS17` reader - XDMAC Channel 17 Software Request Status Bit"]
pub type Swrs17R = crate::BitReader;
#[doc = "Field `SWRS18` reader - XDMAC Channel 18 Software Request Status Bit"]
pub type Swrs18R = crate::BitReader;
#[doc = "Field `SWRS19` reader - XDMAC Channel 19 Software Request Status Bit"]
pub type Swrs19R = crate::BitReader;
#[doc = "Field `SWRS20` reader - XDMAC Channel 20 Software Request Status Bit"]
pub type Swrs20R = crate::BitReader;
#[doc = "Field `SWRS21` reader - XDMAC Channel 21 Software Request Status Bit"]
pub type Swrs21R = crate::BitReader;
#[doc = "Field `SWRS22` reader - XDMAC Channel 22 Software Request Status Bit"]
pub type Swrs22R = crate::BitReader;
#[doc = "Field `SWRS23` reader - XDMAC Channel 23 Software Request Status Bit"]
pub type Swrs23R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs0(&self) -> Swrs0R {
        Swrs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs1(&self) -> Swrs1R {
        Swrs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs2(&self) -> Swrs2R {
        Swrs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs3(&self) -> Swrs3R {
        Swrs3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs4(&self) -> Swrs4R {
        Swrs4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs5(&self) -> Swrs5R {
        Swrs5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs6(&self) -> Swrs6R {
        Swrs6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs7(&self) -> Swrs7R {
        Swrs7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs8(&self) -> Swrs8R {
        Swrs8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs9(&self) -> Swrs9R {
        Swrs9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs10(&self) -> Swrs10R {
        Swrs10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs11(&self) -> Swrs11R {
        Swrs11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs12(&self) -> Swrs12R {
        Swrs12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs13(&self) -> Swrs13R {
        Swrs13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs14(&self) -> Swrs14R {
        Swrs14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs15(&self) -> Swrs15R {
        Swrs15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs16(&self) -> Swrs16R {
        Swrs16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs17(&self) -> Swrs17R {
        Swrs17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs18(&self) -> Swrs18R {
        Swrs18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs19(&self) -> Swrs19R {
        Swrs19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs20(&self) -> Swrs20R {
        Swrs20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs21(&self) -> Swrs21R {
        Swrs21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs22(&self) -> Swrs22R {
        Swrs22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs23(&self) -> Swrs23R {
        Swrs23R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Channel Software Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gsws::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GswsSpec;
impl crate::RegisterSpec for GswsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsws::R`](R) reader structure"]
impl crate::Readable for GswsSpec {}
#[doc = "`reset()` method sets GSWS to value 0"]
impl crate::Resettable for GswsSpec {}
