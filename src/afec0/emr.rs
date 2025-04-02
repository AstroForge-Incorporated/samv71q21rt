#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpmodeselect {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    Low = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    High = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    In = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    Out = 3,
}
impl From<Cmpmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmpmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmpmodeselect {}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CmpmodeR = crate::FieldReader<Cmpmodeselect>;
impl CmpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmodeselect {
        match self.bits {
            0 => Cmpmodeselect::Low,
            1 => Cmpmodeselect::High,
            2 => Cmpmodeselect::In,
            3 => Cmpmodeselect::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cmpmodeselect::Low
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cmpmodeselect::High
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Cmpmodeselect::In
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Cmpmodeselect::Out
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CmpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpmodeselect, crate::Safe>;
impl<'a, REG> CmpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::Low)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::High)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::In)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::Out)
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub type CmpselR = crate::FieldReader;
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub type CmpselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub type CmpallR = crate::BitReader;
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub type CmpallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub type CmpfilterR = crate::FieldReader;
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub type CmpfilterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resselect {
    #[doc = "0: 12-bit resolution, AFE sample rate is maximum (no averaging)."]
    NoAverage = 0,
    #[doc = "2: 13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    Osr4 = 2,
    #[doc = "3: 14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    Osr16 = 3,
    #[doc = "4: 15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    Osr64 = 4,
    #[doc = "5: 16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    Osr256 = 5,
}
impl From<Resselect> for u8 {
    #[inline(always)]
    fn from(variant: Resselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resselect {
    type Ux = u8;
}
impl crate::IsEnum for Resselect {}
#[doc = "Field `RES` reader - Resolution"]
pub type ResR = crate::FieldReader<Resselect>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Resselect> {
        match self.bits {
            0 => Some(Resselect::NoAverage),
            2 => Some(Resselect::Osr4),
            3 => Some(Resselect::Osr16),
            4 => Some(Resselect::Osr64),
            5 => Some(Resselect::Osr256),
            _ => None,
        }
    }
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == Resselect::NoAverage
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        *self == Resselect::Osr4
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        *self == Resselect::Osr16
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == Resselect::Osr64
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == Resselect::Osr256
    }
}
#[doc = "Field `RES` writer - Resolution"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 3, Resselect>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut crate::W<REG> {
        self.variant(Resselect::NoAverage)
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut crate::W<REG> {
        self.variant(Resselect::Osr4)
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut crate::W<REG> {
        self.variant(Resselect::Osr16)
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut crate::W<REG> {
        self.variant(Resselect::Osr64)
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut crate::W<REG> {
        self.variant(Resselect::Osr256)
    }
}
#[doc = "Field `TAG` reader - TAG of the AFEC_LDCR"]
pub type TagR = crate::BitReader;
#[doc = "Field `TAG` writer - TAG of the AFEC_LDCR"]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STM` reader - Single Trigger Mode"]
pub type StmR = crate::BitReader;
#[doc = "Field `STM` writer - Single Trigger Mode"]
pub type StmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sign Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Signmodeselect {
    #[doc = "0: Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    SeUnsgDfSign = 0,
    #[doc = "1: Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    SeSignDfUnsg = 1,
    #[doc = "2: All channels: Unsigned conversions."]
    AllUnsigned = 2,
    #[doc = "3: All channels: Signed conversions."]
    AllSigned = 3,
}
impl From<Signmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Signmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Signmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Signmodeselect {}
#[doc = "Field `SIGNMODE` reader - Sign Mode"]
pub type SignmodeR = crate::FieldReader<Signmodeselect>;
impl SignmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Signmodeselect {
        match self.bits {
            0 => Signmodeselect::SeUnsgDfSign,
            1 => Signmodeselect::SeSignDfUnsg,
            2 => Signmodeselect::AllUnsigned,
            3 => Signmodeselect::AllSigned,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn is_se_unsg_df_sign(&self) -> bool {
        *self == Signmodeselect::SeUnsgDfSign
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn is_se_sign_df_unsg(&self) -> bool {
        *self == Signmodeselect::SeSignDfUnsg
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn is_all_unsigned(&self) -> bool {
        *self == Signmodeselect::AllUnsigned
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn is_all_signed(&self) -> bool {
        *self == Signmodeselect::AllSigned
    }
}
#[doc = "Field `SIGNMODE` writer - Sign Mode"]
pub type SignmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Signmodeselect, crate::Safe>;
impl<'a, REG> SignmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn se_unsg_df_sign(self) -> &'a mut crate::W<REG> {
        self.variant(Signmodeselect::SeUnsgDfSign)
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn se_sign_df_unsg(self) -> &'a mut crate::W<REG> {
        self.variant(Signmodeselect::SeSignDfUnsg)
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn all_unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(Signmodeselect::AllUnsigned)
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn all_signed(self) -> &'a mut crate::W<REG> {
        self.variant(Signmodeselect::AllSigned)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CmpmodeR {
        CmpmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CmpselR {
        CmpselR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CmpallR {
        CmpallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CmpfilterR {
        CmpfilterR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&self) -> SignmodeR {
        SignmodeR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CmpmodeW<EmrSpec> {
        CmpmodeW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CmpselW<EmrSpec> {
        CmpselW::new(self, 3)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> CmpallW<EmrSpec> {
        CmpallW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> CmpfilterW<EmrSpec> {
        CmpfilterW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<EmrSpec> {
        ResW::new(self, 16)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&mut self) -> TagW<EmrSpec> {
        TagW::new(self, 24)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> StmW<EmrSpec> {
        StmW::new(self, 25)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&mut self) -> SignmodeW<EmrSpec> {
        SignmodeW::new(self, 28)
    }
}
#[doc = "AFEC Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {}
