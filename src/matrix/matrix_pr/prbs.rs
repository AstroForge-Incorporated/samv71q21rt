#[doc = "Register `PRBS` reader"]
pub type R = crate::R<PrbsSpec>;
#[doc = "Register `PRBS` writer"]
pub type W = crate::W<PrbsSpec>;
#[doc = "Field `M8PR` reader - Master 8 Priority"]
pub type M8prR = crate::FieldReader;
#[doc = "Field `M8PR` writer - Master 8 Priority"]
pub type M8prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M9PR` reader - Master 9 Priority"]
pub type M9prR = crate::FieldReader;
#[doc = "Field `M9PR` writer - Master 9 Priority"]
pub type M9prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M10PR` reader - Master 10 Priority"]
pub type M10prR = crate::FieldReader;
#[doc = "Field `M10PR` writer - Master 10 Priority"]
pub type M10prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M11PR` reader - Master 11 Priority"]
pub type M11prR = crate::FieldReader;
#[doc = "Field `M11PR` writer - Master 11 Priority"]
pub type M11prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M12PR` reader - Master 12 Priority"]
pub type M12prR = crate::FieldReader;
#[doc = "Field `M12PR` writer - Master 12 Priority"]
pub type M12prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8prR {
        M8prR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9prR {
        M9prR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10prR {
        M10prR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&self) -> M11prR {
        M11prR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&self) -> M12prR {
        M12prR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&mut self) -> M8prW<PrbsSpec> {
        M8prW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&mut self) -> M9prW<PrbsSpec> {
        M9prW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&mut self) -> M10prW<PrbsSpec> {
        M10prW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&mut self) -> M11prW<PrbsSpec> {
        M11prW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&mut self) -> M12prW<PrbsSpec> {
        M12prW::new(self, 16)
    }
}
#[doc = "Priority Register B for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrbsSpec;
impl crate::RegisterSpec for PrbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prbs::R`](R) reader structure"]
impl crate::Readable for PrbsSpec {}
#[doc = "`write(|w| ..)` method takes [`prbs::W`](W) writer structure"]
impl crate::Writable for PrbsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRBS to value 0"]
impl crate::Resettable for PrbsSpec {}
