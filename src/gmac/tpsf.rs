#[doc = "Register `TPSF` reader"]
pub type R = crate::R<TpsfSpec>;
#[doc = "Register `TPSF` writer"]
pub type W = crate::W<TpsfSpec>;
#[doc = "Field `TPB1ADR` reader - Transmit Partial Store and Forward Address"]
pub type Tpb1adrR = crate::FieldReader<u16>;
#[doc = "Field `TPB1ADR` writer - Transmit Partial Store and Forward Address"]
pub type Tpb1adrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ENTXP` reader - Enable TX Partial Store and Forward Operation"]
pub type EntxpR = crate::BitReader;
#[doc = "Field `ENTXP` writer - Enable TX Partial Store and Forward Operation"]
pub type EntxpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Transmit Partial Store and Forward Address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> Tpb1adrR {
        Tpb1adrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Enable TX Partial Store and Forward Operation"]
    #[inline(always)]
    pub fn entxp(&self) -> EntxpR {
        EntxpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transmit Partial Store and Forward Address"]
    #[inline(always)]
    pub fn tpb1adr(&mut self) -> Tpb1adrW<TpsfSpec> {
        Tpb1adrW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable TX Partial Store and Forward Operation"]
    #[inline(always)]
    pub fn entxp(&mut self) -> EntxpW<TpsfSpec> {
        EntxpW::new(self, 31)
    }
}
#[doc = "TX Partial Store and Forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpsf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpsf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpsfSpec;
impl crate::RegisterSpec for TpsfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpsf::R`](R) reader structure"]
impl crate::Readable for TpsfSpec {}
#[doc = "`write(|w| ..)` method takes [`tpsf::W`](W) writer structure"]
impl crate::Writable for TpsfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPSF to value 0"]
impl crate::Resettable for TpsfSpec {}
