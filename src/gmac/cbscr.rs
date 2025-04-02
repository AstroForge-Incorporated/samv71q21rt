#[doc = "Register `CBSCR` reader"]
pub type R = crate::R<CbscrSpec>;
#[doc = "Register `CBSCR` writer"]
pub type W = crate::W<CbscrSpec>;
#[doc = "Field `QBE` reader - Queue B CBS Enable"]
pub type QbeR = crate::BitReader;
#[doc = "Field `QBE` writer - Queue B CBS Enable"]
pub type QbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QAE` reader - Queue A CBS Enable"]
pub type QaeR = crate::BitReader;
#[doc = "Field `QAE` writer - Queue A CBS Enable"]
pub type QaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&self) -> QbeR {
        QbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&self) -> QaeR {
        QaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&mut self) -> QbeW<CbscrSpec> {
        QbeW::new(self, 0)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&mut self) -> QaeW<CbscrSpec> {
        QaeW::new(self, 1)
    }
}
#[doc = "Credit-Based Shaping Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cbscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbscrSpec;
impl crate::RegisterSpec for CbscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbscr::R`](R) reader structure"]
impl crate::Readable for CbscrSpec {}
#[doc = "`write(|w| ..)` method takes [`cbscr::W`](W) writer structure"]
impl crate::Writable for CbscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CBSCR to value 0"]
impl crate::Resettable for CbscrSpec {}
