#[doc = "Register `PRAS` reader"]
pub type R = crate::R<PrasSpec>;
#[doc = "Register `PRAS` writer"]
pub type W = crate::W<PrasSpec>;
#[doc = "Field `M0PR` reader - Master 0 Priority"]
pub type M0prR = crate::FieldReader;
#[doc = "Field `M0PR` writer - Master 0 Priority"]
pub type M0prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M1PR` reader - Master 1 Priority"]
pub type M1prR = crate::FieldReader;
#[doc = "Field `M1PR` writer - Master 1 Priority"]
pub type M1prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2PR` reader - Master 2 Priority"]
pub type M2prR = crate::FieldReader;
#[doc = "Field `M2PR` writer - Master 2 Priority"]
pub type M2prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M3PR` reader - Master 3 Priority"]
pub type M3prR = crate::FieldReader;
#[doc = "Field `M3PR` writer - Master 3 Priority"]
pub type M3prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M4PR` reader - Master 4 Priority"]
pub type M4prR = crate::FieldReader;
#[doc = "Field `M4PR` writer - Master 4 Priority"]
pub type M4prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M5PR` reader - Master 5 Priority"]
pub type M5prR = crate::FieldReader;
#[doc = "Field `M5PR` writer - Master 5 Priority"]
pub type M5prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M6PR` reader - Master 6 Priority"]
pub type M6prR = crate::FieldReader;
#[doc = "Field `M6PR` writer - Master 6 Priority"]
pub type M6prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M7PR` reader - Master 7 Priority"]
pub type M7prR = crate::FieldReader;
#[doc = "Field `M7PR` writer - Master 7 Priority"]
pub type M7prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0prR {
        M0prR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1prR {
        M1prR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2prR {
        M2prR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3prR {
        M3prR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4prR {
        M4prR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&self) -> M5prR {
        M5prR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6pr(&self) -> M6prR {
        M6prR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7pr(&self) -> M7prR {
        M7prR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&mut self) -> M0prW<PrasSpec> {
        M0prW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&mut self) -> M1prW<PrasSpec> {
        M1prW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&mut self) -> M2prW<PrasSpec> {
        M2prW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&mut self) -> M3prW<PrasSpec> {
        M3prW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&mut self) -> M4prW<PrasSpec> {
        M4prW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&mut self) -> M5prW<PrasSpec> {
        M5prW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6pr(&mut self) -> M6prW<PrasSpec> {
        M6prW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7pr(&mut self) -> M7prW<PrasSpec> {
        M7prW::new(self, 28)
    }
}
#[doc = "Priority Register A for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrasSpec;
impl crate::RegisterSpec for PrasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pras::R`](R) reader structure"]
impl crate::Readable for PrasSpec {}
#[doc = "`write(|w| ..)` method takes [`pras::W`](W) writer structure"]
impl crate::Writable for PrasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRAS to value 0"]
impl crate::Resettable for PrasSpec {}
