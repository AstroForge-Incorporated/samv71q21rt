#[doc = "Register `MS1` reader"]
pub type R = crate::R<Ms1Spec>;
#[doc = "Register `MS1` writer"]
pub type W = crate::W<Ms1Spec>;
#[doc = "Field `MCS` reader - MediaLB Channel Status \\[63:32\\] (cleared by writing a 0)"]
pub type McsR = crate::FieldReader<u32>;
#[doc = "Field `MCS` writer - MediaLB Channel Status \\[63:32\\] (cleared by writing a 0)"]
pub type McsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[63:32\\] (cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&self) -> McsR {
        McsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[63:32\\] (cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&mut self) -> McsW<Ms1Spec> {
        McsW::new(self, 0)
    }
}
#[doc = "MediaLB Channel Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ms1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ms1Spec;
impl crate::RegisterSpec for Ms1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms1::R`](R) reader structure"]
impl crate::Readable for Ms1Spec {}
#[doc = "`write(|w| ..)` method takes [`ms1::W`](W) writer structure"]
impl crate::Writable for Ms1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MS1 to value 0"]
impl crate::Resettable for Ms1Spec {}
