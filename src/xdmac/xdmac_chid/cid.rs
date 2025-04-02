#[doc = "Register `CID` writer"]
pub type W = crate::W<CidSpec>;
#[doc = "Field `BID` writer - End of Block Interrupt Disable Bit"]
pub type BidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LID` writer - End of Linked List Interrupt Disable Bit"]
pub type LidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DID` writer - End of Disable Interrupt Disable Bit"]
pub type DidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FID` writer - End of Flush Interrupt Disable Bit"]
pub type FidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBEID` writer - Read Bus Error Interrupt Disable Bit"]
pub type RbeidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBEID` writer - Write Bus Error Interrupt Disable Bit"]
pub type WbeidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROID` writer - Request Overflow Error Interrupt Disable Bit"]
pub type RoidW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Disable Bit"]
    #[inline(always)]
    pub fn bid(&mut self) -> BidW<CidSpec> {
        BidW::new(self, 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Disable Bit"]
    #[inline(always)]
    pub fn lid(&mut self) -> LidW<CidSpec> {
        LidW::new(self, 1)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Disable Bit"]
    #[inline(always)]
    pub fn did(&mut self) -> DidW<CidSpec> {
        DidW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Disable Bit"]
    #[inline(always)]
    pub fn fid(&mut self) -> FidW<CidSpec> {
        FidW::new(self, 3)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn rbeid(&mut self) -> RbeidW<CidSpec> {
        RbeidW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn wbeid(&mut self) -> WbeidW<CidSpec> {
        WbeidW::new(self, 5)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn roid(&mut self) -> RoidW<CidSpec> {
        RoidW::new(self, 6)
    }
}
#[doc = "Channel Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CidSpec;
impl crate::RegisterSpec for CidSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cid::W`](W) writer structure"]
impl crate::Writable for CidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CID to value 0"]
impl crate::Resettable for CidSpec {}
