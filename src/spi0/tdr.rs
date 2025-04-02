#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TD` writer - Transmit Data"]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcsselect {
    #[doc = "14: NPCS0 as Chip Select"]
    Npcs0 = 14,
    #[doc = "13: NPCS1 as Chip Select"]
    Npcs1 = 13,
    #[doc = "11: NPCS2 as Chip Select"]
    Npcs2 = 11,
    #[doc = "7: NPCS3 as Chip Select"]
    Npcs3 = 7,
}
impl From<Pcsselect> for u8 {
    #[inline(always)]
    fn from(variant: Pcsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcsselect {
    type Ux = u8;
}
impl crate::IsEnum for Pcsselect {}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pcsselect>;
impl<'a, REG> PcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn npcs0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsselect::Npcs0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn npcs1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsselect::Npcs1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn npcs2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsselect::Npcs2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn npcs3(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsselect::Npcs3)
    }
}
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LastxferW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn td(&mut self) -> TdW<TdrSpec> {
        TdW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PcsW<TdrSpec> {
        PcsW::new(self, 16)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LastxferW<TdrSpec> {
        LastxferW::new(self, 24)
    }
}
#[doc = "Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TdrSpec {}
