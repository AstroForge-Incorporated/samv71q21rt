#[doc = "Register `IMRPQ[%s]` reader"]
pub type R = crate::R<ImrpqSpec>;
#[doc = "Register `IMRPQ[%s]` writer"]
pub type W = crate::W<ImrpqSpec>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RcompR = crate::BitReader;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RxubrR = crate::BitReader;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded or Late Collision"]
pub type RlexR = crate::BitReader;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RlexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB` reader - AHB Error"]
pub type AhbR = crate::BitReader;
#[doc = "Field `AHB` writer - AHB Error"]
pub type AhbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TcompR = crate::BitReader;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::BitReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HrespR = crate::BitReader;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - AHB Error"]
    #[inline(always)]
    pub fn ahb(&self) -> AhbR {
        AhbR::new(((self.bits >> 6) & 1) != 0)
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
impl W {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&mut self) -> RcompW<ImrpqSpec> {
        RcompW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&mut self) -> RxubrW<ImrpqSpec> {
        RxubrW::new(self, 2)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&mut self) -> RlexW<ImrpqSpec> {
        RlexW::new(self, 5)
    }
    #[doc = "Bit 6 - AHB Error"]
    #[inline(always)]
    pub fn ahb(&mut self) -> AhbW<ImrpqSpec> {
        AhbW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&mut self) -> TcompW<ImrpqSpec> {
        TcompW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&mut self) -> RovrW<ImrpqSpec> {
        RovrW::new(self, 10)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&mut self) -> HrespW<ImrpqSpec> {
        HrespW::new(self, 11)
    }
}
#[doc = "Interrupt Mask Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`imrpq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imrpq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrpqSpec;
impl crate::RegisterSpec for ImrpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imrpq::R`](R) reader structure"]
impl crate::Readable for ImrpqSpec {}
#[doc = "`write(|w| ..)` method takes [`imrpq::W`](W) writer structure"]
impl crate::Writable for ImrpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMRPQ[%s] to value 0"]
impl crate::Resettable for ImrpqSpec {}
