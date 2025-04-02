#[doc = "Register `HSTDMAADDRESS` reader"]
pub type R = crate::R<HstdmaaddressSpec>;
#[doc = "Register `HSTDMAADDRESS` writer"]
pub type W = crate::W<HstdmaaddressSpec>;
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub type BuffAddR = crate::FieldReader<u32>;
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub type BuffAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BuffAddR {
        BuffAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&mut self) -> BuffAddW<HstdmaaddressSpec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "Host DMA Channel Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstdmaaddressSpec;
impl crate::RegisterSpec for HstdmaaddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmaaddress::R`](R) reader structure"]
impl crate::Readable for HstdmaaddressSpec {}
#[doc = "`write(|w| ..)` method takes [`hstdmaaddress::W`](W) writer structure"]
impl crate::Writable for HstdmaaddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTDMAADDRESS to value 0"]
impl crate::Resettable for HstdmaaddressSpec {}
