#[doc = "Register `IAR` reader"]
pub type R = crate::R<IarSpec>;
#[doc = "Register `IAR` writer"]
pub type W = crate::W<IarSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<IarSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Instruction Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IarSpec;
impl crate::RegisterSpec for IarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iar::R`](R) reader structure"]
impl crate::Readable for IarSpec {}
#[doc = "`write(|w| ..)` method takes [`iar::W`](W) writer structure"]
impl crate::Writable for IarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IAR to value 0"]
impl crate::Resettable for IarSpec {}
