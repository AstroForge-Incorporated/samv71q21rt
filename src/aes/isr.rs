#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `DATRDY` reader - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
pub type DatrdyR = crate::BitReader;
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
pub type UradR = crate::BitReader;
#[doc = "Unspecified Register Access (cleared by writing SWRST in AES_CR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uratselect {
    #[doc = "0: Input Data Register written during the data processing when SMOD = 0x2 mode."]
    IdrWrProcessing = 0,
    #[doc = "1: Output Data Register read during the data processing."]
    OdrRdProcessing = 1,
    #[doc = "2: Mode Register written during the data processing."]
    MrWrProcessing = 2,
    #[doc = "3: Output Data Register read during the sub-keys generation."]
    OdrRdSubkgen = 3,
    #[doc = "4: Mode Register written during the sub-keys generation."]
    MrWrSubkgen = 4,
    #[doc = "5: Write-only register read access."]
    WorRdAccess = 5,
}
impl From<Uratselect> for u8 {
    #[inline(always)]
    fn from(variant: Uratselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uratselect {
    type Ux = u8;
}
impl crate::IsEnum for Uratselect {}
#[doc = "Field `URAT` reader - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
pub type UratR = crate::FieldReader<Uratselect>;
impl UratR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uratselect> {
        match self.bits {
            0 => Some(Uratselect::IdrWrProcessing),
            1 => Some(Uratselect::OdrRdProcessing),
            2 => Some(Uratselect::MrWrProcessing),
            3 => Some(Uratselect::OdrRdSubkgen),
            4 => Some(Uratselect::MrWrSubkgen),
            5 => Some(Uratselect::WorRdAccess),
            _ => None,
        }
    }
    #[doc = "Input Data Register written during the data processing when SMOD = 0x2 mode."]
    #[inline(always)]
    pub fn is_idr_wr_processing(&self) -> bool {
        *self == Uratselect::IdrWrProcessing
    }
    #[doc = "Output Data Register read during the data processing."]
    #[inline(always)]
    pub fn is_odr_rd_processing(&self) -> bool {
        *self == Uratselect::OdrRdProcessing
    }
    #[doc = "Mode Register written during the data processing."]
    #[inline(always)]
    pub fn is_mr_wr_processing(&self) -> bool {
        *self == Uratselect::MrWrProcessing
    }
    #[doc = "Output Data Register read during the sub-keys generation."]
    #[inline(always)]
    pub fn is_odr_rd_subkgen(&self) -> bool {
        *self == Uratselect::OdrRdSubkgen
    }
    #[doc = "Mode Register written during the sub-keys generation."]
    #[inline(always)]
    pub fn is_mr_wr_subkgen(&self) -> bool {
        *self == Uratselect::MrWrSubkgen
    }
    #[doc = "Write-only register read access."]
    #[inline(always)]
    pub fn is_wor_rd_access(&self) -> bool {
        *self == Uratselect::WorRdAccess
    }
}
#[doc = "Field `TAGRDY` reader - GCM Tag Ready"]
pub type TagrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
    #[inline(always)]
    pub fn datrdy(&self) -> DatrdyR {
        DatrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urad(&self) -> UradR {
        UradR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urat(&self) -> UratR {
        UratR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - GCM Tag Ready"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TagrdyR {
        TagrdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
