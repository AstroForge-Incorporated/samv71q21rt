#[doc = "Register `CLK` reader"]
pub type R = crate::R<ClkSpec>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<ClkSpec>;
#[doc = "CLKA Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divaselect {
    #[doc = "0: CLKA clock is turned off"]
    ClkaPoff = 0,
    #[doc = "1: CLKA clock is clock selected by PREA"]
    Prea = 1,
}
impl From<Divaselect> for u8 {
    #[inline(always)]
    fn from(variant: Divaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divaselect {
    type Ux = u8;
}
impl crate::IsEnum for Divaselect {}
#[doc = "Field `DIVA` reader - CLKA Divide Factor"]
pub type DivaR = crate::FieldReader<Divaselect>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divaselect> {
        match self.bits {
            0 => Some(Divaselect::ClkaPoff),
            1 => Some(Divaselect::Prea),
            _ => None,
        }
    }
    #[doc = "CLKA clock is turned off"]
    #[inline(always)]
    pub fn is_clka_poff(&self) -> bool {
        *self == Divaselect::ClkaPoff
    }
    #[doc = "CLKA clock is clock selected by PREA"]
    #[inline(always)]
    pub fn is_prea(&self) -> bool {
        *self == Divaselect::Prea
    }
}
#[doc = "Field `DIVA` writer - CLKA Divide Factor"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divaselect>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKA clock is turned off"]
    #[inline(always)]
    pub fn clka_poff(self) -> &'a mut crate::W<REG> {
        self.variant(Divaselect::ClkaPoff)
    }
    #[doc = "CLKA clock is clock selected by PREA"]
    #[inline(always)]
    pub fn prea(self) -> &'a mut crate::W<REG> {
        self.variant(Divaselect::Prea)
    }
}
#[doc = "CLKA Source Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preaselect {
    #[doc = "0: Peripheral clock"]
    Clk = 0,
    #[doc = "1: Peripheral clock/2"]
    ClkDiv2 = 1,
    #[doc = "2: Peripheral clock/4"]
    ClkDiv4 = 2,
    #[doc = "3: Peripheral clock/8"]
    ClkDiv8 = 3,
    #[doc = "4: Peripheral clock/16"]
    ClkDiv16 = 4,
    #[doc = "5: Peripheral clock/32"]
    ClkDiv32 = 5,
    #[doc = "6: Peripheral clock/64"]
    ClkDiv64 = 6,
    #[doc = "7: Peripheral clock/128"]
    ClkDiv128 = 7,
    #[doc = "8: Peripheral clock/256"]
    ClkDiv256 = 8,
    #[doc = "9: Peripheral clock/512"]
    ClkDiv512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    ClkDiv1024 = 10,
}
impl From<Preaselect> for u8 {
    #[inline(always)]
    fn from(variant: Preaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Preaselect {
    type Ux = u8;
}
impl crate::IsEnum for Preaselect {}
#[doc = "Field `PREA` reader - CLKA Source Clock Selection"]
pub type PreaR = crate::FieldReader<Preaselect>;
impl PreaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preaselect> {
        match self.bits {
            0 => Some(Preaselect::Clk),
            1 => Some(Preaselect::ClkDiv2),
            2 => Some(Preaselect::ClkDiv4),
            3 => Some(Preaselect::ClkDiv8),
            4 => Some(Preaselect::ClkDiv16),
            5 => Some(Preaselect::ClkDiv32),
            6 => Some(Preaselect::ClkDiv64),
            7 => Some(Preaselect::ClkDiv128),
            8 => Some(Preaselect::ClkDiv256),
            9 => Some(Preaselect::ClkDiv512),
            10 => Some(Preaselect::ClkDiv1024),
            _ => None,
        }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Preaselect::Clk
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn is_clk_div2(&self) -> bool {
        *self == Preaselect::ClkDiv2
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn is_clk_div4(&self) -> bool {
        *self == Preaselect::ClkDiv4
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == Preaselect::ClkDiv8
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn is_clk_div16(&self) -> bool {
        *self == Preaselect::ClkDiv16
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn is_clk_div32(&self) -> bool {
        *self == Preaselect::ClkDiv32
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn is_clk_div64(&self) -> bool {
        *self == Preaselect::ClkDiv64
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn is_clk_div128(&self) -> bool {
        *self == Preaselect::ClkDiv128
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn is_clk_div256(&self) -> bool {
        *self == Preaselect::ClkDiv256
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn is_clk_div512(&self) -> bool {
        *self == Preaselect::ClkDiv512
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn is_clk_div1024(&self) -> bool {
        *self == Preaselect::ClkDiv1024
    }
}
#[doc = "Field `PREA` writer - CLKA Source Clock Selection"]
pub type PreaW<'a, REG> = crate::FieldWriter<'a, REG, 4, Preaselect>;
impl<'a, REG> PreaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn clk(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::Clk)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn clk_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn clk_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn clk_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn clk_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn clk_div64(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn clk_div128(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn clk_div256(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn clk_div512(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn clk_div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Preaselect::ClkDiv1024)
    }
}
#[doc = "CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divbselect {
    #[doc = "0: CLKB clock is turned off"]
    ClkbPoff = 0,
    #[doc = "1: CLKB clock is clock selected by PREB"]
    Preb = 1,
}
impl From<Divbselect> for u8 {
    #[inline(always)]
    fn from(variant: Divbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divbselect {
    type Ux = u8;
}
impl crate::IsEnum for Divbselect {}
#[doc = "Field `DIVB` reader - CLKB Divide Factor"]
pub type DivbR = crate::FieldReader<Divbselect>;
impl DivbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divbselect> {
        match self.bits {
            0 => Some(Divbselect::ClkbPoff),
            1 => Some(Divbselect::Preb),
            _ => None,
        }
    }
    #[doc = "CLKB clock is turned off"]
    #[inline(always)]
    pub fn is_clkb_poff(&self) -> bool {
        *self == Divbselect::ClkbPoff
    }
    #[doc = "CLKB clock is clock selected by PREB"]
    #[inline(always)]
    pub fn is_preb(&self) -> bool {
        *self == Divbselect::Preb
    }
}
#[doc = "Field `DIVB` writer - CLKB Divide Factor"]
pub type DivbW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divbselect>;
impl<'a, REG> DivbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKB clock is turned off"]
    #[inline(always)]
    pub fn clkb_poff(self) -> &'a mut crate::W<REG> {
        self.variant(Divbselect::ClkbPoff)
    }
    #[doc = "CLKB clock is clock selected by PREB"]
    #[inline(always)]
    pub fn preb(self) -> &'a mut crate::W<REG> {
        self.variant(Divbselect::Preb)
    }
}
#[doc = "CLKB Source Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prebselect {
    #[doc = "0: Peripheral clock"]
    Clk = 0,
    #[doc = "1: Peripheral clock/2"]
    ClkDiv2 = 1,
    #[doc = "2: Peripheral clock/4"]
    ClkDiv4 = 2,
    #[doc = "3: Peripheral clock/8"]
    ClkDiv8 = 3,
    #[doc = "4: Peripheral clock/16"]
    ClkDiv16 = 4,
    #[doc = "5: Peripheral clock/32"]
    ClkDiv32 = 5,
    #[doc = "6: Peripheral clock/64"]
    ClkDiv64 = 6,
    #[doc = "7: Peripheral clock/128"]
    ClkDiv128 = 7,
    #[doc = "8: Peripheral clock/256"]
    ClkDiv256 = 8,
    #[doc = "9: Peripheral clock/512"]
    ClkDiv512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    ClkDiv1024 = 10,
}
impl From<Prebselect> for u8 {
    #[inline(always)]
    fn from(variant: Prebselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prebselect {
    type Ux = u8;
}
impl crate::IsEnum for Prebselect {}
#[doc = "Field `PREB` reader - CLKB Source Clock Selection"]
pub type PrebR = crate::FieldReader<Prebselect>;
impl PrebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prebselect> {
        match self.bits {
            0 => Some(Prebselect::Clk),
            1 => Some(Prebselect::ClkDiv2),
            2 => Some(Prebselect::ClkDiv4),
            3 => Some(Prebselect::ClkDiv8),
            4 => Some(Prebselect::ClkDiv16),
            5 => Some(Prebselect::ClkDiv32),
            6 => Some(Prebselect::ClkDiv64),
            7 => Some(Prebselect::ClkDiv128),
            8 => Some(Prebselect::ClkDiv256),
            9 => Some(Prebselect::ClkDiv512),
            10 => Some(Prebselect::ClkDiv1024),
            _ => None,
        }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Prebselect::Clk
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn is_clk_div2(&self) -> bool {
        *self == Prebselect::ClkDiv2
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn is_clk_div4(&self) -> bool {
        *self == Prebselect::ClkDiv4
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == Prebselect::ClkDiv8
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn is_clk_div16(&self) -> bool {
        *self == Prebselect::ClkDiv16
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn is_clk_div32(&self) -> bool {
        *self == Prebselect::ClkDiv32
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn is_clk_div64(&self) -> bool {
        *self == Prebselect::ClkDiv64
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn is_clk_div128(&self) -> bool {
        *self == Prebselect::ClkDiv128
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn is_clk_div256(&self) -> bool {
        *self == Prebselect::ClkDiv256
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn is_clk_div512(&self) -> bool {
        *self == Prebselect::ClkDiv512
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn is_clk_div1024(&self) -> bool {
        *self == Prebselect::ClkDiv1024
    }
}
#[doc = "Field `PREB` writer - CLKB Source Clock Selection"]
pub type PrebW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prebselect>;
impl<'a, REG> PrebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn clk(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::Clk)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn clk_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn clk_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn clk_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn clk_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn clk_div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn clk_div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn clk_div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn clk_div512(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn clk_div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prebselect::ClkDiv1024)
    }
}
impl R {
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&self) -> PreaR {
        PreaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DivbR {
        DivbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&self) -> PrebR {
        PrebR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<ClkSpec> {
        DivaW::new(self, 0)
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&mut self) -> PreaW<ClkSpec> {
        PreaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> DivbW<ClkSpec> {
        DivbW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&mut self) -> PrebW<ClkSpec> {
        PrebW::new(self, 24)
    }
}
#[doc = "PWM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSpec;
impl crate::RegisterSpec for ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for ClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for ClkSpec {}
