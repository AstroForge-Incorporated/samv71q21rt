#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Selection for Minus Comparator Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selminusselect {
    #[doc = "0: Select TS"]
    Ts = 0,
    #[doc = "1: Select VREFP"]
    Vrefp = 1,
    #[doc = "2: Select DAC0"]
    Dac0 = 2,
    #[doc = "3: Select DAC1"]
    Dac1 = 3,
    #[doc = "4: Select AFE0_AD0"]
    Afe0Ad0 = 4,
    #[doc = "5: Select AFE0_AD1"]
    Afe0Ad1 = 5,
    #[doc = "6: Select AFE0_AD2"]
    Afe0Ad2 = 6,
    #[doc = "7: Select AFE0_AD3"]
    Afe0Ad3 = 7,
}
impl From<Selminusselect> for u8 {
    #[inline(always)]
    fn from(variant: Selminusselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selminusselect {
    type Ux = u8;
}
impl crate::IsEnum for Selminusselect {}
#[doc = "Field `SELMINUS` reader - Selection for Minus Comparator Input"]
pub type SelminusR = crate::FieldReader<Selminusselect>;
impl SelminusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selminusselect {
        match self.bits {
            0 => Selminusselect::Ts,
            1 => Selminusselect::Vrefp,
            2 => Selminusselect::Dac0,
            3 => Selminusselect::Dac1,
            4 => Selminusselect::Afe0Ad0,
            5 => Selminusselect::Afe0Ad1,
            6 => Selminusselect::Afe0Ad2,
            7 => Selminusselect::Afe0Ad3,
            _ => unreachable!(),
        }
    }
    #[doc = "Select TS"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Selminusselect::Ts
    }
    #[doc = "Select VREFP"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == Selminusselect::Vrefp
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == Selminusselect::Dac0
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == Selminusselect::Dac1
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        *self == Selminusselect::Afe0Ad0
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        *self == Selminusselect::Afe0Ad1
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        *self == Selminusselect::Afe0Ad2
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        *self == Selminusselect::Afe0Ad3
    }
}
#[doc = "Field `SELMINUS` writer - Selection for Minus Comparator Input"]
pub type SelminusW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selminusselect, crate::Safe>;
impl<'a, REG> SelminusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select TS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Ts)
    }
    #[doc = "Select VREFP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Vrefp)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Dac0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Dac1)
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Afe0Ad0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Afe0Ad1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Afe0Ad2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut crate::W<REG> {
        self.variant(Selminusselect::Afe0Ad3)
    }
}
#[doc = "Selection For Plus Comparator Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selplusselect {
    #[doc = "0: Select AFE0_AD0"]
    Afe0Ad0 = 0,
    #[doc = "1: Select AFE0_AD1"]
    Afe0Ad1 = 1,
    #[doc = "2: Select AFE0_AD2"]
    Afe0Ad2 = 2,
    #[doc = "3: Select AFE0_AD3"]
    Afe0Ad3 = 3,
    #[doc = "4: Select AFE0_AD4"]
    Afe0Ad4 = 4,
    #[doc = "5: Select AFE0_AD5"]
    Afe0Ad5 = 5,
    #[doc = "6: Select AFE1_AD0"]
    Afe1Ad0 = 6,
    #[doc = "7: Select AFE1_AD1"]
    Afe1Ad1 = 7,
}
impl From<Selplusselect> for u8 {
    #[inline(always)]
    fn from(variant: Selplusselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selplusselect {
    type Ux = u8;
}
impl crate::IsEnum for Selplusselect {}
#[doc = "Field `SELPLUS` reader - Selection For Plus Comparator Input"]
pub type SelplusR = crate::FieldReader<Selplusselect>;
impl SelplusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selplusselect {
        match self.bits {
            0 => Selplusselect::Afe0Ad0,
            1 => Selplusselect::Afe0Ad1,
            2 => Selplusselect::Afe0Ad2,
            3 => Selplusselect::Afe0Ad3,
            4 => Selplusselect::Afe0Ad4,
            5 => Selplusselect::Afe0Ad5,
            6 => Selplusselect::Afe1Ad0,
            7 => Selplusselect::Afe1Ad1,
            _ => unreachable!(),
        }
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        *self == Selplusselect::Afe0Ad0
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        *self == Selplusselect::Afe0Ad1
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        *self == Selplusselect::Afe0Ad2
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        *self == Selplusselect::Afe0Ad3
    }
    #[doc = "Select AFE0_AD4"]
    #[inline(always)]
    pub fn is_afe0_ad4(&self) -> bool {
        *self == Selplusselect::Afe0Ad4
    }
    #[doc = "Select AFE0_AD5"]
    #[inline(always)]
    pub fn is_afe0_ad5(&self) -> bool {
        *self == Selplusselect::Afe0Ad5
    }
    #[doc = "Select AFE1_AD0"]
    #[inline(always)]
    pub fn is_afe1_ad0(&self) -> bool {
        *self == Selplusselect::Afe1Ad0
    }
    #[doc = "Select AFE1_AD1"]
    #[inline(always)]
    pub fn is_afe1_ad1(&self) -> bool {
        *self == Selplusselect::Afe1Ad1
    }
}
#[doc = "Field `SELPLUS` writer - Selection For Plus Comparator Input"]
pub type SelplusW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selplusselect, crate::Safe>;
impl<'a, REG> SelplusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe0Ad0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe0Ad1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe0Ad2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe0Ad3)
    }
    #[doc = "Select AFE0_AD4"]
    #[inline(always)]
    pub fn afe0_ad4(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe0Ad4)
    }
    #[doc = "Select AFE0_AD5"]
    #[inline(always)]
    pub fn afe0_ad5(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe0Ad5)
    }
    #[doc = "Select AFE1_AD0"]
    #[inline(always)]
    pub fn afe1_ad0(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe1Ad0)
    }
    #[doc = "Select AFE1_AD1"]
    #[inline(always)]
    pub fn afe1_ad1(self) -> &'a mut crate::W<REG> {
        self.variant(Selplusselect::Afe1Ad1)
    }
}
#[doc = "Analog Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acenselect {
    #[doc = "0: Analog comparator disabled."]
    Dis = 0,
    #[doc = "1: Analog comparator enabled."]
    En = 1,
}
impl From<Acenselect> for bool {
    #[inline(always)]
    fn from(variant: Acenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACEN` reader - Analog Comparator Enable"]
pub type AcenR = crate::BitReader<Acenselect>;
impl AcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acenselect {
        match self.bits {
            false => Acenselect::Dis,
            true => Acenselect::En,
        }
    }
    #[doc = "Analog comparator disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Acenselect::Dis
    }
    #[doc = "Analog comparator enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Acenselect::En
    }
}
#[doc = "Field `ACEN` writer - Analog Comparator Enable"]
pub type AcenW<'a, REG> = crate::BitWriter<'a, REG, Acenselect>;
impl<'a, REG> AcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog comparator disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Acenselect::Dis)
    }
    #[doc = "Analog comparator enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Acenselect::En)
    }
}
#[doc = "Edge Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgetypselect {
    #[doc = "0: Only rising edge of comparator output"]
    Rising = 0,
    #[doc = "1: Falling edge of comparator output"]
    Falling = 1,
    #[doc = "2: Any edge of comparator output"]
    Any = 2,
}
impl From<Edgetypselect> for u8 {
    #[inline(always)]
    fn from(variant: Edgetypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgetypselect {
    type Ux = u8;
}
impl crate::IsEnum for Edgetypselect {}
#[doc = "Field `EDGETYP` reader - Edge Type"]
pub type EdgetypR = crate::FieldReader<Edgetypselect>;
impl EdgetypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Edgetypselect> {
        match self.bits {
            0 => Some(Edgetypselect::Rising),
            1 => Some(Edgetypselect::Falling),
            2 => Some(Edgetypselect::Any),
            _ => None,
        }
    }
    #[doc = "Only rising edge of comparator output"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Edgetypselect::Rising
    }
    #[doc = "Falling edge of comparator output"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Edgetypselect::Falling
    }
    #[doc = "Any edge of comparator output"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == Edgetypselect::Any
    }
}
#[doc = "Field `EDGETYP` writer - Edge Type"]
pub type EdgetypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgetypselect>;
impl<'a, REG> EdgetypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edgetypselect::Rising)
    }
    #[doc = "Falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Edgetypselect::Falling)
    }
    #[doc = "Any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(Edgetypselect::Any)
    }
}
#[doc = "Invert Comparator Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invselect {
    #[doc = "0: Analog comparator output is directly processed."]
    Dis = 0,
    #[doc = "1: Analog comparator output is inverted prior to being processed."]
    En = 1,
}
impl From<Invselect> for bool {
    #[inline(always)]
    fn from(variant: Invselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert Comparator Output"]
pub type InvR = crate::BitReader<Invselect>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invselect {
        match self.bits {
            false => Invselect::Dis,
            true => Invselect::En,
        }
    }
    #[doc = "Analog comparator output is directly processed."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Invselect::Dis
    }
    #[doc = "Analog comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Invselect::En
    }
}
#[doc = "Field `INV` writer - Invert Comparator Output"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Invselect>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Invselect::Dis)
    }
    #[doc = "Analog comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Invselect::En)
    }
}
#[doc = "Selection Of Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selfsselect {
    #[doc = "0: The CE flag is used to drive the FAULT output."]
    Ce = 0,
    #[doc = "1: The output of the analog comparator flag is used to drive the FAULT output."]
    Output = 1,
}
impl From<Selfsselect> for bool {
    #[inline(always)]
    fn from(variant: Selfsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELFS` reader - Selection Of Fault Source"]
pub type SelfsR = crate::BitReader<Selfsselect>;
impl SelfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selfsselect {
        match self.bits {
            false => Selfsselect::Ce,
            true => Selfsselect::Output,
        }
    }
    #[doc = "The CE flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn is_ce(&self) -> bool {
        *self == Selfsselect::Ce
    }
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Selfsselect::Output
    }
}
#[doc = "Field `SELFS` writer - Selection Of Fault Source"]
pub type SelfsW<'a, REG> = crate::BitWriter<'a, REG, Selfsselect>;
impl<'a, REG> SelfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CE flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn ce(self) -> &'a mut crate::W<REG> {
        self.variant(Selfsselect::Ce)
    }
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Selfsselect::Output)
    }
}
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feselect {
    #[doc = "0: The FAULT output is tied to 0."]
    Dis = 0,
    #[doc = "1: The FAULT output is driven by the signal defined by SELFS."]
    En = 1,
}
impl From<Feselect> for bool {
    #[inline(always)]
    fn from(variant: Feselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Fault Enable"]
pub type FeR = crate::BitReader<Feselect>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Feselect {
        match self.bits {
            false => Feselect::Dis,
            true => Feselect::En,
        }
    }
    #[doc = "The FAULT output is tied to 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Feselect::Dis
    }
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Feselect::En
    }
}
#[doc = "Field `FE` writer - Fault Enable"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Feselect>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Feselect::Dis)
    }
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Feselect::En)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&self) -> SelminusR {
        SelminusR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&self) -> SelplusR {
        SelplusR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&self) -> AcenR {
        AcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EdgetypR {
        EdgetypR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SelfsR {
        SelfsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&mut self) -> SelminusW<MrSpec> {
        SelminusW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&mut self) -> SelplusW<MrSpec> {
        SelplusW::new(self, 4)
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&mut self) -> AcenW<MrSpec> {
        AcenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&mut self) -> EdgetypW<MrSpec> {
        EdgetypW::new(self, 9)
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<MrSpec> {
        InvW::new(self, 12)
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&mut self) -> SelfsW<MrSpec> {
        SelfsW::new(self, 13)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<MrSpec> {
        FeW::new(self, 14)
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
