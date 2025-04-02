#[doc = "Register `DMA_C_CTRL` reader"]
pub type R = crate::R<DmaCCtrlSpec>;
#[doc = "Register `DMA_C_CTRL` writer"]
pub type W = crate::W<DmaCCtrlSpec>;
#[doc = "Field `C_FETCH` reader - Descriptor Fetch Control Bit"]
pub type CFetchR = crate::BitReader;
#[doc = "Field `C_FETCH` writer - Descriptor Fetch Control Bit"]
pub type CFetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_WB` reader - Descriptor Writeback Control Bit"]
pub type CWbR = crate::BitReader;
#[doc = "Field `C_WB` writer - Descriptor Writeback Control Bit"]
pub type CWbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_IEN` reader - Transfer Done Flag Control"]
pub type CIenR = crate::BitReader;
#[doc = "Field `C_IEN` writer - Transfer Done Flag Control"]
pub type CIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DONE` reader - Codec Transfer Done"]
pub type CDoneR = crate::BitReader;
#[doc = "Field `C_DONE` writer - Codec Transfer Done"]
pub type CDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn c_fetch(&self) -> CFetchR {
        CFetchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn c_wb(&self) -> CWbR {
        CWbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn c_ien(&self) -> CIenR {
        CIenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Codec Transfer Done"]
    #[inline(always)]
    pub fn c_done(&self) -> CDoneR {
        CDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn c_fetch(&mut self) -> CFetchW<DmaCCtrlSpec> {
        CFetchW::new(self, 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn c_wb(&mut self) -> CWbW<DmaCCtrlSpec> {
        CWbW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn c_ien(&mut self) -> CIenW<DmaCCtrlSpec> {
        CIenW::new(self, 2)
    }
    #[doc = "Bit 3 - Codec Transfer Done"]
    #[inline(always)]
    pub fn c_done(&mut self) -> CDoneW<DmaCCtrlSpec> {
        CDoneW::new(self, 3)
    }
}
#[doc = "DMA Codec Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCCtrlSpec;
impl crate::RegisterSpec for DmaCCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c_ctrl::R`](R) reader structure"]
impl crate::Readable for DmaCCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c_ctrl::W`](W) writer structure"]
impl crate::Writable for DmaCCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C_CTRL to value 0"]
impl crate::Resettable for DmaCCtrlSpec {}
