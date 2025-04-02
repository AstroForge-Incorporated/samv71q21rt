#[doc = "Register `SMMR` reader"]
pub type R = crate::R<SmmrSpec>;
#[doc = "Register `SMMR` writer"]
pub type W = crate::W<SmmrSpec>;
#[doc = "Field `SMTH` reader - Supply Monitor Threshold"]
pub type SmthR = crate::FieldReader;
#[doc = "Field `SMTH` writer - Supply Monitor Threshold"]
pub type SmthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Supply Monitor Sampling Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smsmplselect {
    #[doc = "0: Supply Monitor disabled"]
    Smd = 0,
    #[doc = "1: Continuous Supply Monitor"]
    Csm = 1,
    #[doc = "2: Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32slck = 2,
    #[doc = "3: Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256slck = 3,
    #[doc = "4: Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048slck = 4,
}
impl From<Smsmplselect> for u8 {
    #[inline(always)]
    fn from(variant: Smsmplselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smsmplselect {
    type Ux = u8;
}
impl crate::IsEnum for Smsmplselect {}
#[doc = "Field `SMSMPL` reader - Supply Monitor Sampling Period"]
pub type SmsmplR = crate::FieldReader<Smsmplselect>;
impl SmsmplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smsmplselect> {
        match self.bits {
            0 => Some(Smsmplselect::Smd),
            1 => Some(Smsmplselect::Csm),
            2 => Some(Smsmplselect::_32slck),
            3 => Some(Smsmplselect::_256slck),
            4 => Some(Smsmplselect::_2048slck),
            _ => None,
        }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        *self == Smsmplselect::Smd
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        *self == Smsmplselect::Csm
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        *self == Smsmplselect::_32slck
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        *self == Smsmplselect::_256slck
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        *self == Smsmplselect::_2048slck
    }
}
#[doc = "Field `SMSMPL` writer - Supply Monitor Sampling Period"]
pub type SmsmplW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smsmplselect>;
impl<'a, REG> SmsmplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmplselect::Smd)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmplselect::Csm)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmplselect::_32slck)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmplselect::_256slck)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmplselect::_2048slck)
    }
}
#[doc = "Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smrstenselect {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    Enable = 1,
}
impl From<Smrstenselect> for bool {
    #[inline(always)]
    fn from(variant: Smrstenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTEN` reader - Supply Monitor Reset Enable"]
pub type SmrstenR = crate::BitReader<Smrstenselect>;
impl SmrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smrstenselect {
        match self.bits {
            false => Smrstenselect::NotEnable,
            true => Smrstenselect::Enable,
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smrstenselect::NotEnable
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smrstenselect::Enable
    }
}
#[doc = "Field `SMRSTEN` writer - Supply Monitor Reset Enable"]
pub type SmrstenW<'a, REG> = crate::BitWriter<'a, REG, Smrstenselect>;
impl<'a, REG> SmrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smrstenselect::NotEnable)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smrstenselect::Enable)
    }
}
#[doc = "Supply Monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smienselect {
    #[doc = "0: The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    Enable = 1,
}
impl From<Smienselect> for bool {
    #[inline(always)]
    fn from(variant: Smienselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMIEN` reader - Supply Monitor Interrupt Enable"]
pub type SmienR = crate::BitReader<Smienselect>;
impl SmienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smienselect {
        match self.bits {
            false => Smienselect::NotEnable,
            true => Smienselect::Enable,
        }
    }
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smienselect::NotEnable
    }
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smienselect::Enable
    }
}
#[doc = "Field `SMIEN` writer - Supply Monitor Interrupt Enable"]
pub type SmienW<'a, REG> = crate::BitWriter<'a, REG, Smienselect>;
impl<'a, REG> SmienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smienselect::NotEnable)
    }
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smienselect::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SmthR {
        SmthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SmsmplR {
        SmsmplR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SmrstenR {
        SmrstenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SmienR {
        SmienR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&mut self) -> SmthW<SmmrSpec> {
        SmthW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&mut self) -> SmsmplW<SmmrSpec> {
        SmsmplW::new(self, 8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&mut self) -> SmrstenW<SmmrSpec> {
        SmrstenW::new(self, 12)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&mut self) -> SmienW<SmmrSpec> {
        SmienW::new(self, 13)
    }
}
#[doc = "Supply Controller Supply Monitor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmmrSpec;
impl crate::RegisterSpec for SmmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr::R`](R) reader structure"]
impl crate::Readable for SmmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smmr::W`](W) writer structure"]
impl crate::Writable for SmmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SmmrSpec {}
