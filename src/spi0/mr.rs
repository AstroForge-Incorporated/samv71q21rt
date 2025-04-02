#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstrselect {
    #[doc = "1: Master"]
    Master = 1,
    #[doc = "0: Slave"]
    Slave = 0,
}
impl From<Mstrselect> for bool {
    #[inline(always)]
    fn from(variant: Mstrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub type MstrR = crate::BitReader<Mstrselect>;
impl MstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstrselect {
        match self.bits {
            true => Mstrselect::Master,
            false => Mstrselect::Slave,
        }
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Mstrselect::Master
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Mstrselect::Slave
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, Mstrselect>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Mstrselect::Master)
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Mstrselect::Slave)
    }
}
#[doc = "Field `PS` reader - Peripheral Select"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Peripheral Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub type PcsdecR = crate::BitReader;
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub type PcsdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub type ModfdisR = crate::BitReader;
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub type ModfdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WdrbtR = crate::BitReader;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WdrbtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LlbR = crate::BitReader;
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LlbW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PcsR = crate::FieldReader<Pcsselect>;
impl PcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pcsselect> {
        match self.bits {
            14 => Some(Pcsselect::Npcs0),
            13 => Some(Pcsselect::Npcs1),
            11 => Some(Pcsselect::Npcs2),
            7 => Some(Pcsselect::Npcs3),
            _ => None,
        }
    }
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs0(&self) -> bool {
        *self == Pcsselect::Npcs0
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs1(&self) -> bool {
        *self == Pcsselect::Npcs1
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs2(&self) -> bool {
        *self == Pcsselect::Npcs2
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn is_npcs3(&self) -> bool {
        *self == Pcsselect::Npcs3
    }
}
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
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub type DlybcsR = crate::FieldReader;
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub type DlybcsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PcsdecR {
        PcsdecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> ModfdisR {
        ModfdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WdrbtR {
        WdrbtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LlbR {
        LlbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DlybcsR {
        DlybcsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<MrSpec> {
        MstrW::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<MrSpec> {
        PsW::new(self, 1)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&mut self) -> PcsdecW<MrSpec> {
        PcsdecW::new(self, 2)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&mut self) -> ModfdisW<MrSpec> {
        ModfdisW::new(self, 4)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WdrbtW<MrSpec> {
        WdrbtW::new(self, 5)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LlbW<MrSpec> {
        LlbW::new(self, 7)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PcsW<MrSpec> {
        PcsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&mut self) -> DlybcsW<MrSpec> {
        DlybcsW::new(self, 24)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
