#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` writer - Enables the TRNG to Provide Random Values"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Security Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Keyselect {
    #[doc = "5393991: Writing any other value in this field aborts the write operation."]
    Passwd = 5393991,
}
impl From<Keyselect> for u32 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u32;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` writer - Security Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the TRNG to Provide Random Values"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Security Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
