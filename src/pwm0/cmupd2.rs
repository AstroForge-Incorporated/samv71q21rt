#[doc = "Register `CMUPD2` writer"]
pub type W = crate::W<Cmupd2Spec>;
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub type CpolupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub type CpolinvupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    pub fn cpolup(&mut self) -> CpolupW<Cmupd2Spec> {
        CpolupW::new(self, 9)
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    pub fn cpolinvup(&mut self) -> CpolinvupW<Cmupd2Spec> {
        CpolinvupW::new(self, 13)
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 2)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmupd2Spec;
impl crate::RegisterSpec for Cmupd2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmupd2::W`](W) writer structure"]
impl crate::Writable for Cmupd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMUPD2 to value 0"]
impl crate::Resettable for Cmupd2Spec {}
