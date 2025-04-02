#[doc = "Register `CMUPD1` writer"]
pub type W = crate::W<Cmupd1Spec>;
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub type CpolupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub type CpolinvupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    pub fn cpolup(&mut self) -> CpolupW<Cmupd1Spec> {
        CpolupW::new(self, 9)
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    pub fn cpolinvup(&mut self) -> CpolinvupW<Cmupd1Spec> {
        CpolinvupW::new(self, 13)
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmupd1Spec;
impl crate::RegisterSpec for Cmupd1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmupd1::W`](W) writer structure"]
impl crate::Writable for Cmupd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMUPD1 to value 0"]
impl crate::Resettable for Cmupd1Spec {}
