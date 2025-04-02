#[doc = "Register `CFR1` reader"]
pub type R = crate::R<Cfr1Spec>;
#[doc = "Register `CFR1` writer"]
pub type W = crate::W<Cfr1Spec>;
#[doc = "Field `TMRD` reader - Load Mode Register Command to Active or Refresh Command"]
pub type TmrdR = crate::FieldReader;
#[doc = "Field `TMRD` writer - Load Mode Register Command to Active or Refresh Command"]
pub type TmrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Support Unaligned Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unalselect {
    #[doc = "0: Unaligned access is not supported."]
    Unsupported = 0,
    #[doc = "1: Unaligned access is supported."]
    Supported = 1,
}
impl From<Unalselect> for bool {
    #[inline(always)]
    fn from(variant: Unalselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNAL` reader - Support Unaligned Access"]
pub type UnalR = crate::BitReader<Unalselect>;
impl UnalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unalselect {
        match self.bits {
            false => Unalselect::Unsupported,
            true => Unalselect::Supported,
        }
    }
    #[doc = "Unaligned access is not supported."]
    #[inline(always)]
    pub fn is_unsupported(&self) -> bool {
        *self == Unalselect::Unsupported
    }
    #[doc = "Unaligned access is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Unalselect::Supported
    }
}
#[doc = "Field `UNAL` writer - Support Unaligned Access"]
pub type UnalW<'a, REG> = crate::BitWriter<'a, REG, Unalselect>;
impl<'a, REG> UnalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unaligned access is not supported."]
    #[inline(always)]
    pub fn unsupported(self) -> &'a mut crate::W<REG> {
        self.variant(Unalselect::Unsupported)
    }
    #[doc = "Unaligned access is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Unalselect::Supported)
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&self) -> TmrdR {
        TmrdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&self) -> UnalR {
        UnalR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TmrdW<Cfr1Spec> {
        TmrdW::new(self, 0)
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&mut self) -> UnalW<Cfr1Spec> {
        UnalW::new(self, 8)
    }
}
#[doc = "SDRAMC Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfr1Spec;
impl crate::RegisterSpec for Cfr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr1::R`](R) reader structure"]
impl crate::Readable for Cfr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfr1::W`](W) writer structure"]
impl crate::Writable for Cfr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFR1 to value 0"]
impl crate::Resettable for Cfr1Spec {}
