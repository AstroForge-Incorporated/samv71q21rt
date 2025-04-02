#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `INST` reader - Instruction Code"]
pub type InstR = crate::FieldReader;
#[doc = "Field `INST` writer - Instruction Code"]
pub type InstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OPT` reader - Option Code"]
pub type OptR = crate::FieldReader;
#[doc = "Field `OPT` writer - Option Code"]
pub type OptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&self) -> InstR {
        InstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&self) -> OptR {
        OptR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&mut self) -> InstW<IcrSpec> {
        InstW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&mut self) -> OptW<IcrSpec> {
        OptW::new(self, 16)
    }
}
#[doc = "Instruction Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
