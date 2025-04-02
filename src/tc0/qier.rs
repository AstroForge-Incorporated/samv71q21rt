#[doc = "Register `QIER` writer"]
pub type W = crate::W<QierSpec>;
#[doc = "Field `IDX` writer - Index"]
pub type IdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QERR` writer - Quadrature Error"]
pub type QerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPE` writer - Consecutive Missing Pulse Error"]
pub type MpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&mut self) -> IdxW<QierSpec> {
        IdxW::new(self, 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DirchgW<QierSpec> {
        DirchgW::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&mut self) -> QerrW<QierSpec> {
        QerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Consecutive Missing Pulse Error"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MpeW<QierSpec> {
        MpeW::new(self, 3)
    }
}
#[doc = "QDEC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QierSpec;
impl crate::RegisterSpec for QierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qier::W`](W) writer structure"]
impl crate::Writable for QierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QIER to value 0"]
impl crate::Resettable for QierSpec {}
