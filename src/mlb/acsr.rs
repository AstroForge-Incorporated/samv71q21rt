#[doc = "Register `ACSR[%s]` reader"]
pub type R = crate::R<AcsrSpec>;
#[doc = "Register `ACSR[%s]` writer"]
pub type W = crate::W<AcsrSpec>;
#[doc = "Field `CHS` reader - Interrupt Status for Logical Channels \\[31:0\\] (cleared by writing a 1)"]
pub type ChsR = crate::FieldReader<u32>;
#[doc = "Field `CHS` writer - Interrupt Status for Logical Channels \\[31:0\\] (cleared by writing a 1)"]
pub type ChsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\] (cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&self) -> ChsR {
        ChsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\] (cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&mut self) -> ChsW<AcsrSpec> {
        ChsW::new(self, 0)
    }
}
#[doc = "AHB Channel Status 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`acsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcsrSpec;
impl crate::RegisterSpec for AcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acsr::R`](R) reader structure"]
impl crate::Readable for AcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`acsr::W`](W) writer structure"]
impl crate::Writable for AcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACSR[%s] to value 0"]
impl crate::Resettable for AcsrSpec {}
