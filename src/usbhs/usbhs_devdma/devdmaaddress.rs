#[doc = "Register `DEVDMAADDRESS` reader"]
pub type R = crate::R<DevdmaaddressSpec>;
#[doc = "Register `DEVDMAADDRESS` writer"]
pub type W = crate::W<DevdmaaddressSpec>;
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
    pub fn buff_add(&mut self) -> BuffAddW<DevdmaaddressSpec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "Device DMA Channel Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevdmaaddressSpec;
impl crate::RegisterSpec for DevdmaaddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmaaddress::R`](R) reader structure"]
impl crate::Readable for DevdmaaddressSpec {}
#[doc = "`write(|w| ..)` method takes [`devdmaaddress::W`](W) writer structure"]
impl crate::Writable for DevdmaaddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVDMAADDRESS to value 0"]
impl crate::Resettable for DevdmaaddressSpec {}
