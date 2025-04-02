#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Receiver Digital Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filterselect {
    #[doc = "0: UART does not filter the receive line."]
    Disabled = 0,
    #[doc = "1: UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    Enabled = 1,
}
impl From<Filterselect> for bool {
    #[inline(always)]
    fn from(variant: Filterselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTER` reader - Receiver Digital Filter"]
pub type FilterR = crate::BitReader<Filterselect>;
impl FilterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filterselect {
        match self.bits {
            false => Filterselect::Disabled,
            true => Filterselect::Enabled,
        }
    }
    #[doc = "UART does not filter the receive line."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Filterselect::Disabled
    }
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Filterselect::Enabled
    }
}
#[doc = "Field `FILTER` writer - Receiver Digital Filter"]
pub type FilterW<'a, REG> = crate::BitWriter<'a, REG, Filterselect>;
impl<'a, REG> FilterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART does not filter the receive line."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Disabled)
    }
    #[doc = "UART filters the receive line using a three-sample filter (16x-bit clock) (2 over 3 majority)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Filterselect::Enabled)
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parselect {
    #[doc = "0: Even Parity"]
    Even = 0,
    #[doc = "1: Odd Parity"]
    Odd = 1,
    #[doc = "2: Space: parity forced to 0"]
    Space = 2,
    #[doc = "3: Mark: parity forced to 1"]
    Mark = 3,
    #[doc = "4: No parity"]
    No = 4,
}
impl From<Parselect> for u8 {
    #[inline(always)]
    fn from(variant: Parselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parselect {
    type Ux = u8;
}
impl crate::IsEnum for Parselect {}
#[doc = "Field `PAR` reader - Parity Type"]
pub type ParR = crate::FieldReader<Parselect>;
impl ParR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parselect> {
        match self.bits {
            0 => Some(Parselect::Even),
            1 => Some(Parselect::Odd),
            2 => Some(Parselect::Space),
            3 => Some(Parselect::Mark),
            4 => Some(Parselect::No),
            _ => None,
        }
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Parselect::Even
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Parselect::Odd
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == Parselect::Space
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == Parselect::Mark
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Parselect::No
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type ParW<'a, REG> = crate::FieldWriter<'a, REG, 3, Parselect>;
impl<'a, REG> ParW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Even)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Odd)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Space)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::Mark)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Parselect::No)
    }
}
#[doc = "Baud Rate Source Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brsrcckselect {
    #[doc = "0: The baud rate is driven by the peripheral clock"]
    PeriphClk = 0,
    #[doc = "1: The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    PmcPck = 1,
}
impl From<Brsrcckselect> for bool {
    #[inline(always)]
    fn from(variant: Brsrcckselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSRCCK` reader - Baud Rate Source Clock"]
pub type BrsrcckR = crate::BitReader<Brsrcckselect>;
impl BrsrcckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brsrcckselect {
        match self.bits {
            false => Brsrcckselect::PeriphClk,
            true => Brsrcckselect::PmcPck,
        }
    }
    #[doc = "The baud rate is driven by the peripheral clock"]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        *self == Brsrcckselect::PeriphClk
    }
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        *self == Brsrcckselect::PmcPck
    }
}
#[doc = "Field `BRSRCCK` writer - Baud Rate Source Clock"]
pub type BrsrcckW<'a, REG> = crate::BitWriter<'a, REG, Brsrcckselect>;
impl<'a, REG> BrsrcckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The baud rate is driven by the peripheral clock"]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Brsrcckselect::PeriphClk)
    }
    #[doc = "The baud rate is driven by a PMC-programmable clock PCK (see section Power Management Controller (PMC))."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut crate::W<REG> {
        self.variant(Brsrcckselect::PmcPck)
    }
}
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chmodeselect {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Automatic echo"]
    Automatic = 1,
    #[doc = "2: Local loopback"]
    LocalLoopback = 2,
    #[doc = "3: Remote loopback"]
    RemoteLoopback = 3,
}
impl From<Chmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chmodeselect {}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type ChmodeR = crate::FieldReader<Chmodeselect>;
impl ChmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chmodeselect {
        match self.bits {
            0 => Chmodeselect::Normal,
            1 => Chmodeselect::Automatic,
            2 => Chmodeselect::LocalLoopback,
            3 => Chmodeselect::RemoteLoopback,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Chmodeselect::Normal
    }
    #[doc = "Automatic echo"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Chmodeselect::Automatic
    }
    #[doc = "Local loopback"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == Chmodeselect::LocalLoopback
    }
    #[doc = "Remote loopback"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == Chmodeselect::RemoteLoopback
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type ChmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chmodeselect, crate::Safe>;
impl<'a, REG> ChmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::Normal)
    }
    #[doc = "Automatic echo"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::Automatic)
    }
    #[doc = "Local loopback"]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::LocalLoopback)
    }
    #[doc = "Remote loopback"]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmodeselect::RemoteLoopback)
    }
}
impl R {
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&self) -> BrsrcckR {
        BrsrcckR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> ChmodeR {
        ChmodeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Receiver Digital Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> FilterW<MrSpec> {
        FilterW::new(self, 4)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&mut self) -> ParW<MrSpec> {
        ParW::new(self, 9)
    }
    #[doc = "Bit 12 - Baud Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcck(&mut self) -> BrsrcckW<MrSpec> {
        BrsrcckW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&mut self) -> ChmodeW<MrSpec> {
        ChmodeW::new(self, 14)
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
