#[doc = "Register `CMUPD0` writer"]
pub type W = crate::W<Cmupd0Spec>;
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub type CpolupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub type CpolinvupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    pub fn cpolup(&mut self) -> CpolupW<Cmupd0Spec> {
        CpolupW::new(self, 9)
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    pub fn cpolinvup(&mut self) -> CpolinvupW<Cmupd0Spec> {
        CpolinvupW::new(self, 13)
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 0)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmupd0Spec;
impl crate::RegisterSpec for Cmupd0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmupd0::W`](W) writer structure"]
impl crate::Writable for Cmupd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMUPD0 to value 0"]
impl crate::Resettable for Cmupd0Spec {}
