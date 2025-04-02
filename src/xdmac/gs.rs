#[doc = "Register `GS` reader"]
pub type R = crate::R<GsSpec>;
#[doc = "Field `ST0` reader - XDMAC Channel 0 Status Bit"]
pub type St0R = crate::BitReader;
#[doc = "Field `ST1` reader - XDMAC Channel 1 Status Bit"]
pub type St1R = crate::BitReader;
#[doc = "Field `ST2` reader - XDMAC Channel 2 Status Bit"]
pub type St2R = crate::BitReader;
#[doc = "Field `ST3` reader - XDMAC Channel 3 Status Bit"]
pub type St3R = crate::BitReader;
#[doc = "Field `ST4` reader - XDMAC Channel 4 Status Bit"]
pub type St4R = crate::BitReader;
#[doc = "Field `ST5` reader - XDMAC Channel 5 Status Bit"]
pub type St5R = crate::BitReader;
#[doc = "Field `ST6` reader - XDMAC Channel 6 Status Bit"]
pub type St6R = crate::BitReader;
#[doc = "Field `ST7` reader - XDMAC Channel 7 Status Bit"]
pub type St7R = crate::BitReader;
#[doc = "Field `ST8` reader - XDMAC Channel 8 Status Bit"]
pub type St8R = crate::BitReader;
#[doc = "Field `ST9` reader - XDMAC Channel 9 Status Bit"]
pub type St9R = crate::BitReader;
#[doc = "Field `ST10` reader - XDMAC Channel 10 Status Bit"]
pub type St10R = crate::BitReader;
#[doc = "Field `ST11` reader - XDMAC Channel 11 Status Bit"]
pub type St11R = crate::BitReader;
#[doc = "Field `ST12` reader - XDMAC Channel 12 Status Bit"]
pub type St12R = crate::BitReader;
#[doc = "Field `ST13` reader - XDMAC Channel 13 Status Bit"]
pub type St13R = crate::BitReader;
#[doc = "Field `ST14` reader - XDMAC Channel 14 Status Bit"]
pub type St14R = crate::BitReader;
#[doc = "Field `ST15` reader - XDMAC Channel 15 Status Bit"]
pub type St15R = crate::BitReader;
#[doc = "Field `ST16` reader - XDMAC Channel 16 Status Bit"]
pub type St16R = crate::BitReader;
#[doc = "Field `ST17` reader - XDMAC Channel 17 Status Bit"]
pub type St17R = crate::BitReader;
#[doc = "Field `ST18` reader - XDMAC Channel 18 Status Bit"]
pub type St18R = crate::BitReader;
#[doc = "Field `ST19` reader - XDMAC Channel 19 Status Bit"]
pub type St19R = crate::BitReader;
#[doc = "Field `ST20` reader - XDMAC Channel 20 Status Bit"]
pub type St20R = crate::BitReader;
#[doc = "Field `ST21` reader - XDMAC Channel 21 Status Bit"]
pub type St21R = crate::BitReader;
#[doc = "Field `ST22` reader - XDMAC Channel 22 Status Bit"]
pub type St22R = crate::BitReader;
#[doc = "Field `ST23` reader - XDMAC Channel 23 Status Bit"]
pub type St23R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Status Bit"]
    #[inline(always)]
    pub fn st0(&self) -> St0R {
        St0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Status Bit"]
    #[inline(always)]
    pub fn st1(&self) -> St1R {
        St1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Status Bit"]
    #[inline(always)]
    pub fn st2(&self) -> St2R {
        St2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Status Bit"]
    #[inline(always)]
    pub fn st3(&self) -> St3R {
        St3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Status Bit"]
    #[inline(always)]
    pub fn st4(&self) -> St4R {
        St4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Status Bit"]
    #[inline(always)]
    pub fn st5(&self) -> St5R {
        St5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Status Bit"]
    #[inline(always)]
    pub fn st6(&self) -> St6R {
        St6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Status Bit"]
    #[inline(always)]
    pub fn st7(&self) -> St7R {
        St7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Status Bit"]
    #[inline(always)]
    pub fn st8(&self) -> St8R {
        St8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Status Bit"]
    #[inline(always)]
    pub fn st9(&self) -> St9R {
        St9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Status Bit"]
    #[inline(always)]
    pub fn st10(&self) -> St10R {
        St10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Status Bit"]
    #[inline(always)]
    pub fn st11(&self) -> St11R {
        St11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Status Bit"]
    #[inline(always)]
    pub fn st12(&self) -> St12R {
        St12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Status Bit"]
    #[inline(always)]
    pub fn st13(&self) -> St13R {
        St13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Status Bit"]
    #[inline(always)]
    pub fn st14(&self) -> St14R {
        St14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Status Bit"]
    #[inline(always)]
    pub fn st15(&self) -> St15R {
        St15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Status Bit"]
    #[inline(always)]
    pub fn st16(&self) -> St16R {
        St16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Status Bit"]
    #[inline(always)]
    pub fn st17(&self) -> St17R {
        St17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Status Bit"]
    #[inline(always)]
    pub fn st18(&self) -> St18R {
        St18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Status Bit"]
    #[inline(always)]
    pub fn st19(&self) -> St19R {
        St19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Status Bit"]
    #[inline(always)]
    pub fn st20(&self) -> St20R {
        St20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Status Bit"]
    #[inline(always)]
    pub fn st21(&self) -> St21R {
        St21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Status Bit"]
    #[inline(always)]
    pub fn st22(&self) -> St22R {
        St22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Status Bit"]
    #[inline(always)]
    pub fn st23(&self) -> St23R {
        St23R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GsSpec;
impl crate::RegisterSpec for GsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gs::R`](R) reader structure"]
impl crate::Readable for GsSpec {}
#[doc = "`reset()` method sets GS to value 0"]
impl crate::Resettable for GsSpec {}
