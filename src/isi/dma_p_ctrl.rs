#[doc = "Register `DMA_P_CTRL` reader"]
pub type R = crate::R<DmaPCtrlSpec>;
#[doc = "Register `DMA_P_CTRL` writer"]
pub type W = crate::W<DmaPCtrlSpec>;
#[doc = "Field `P_FETCH` reader - Descriptor Fetch Control Bit"]
pub type PFetchR = crate::BitReader;
#[doc = "Field `P_FETCH` writer - Descriptor Fetch Control Bit"]
pub type PFetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_WB` reader - Descriptor Writeback Control Bit"]
pub type PWbR = crate::BitReader;
#[doc = "Field `P_WB` writer - Descriptor Writeback Control Bit"]
pub type PWbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_IEN` reader - Transfer Done Flag Control"]
pub type PIenR = crate::BitReader;
#[doc = "Field `P_IEN` writer - Transfer Done Flag Control"]
pub type PIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_DONE` reader - Preview Transfer Done"]
pub type PDoneR = crate::BitReader;
#[doc = "Field `P_DONE` writer - Preview Transfer Done"]
pub type PDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&self) -> PFetchR {
        PFetchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&self) -> PWbR {
        PWbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&self) -> PIenR {
        PIenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&self) -> PDoneR {
        PDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&mut self) -> PFetchW<DmaPCtrlSpec> {
        PFetchW::new(self, 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&mut self) -> PWbW<DmaPCtrlSpec> {
        PWbW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&mut self) -> PIenW<DmaPCtrlSpec> {
        PIenW::new(self, 2)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&mut self) -> PDoneW<DmaPCtrlSpec> {
        PDoneW::new(self, 3)
    }
}
#[doc = "DMA Preview Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_p_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_p_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaPCtrlSpec;
impl crate::RegisterSpec for DmaPCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_p_ctrl::R`](R) reader structure"]
impl crate::Readable for DmaPCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_p_ctrl::W`](W) writer structure"]
impl crate::Writable for DmaPCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_P_CTRL to value 0"]
impl crate::Resettable for DmaPCtrlSpec {}
