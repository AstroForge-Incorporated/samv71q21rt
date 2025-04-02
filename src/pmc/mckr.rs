#[doc = "Register `MCKR` reader"]
pub type R = crate::R<MckrSpec>;
#[doc = "Register `MCKR` writer"]
pub type W = crate::W<MckrSpec>;
#[doc = "Master Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cssselect {
    #[doc = "0: Slow Clock is selected"]
    SlowClk = 0,
    #[doc = "1: Main Clock is selected"]
    MainClk = 1,
    #[doc = "2: PLLA Clock is selected"]
    PllaClk = 2,
    #[doc = "3: Divided UPLL Clock is selected"]
    UpllClk = 3,
}
impl From<Cssselect> for u8 {
    #[inline(always)]
    fn from(variant: Cssselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cssselect {
    type Ux = u8;
}
impl crate::IsEnum for Cssselect {}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CssR = crate::FieldReader<Cssselect>;
impl CssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cssselect {
        match self.bits {
            0 => Cssselect::SlowClk,
            1 => Cssselect::MainClk,
            2 => Cssselect::PllaClk,
            3 => Cssselect::UpllClk,
            _ => unreachable!(),
        }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == Cssselect::SlowClk
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Cssselect::MainClk
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == Cssselect::PllaClk
    }
    #[doc = "Divided UPLL Clock is selected"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == Cssselect::UpllClk
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cssselect, crate::Safe>;
impl<'a, REG> CssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::SlowClk)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::MainClk)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::PllaClk)
    }
    #[doc = "Divided UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Cssselect::UpllClk)
    }
}
#[doc = "Processor Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presselect {
    #[doc = "0: Selected clock"]
    Clk1 = 0,
    #[doc = "1: Selected clock divided by 2"]
    Clk2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    Clk4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    Clk8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    Clk16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    Clk32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    Clk64 = 6,
    #[doc = "7: Selected clock divided by 3"]
    Clk3 = 7,
}
impl From<Presselect> for u8 {
    #[inline(always)]
    fn from(variant: Presselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presselect {
    type Ux = u8;
}
impl crate::IsEnum for Presselect {}
#[doc = "Field `PRES` reader - Processor Clock Prescaler"]
pub type PresR = crate::FieldReader<Presselect>;
impl PresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presselect {
        match self.bits {
            0 => Presselect::Clk1,
            1 => Presselect::Clk2,
            2 => Presselect::Clk4,
            3 => Presselect::Clk8,
            4 => Presselect::Clk16,
            5 => Presselect::Clk32,
            6 => Presselect::Clk64,
            7 => Presselect::Clk3,
            _ => unreachable!(),
        }
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == Presselect::Clk1
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == Presselect::Clk2
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == Presselect::Clk4
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == Presselect::Clk8
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == Presselect::Clk16
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == Presselect::Clk32
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == Presselect::Clk64
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == Presselect::Clk3
    }
}
#[doc = "Field `PRES` writer - Processor Clock Prescaler"]
pub type PresW<'a, REG> = crate::FieldWriter<'a, REG, 3, Presselect, crate::Safe>;
impl<'a, REG> PresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut crate::W<REG> {
        self.variant(Presselect::Clk3)
    }
}
#[doc = "Master Clock Division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdivselect {
    #[doc = "0: Master Clock is Prescaler Output Clock divided by 1."]
    EqPck = 0,
    #[doc = "1: Master Clock is Prescaler Output Clock divided by 2."]
    PckDiv2 = 1,
    #[doc = "2: Master Clock is Prescaler Output Clock divided by 4."]
    PckDiv4 = 2,
    #[doc = "3: Master Clock is Prescaler Output Clock divided by 3."]
    PckDiv3 = 3,
}
impl From<Mdivselect> for u8 {
    #[inline(always)]
    fn from(variant: Mdivselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdivselect {
    type Ux = u8;
}
impl crate::IsEnum for Mdivselect {}
#[doc = "Field `MDIV` reader - Master Clock Division"]
pub type MdivR = crate::FieldReader<Mdivselect>;
impl MdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdivselect {
        match self.bits {
            0 => Mdivselect::EqPck,
            1 => Mdivselect::PckDiv2,
            2 => Mdivselect::PckDiv4,
            3 => Mdivselect::PckDiv3,
            _ => unreachable!(),
        }
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    #[inline(always)]
    pub fn is_eq_pck(&self) -> bool {
        *self == Mdivselect::EqPck
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    #[inline(always)]
    pub fn is_pck_div2(&self) -> bool {
        *self == Mdivselect::PckDiv2
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    #[inline(always)]
    pub fn is_pck_div4(&self) -> bool {
        *self == Mdivselect::PckDiv4
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    #[inline(always)]
    pub fn is_pck_div3(&self) -> bool {
        *self == Mdivselect::PckDiv3
    }
}
#[doc = "Field `MDIV` writer - Master Clock Division"]
pub type MdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mdivselect, crate::Safe>;
impl<'a, REG> MdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock is Prescaler Output Clock divided by 1."]
    #[inline(always)]
    pub fn eq_pck(self) -> &'a mut crate::W<REG> {
        self.variant(Mdivselect::EqPck)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 2."]
    #[inline(always)]
    pub fn pck_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Mdivselect::PckDiv2)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 4."]
    #[inline(always)]
    pub fn pck_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Mdivselect::PckDiv4)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 3."]
    #[inline(always)]
    pub fn pck_div3(self) -> &'a mut crate::W<REG> {
        self.variant(Mdivselect::PckDiv3)
    }
}
#[doc = "Field `UPLLDIV2` reader - UPLL Divider by 2"]
pub type Uplldiv2R = crate::BitReader;
#[doc = "Field `UPLLDIV2` writer - UPLL Divider by 2"]
pub type Uplldiv2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&self) -> MdivR {
        MdivR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline(always)]
    pub fn uplldiv2(&self) -> Uplldiv2R {
        Uplldiv2R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<MckrSpec> {
        CssW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PresW<MckrSpec> {
        PresW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MdivW<MckrSpec> {
        MdivW::new(self, 8)
    }
    #[doc = "Bit 13 - UPLL Divider by 2"]
    #[inline(always)]
    pub fn uplldiv2(&mut self) -> Uplldiv2W<MckrSpec> {
        Uplldiv2W::new(self, 13)
    }
}
#[doc = "Master Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MckrSpec;
impl crate::RegisterSpec for MckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mckr::R`](R) reader structure"]
impl crate::Readable for MckrSpec {}
#[doc = "`write(|w| ..)` method takes [`mckr::W`](W) writer structure"]
impl crate::Writable for MckrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCKR to value 0"]
impl crate::Resettable for MckrSpec {}
