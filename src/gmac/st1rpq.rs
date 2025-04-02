#[doc = "Register `ST1RPQ[%s]` reader"]
pub type R = crate::R<St1rpqSpec>;
#[doc = "Register `ST1RPQ[%s]` writer"]
pub type W = crate::W<St1rpqSpec>;
#[doc = "Field `QNB` reader - Queue Number (0-5)"]
pub type QnbR = crate::FieldReader;
#[doc = "Field `QNB` writer - Queue Number (0-5)"]
pub type QnbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSTCM` reader - Differentiated Services or Traffic Class Match"]
pub type DstcmR = crate::FieldReader;
#[doc = "Field `DSTCM` writer - Differentiated Services or Traffic Class Match"]
pub type DstcmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UDPM` reader - UDP Port Match"]
pub type UdpmR = crate::FieldReader<u16>;
#[doc = "Field `UDPM` writer - UDP Port Match"]
pub type UdpmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DSTCE` reader - Differentiated Services or Traffic Class Match Enable"]
pub type DstceR = crate::BitReader;
#[doc = "Field `DSTCE` writer - Differentiated Services or Traffic Class Match Enable"]
pub type DstceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDPE` reader - UDP Port Match Enable"]
pub type UdpeR = crate::BitReader;
#[doc = "Field `UDPE` writer - UDP Port Match Enable"]
pub type UdpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QnbR {
        QnbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&self) -> DstcmR {
        DstcmR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&self) -> UdpmR {
        UdpmR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&self) -> DstceR {
        DstceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&self) -> UdpeR {
        UdpeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&mut self) -> QnbW<St1rpqSpec> {
        QnbW::new(self, 0)
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&mut self) -> DstcmW<St1rpqSpec> {
        DstcmW::new(self, 4)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&mut self) -> UdpmW<St1rpqSpec> {
        UdpmW::new(self, 12)
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&mut self) -> DstceW<St1rpqSpec> {
        DstceW::new(self, 28)
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&mut self) -> UdpeW<St1rpqSpec> {
        UdpeW::new(self, 29)
    }
}
#[doc = "Screening Type 1 Register Priority Queue\n\nYou can [`read`](crate::Reg::read) this register and get [`st1rpq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1rpq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St1rpqSpec;
impl crate::RegisterSpec for St1rpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1rpq::R`](R) reader structure"]
impl crate::Readable for St1rpqSpec {}
#[doc = "`write(|w| ..)` method takes [`st1rpq::W`](W) writer structure"]
impl crate::Writable for St1rpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST1RPQ[%s] to value 0"]
impl crate::Resettable for St1rpqSpec {}
