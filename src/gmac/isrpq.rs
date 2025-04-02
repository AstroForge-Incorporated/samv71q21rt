#[doc = "Register `ISRPQ[%s]` reader"]
pub type R = crate::R<IsrpqSpec>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RcompR = crate::BitReader;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RxubrR = crate::BitReader;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded or Late Collision"]
pub type RlexR = crate::BitReader;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TfcR = crate::BitReader;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TcompR = crate::BitReader;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::BitReader;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HrespR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RcompR {
        RcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RxubrR {
        RxubrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&self) -> RlexR {
        RlexR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TfcR {
        TfcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TcompR {
        TcompR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HrespR {
        HrespR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`isrpq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrpqSpec;
impl crate::RegisterSpec for IsrpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isrpq::R`](R) reader structure"]
impl crate::Readable for IsrpqSpec {}
#[doc = "`reset()` method sets ISRPQ[%s] to value 0"]
impl crate::Resettable for IsrpqSpec {}
