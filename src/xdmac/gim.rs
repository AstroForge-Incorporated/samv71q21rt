#[doc = "Register `GIM` reader"]
pub type R = crate::R<GimSpec>;
#[doc = "Field `IM0` reader - XDMAC Channel 0 Interrupt Mask Bit"]
pub type Im0R = crate::BitReader;
#[doc = "Field `IM1` reader - XDMAC Channel 1 Interrupt Mask Bit"]
pub type Im1R = crate::BitReader;
#[doc = "Field `IM2` reader - XDMAC Channel 2 Interrupt Mask Bit"]
pub type Im2R = crate::BitReader;
#[doc = "Field `IM3` reader - XDMAC Channel 3 Interrupt Mask Bit"]
pub type Im3R = crate::BitReader;
#[doc = "Field `IM4` reader - XDMAC Channel 4 Interrupt Mask Bit"]
pub type Im4R = crate::BitReader;
#[doc = "Field `IM5` reader - XDMAC Channel 5 Interrupt Mask Bit"]
pub type Im5R = crate::BitReader;
#[doc = "Field `IM6` reader - XDMAC Channel 6 Interrupt Mask Bit"]
pub type Im6R = crate::BitReader;
#[doc = "Field `IM7` reader - XDMAC Channel 7 Interrupt Mask Bit"]
pub type Im7R = crate::BitReader;
#[doc = "Field `IM8` reader - XDMAC Channel 8 Interrupt Mask Bit"]
pub type Im8R = crate::BitReader;
#[doc = "Field `IM9` reader - XDMAC Channel 9 Interrupt Mask Bit"]
pub type Im9R = crate::BitReader;
#[doc = "Field `IM10` reader - XDMAC Channel 10 Interrupt Mask Bit"]
pub type Im10R = crate::BitReader;
#[doc = "Field `IM11` reader - XDMAC Channel 11 Interrupt Mask Bit"]
pub type Im11R = crate::BitReader;
#[doc = "Field `IM12` reader - XDMAC Channel 12 Interrupt Mask Bit"]
pub type Im12R = crate::BitReader;
#[doc = "Field `IM13` reader - XDMAC Channel 13 Interrupt Mask Bit"]
pub type Im13R = crate::BitReader;
#[doc = "Field `IM14` reader - XDMAC Channel 14 Interrupt Mask Bit"]
pub type Im14R = crate::BitReader;
#[doc = "Field `IM15` reader - XDMAC Channel 15 Interrupt Mask Bit"]
pub type Im15R = crate::BitReader;
#[doc = "Field `IM16` reader - XDMAC Channel 16 Interrupt Mask Bit"]
pub type Im16R = crate::BitReader;
#[doc = "Field `IM17` reader - XDMAC Channel 17 Interrupt Mask Bit"]
pub type Im17R = crate::BitReader;
#[doc = "Field `IM18` reader - XDMAC Channel 18 Interrupt Mask Bit"]
pub type Im18R = crate::BitReader;
#[doc = "Field `IM19` reader - XDMAC Channel 19 Interrupt Mask Bit"]
pub type Im19R = crate::BitReader;
#[doc = "Field `IM20` reader - XDMAC Channel 20 Interrupt Mask Bit"]
pub type Im20R = crate::BitReader;
#[doc = "Field `IM21` reader - XDMAC Channel 21 Interrupt Mask Bit"]
pub type Im21R = crate::BitReader;
#[doc = "Field `IM22` reader - XDMAC Channel 22 Interrupt Mask Bit"]
pub type Im22R = crate::BitReader;
#[doc = "Field `IM23` reader - XDMAC Channel 23 Interrupt Mask Bit"]
pub type Im23R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im0(&self) -> Im0R {
        Im0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im1(&self) -> Im1R {
        Im1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im2(&self) -> Im2R {
        Im2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im3(&self) -> Im3R {
        Im3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im4(&self) -> Im4R {
        Im4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im5(&self) -> Im5R {
        Im5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im6(&self) -> Im6R {
        Im6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im7(&self) -> Im7R {
        Im7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im8(&self) -> Im8R {
        Im8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im9(&self) -> Im9R {
        Im9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im10(&self) -> Im10R {
        Im10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im11(&self) -> Im11R {
        Im11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im12(&self) -> Im12R {
        Im12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im13(&self) -> Im13R {
        Im13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im14(&self) -> Im14R {
        Im14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im15(&self) -> Im15R {
        Im15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im16(&self) -> Im16R {
        Im16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im17(&self) -> Im17R {
        Im17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im18(&self) -> Im18R {
        Im18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im19(&self) -> Im19R {
        Im19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im20(&self) -> Im20R {
        Im20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im21(&self) -> Im21R {
        Im21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im22(&self) -> Im22R {
        Im22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im23(&self) -> Im23R {
        Im23R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GimSpec;
impl crate::RegisterSpec for GimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gim::R`](R) reader structure"]
impl crate::Readable for GimSpec {}
#[doc = "`reset()` method sets GIM to value 0"]
impl crate::Resettable for GimSpec {}
