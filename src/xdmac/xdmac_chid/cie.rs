#[doc = "Register `CIE` writer"]
pub type W = crate::W<CieSpec>;
#[doc = "Field `BIE` writer - End of Block Interrupt Enable Bit"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIE` writer - End of Linked List Interrupt Enable Bit"]
pub type LieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIE` writer - End of Disable Interrupt Enable Bit"]
pub type DieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIE` writer - End of Flush Interrupt Enable Bit"]
pub type FieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBIE` writer - Read Bus Error Interrupt Enable Bit"]
pub type RbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBIE` writer - Write Bus Error Interrupt Enable Bit"]
pub type WbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIE` writer - Request Overflow Error Interrupt Enable Bit"]
pub type RoieW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Enable Bit"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<CieSpec> {
        BieW::new(self, 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Enable Bit"]
    #[inline(always)]
    pub fn lie(&mut self) -> LieW<CieSpec> {
        LieW::new(self, 1)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Enable Bit"]
    #[inline(always)]
    pub fn die(&mut self) -> DieW<CieSpec> {
        DieW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Enable Bit"]
    #[inline(always)]
    pub fn fie(&mut self) -> FieW<CieSpec> {
        FieW::new(self, 3)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rbie(&mut self) -> RbieW<CieSpec> {
        RbieW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn wbie(&mut self) -> WbieW<CieSpec> {
        WbieW::new(self, 5)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn roie(&mut self) -> RoieW<CieSpec> {
        RoieW::new(self, 6)
    }
}
#[doc = "Channel Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cie::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CieSpec;
impl crate::RegisterSpec for CieSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cie::W`](W) writer structure"]
impl crate::Writable for CieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIE to value 0"]
impl crate::Resettable for CieSpec {}
