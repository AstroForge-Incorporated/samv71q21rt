#[doc = "Register `MDAT[%s]` reader"]
pub type R = crate::R<MdatSpec>;
#[doc = "Register `MDAT[%s]` writer"]
pub type W = crate::W<MdatSpec>;
#[doc = "Field `DATA` reader - CRT or DBR Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - CRT or DBR Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRT or DBR Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRT or DBR Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<MdatSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "MIF Data 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdatSpec;
impl crate::RegisterSpec for MdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdat::R`](R) reader structure"]
impl crate::Readable for MdatSpec {}
#[doc = "`write(|w| ..)` method takes [`mdat::W`](W) writer structure"]
impl crate::Writable for MdatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDAT[%s] to value 0"]
impl crate::Resettable for MdatSpec {}
