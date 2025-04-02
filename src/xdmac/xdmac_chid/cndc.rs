#[doc = "Register `CNDC` reader"]
pub type R = crate::R<CndcSpec>;
#[doc = "Register `CNDC` writer"]
pub type W = crate::W<CndcSpec>;
#[doc = "Channel x Next Descriptor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ndeselect {
    #[doc = "0: Descriptor fetch is disabled."]
    DscrFetchDis = 0,
    #[doc = "1: Descriptor fetch is enabled."]
    DscrFetchEn = 1,
}
impl From<Ndeselect> for bool {
    #[inline(always)]
    fn from(variant: Ndeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDE` reader - Channel x Next Descriptor Enable"]
pub type NdeR = crate::BitReader<Ndeselect>;
impl NdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ndeselect {
        match self.bits {
            false => Ndeselect::DscrFetchDis,
            true => Ndeselect::DscrFetchEn,
        }
    }
    #[doc = "Descriptor fetch is disabled."]
    #[inline(always)]
    pub fn is_dscr_fetch_dis(&self) -> bool {
        *self == Ndeselect::DscrFetchDis
    }
    #[doc = "Descriptor fetch is enabled."]
    #[inline(always)]
    pub fn is_dscr_fetch_en(&self) -> bool {
        *self == Ndeselect::DscrFetchEn
    }
}
#[doc = "Field `NDE` writer - Channel x Next Descriptor Enable"]
pub type NdeW<'a, REG> = crate::BitWriter<'a, REG, Ndeselect>;
impl<'a, REG> NdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Descriptor fetch is disabled."]
    #[inline(always)]
    pub fn dscr_fetch_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ndeselect::DscrFetchDis)
    }
    #[doc = "Descriptor fetch is enabled."]
    #[inline(always)]
    pub fn dscr_fetch_en(self) -> &'a mut crate::W<REG> {
        self.variant(Ndeselect::DscrFetchEn)
    }
}
#[doc = "Channel x Next Descriptor Source Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ndsupselect {
    #[doc = "0: Source parameters remain unchanged."]
    SrcParamsUnchanged = 0,
    #[doc = "1: Source parameters are updated when the descriptor is retrieved."]
    SrcParamsUpdated = 1,
}
impl From<Ndsupselect> for bool {
    #[inline(always)]
    fn from(variant: Ndsupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDSUP` reader - Channel x Next Descriptor Source Update"]
pub type NdsupR = crate::BitReader<Ndsupselect>;
impl NdsupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ndsupselect {
        match self.bits {
            false => Ndsupselect::SrcParamsUnchanged,
            true => Ndsupselect::SrcParamsUpdated,
        }
    }
    #[doc = "Source parameters remain unchanged."]
    #[inline(always)]
    pub fn is_src_params_unchanged(&self) -> bool {
        *self == Ndsupselect::SrcParamsUnchanged
    }
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn is_src_params_updated(&self) -> bool {
        *self == Ndsupselect::SrcParamsUpdated
    }
}
#[doc = "Field `NDSUP` writer - Channel x Next Descriptor Source Update"]
pub type NdsupW<'a, REG> = crate::BitWriter<'a, REG, Ndsupselect>;
impl<'a, REG> NdsupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source parameters remain unchanged."]
    #[inline(always)]
    pub fn src_params_unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Ndsupselect::SrcParamsUnchanged)
    }
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn src_params_updated(self) -> &'a mut crate::W<REG> {
        self.variant(Ndsupselect::SrcParamsUpdated)
    }
}
#[doc = "Channel x Next Descriptor Destination Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nddupselect {
    #[doc = "0: Destination parameters remain unchanged."]
    DstParamsUnchanged = 0,
    #[doc = "1: Destination parameters are updated when the descriptor is retrieved."]
    DstParamsUpdated = 1,
}
impl From<Nddupselect> for bool {
    #[inline(always)]
    fn from(variant: Nddupselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDDUP` reader - Channel x Next Descriptor Destination Update"]
pub type NddupR = crate::BitReader<Nddupselect>;
impl NddupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nddupselect {
        match self.bits {
            false => Nddupselect::DstParamsUnchanged,
            true => Nddupselect::DstParamsUpdated,
        }
    }
    #[doc = "Destination parameters remain unchanged."]
    #[inline(always)]
    pub fn is_dst_params_unchanged(&self) -> bool {
        *self == Nddupselect::DstParamsUnchanged
    }
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn is_dst_params_updated(&self) -> bool {
        *self == Nddupselect::DstParamsUpdated
    }
}
#[doc = "Field `NDDUP` writer - Channel x Next Descriptor Destination Update"]
pub type NddupW<'a, REG> = crate::BitWriter<'a, REG, Nddupselect>;
impl<'a, REG> NddupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Destination parameters remain unchanged."]
    #[inline(always)]
    pub fn dst_params_unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Nddupselect::DstParamsUnchanged)
    }
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn dst_params_updated(self) -> &'a mut crate::W<REG> {
        self.variant(Nddupselect::DstParamsUpdated)
    }
}
#[doc = "Channel x Next Descriptor View\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ndviewselect {
    #[doc = "0: Next Descriptor View 0"]
    Ndv0 = 0,
    #[doc = "1: Next Descriptor View 1"]
    Ndv1 = 1,
    #[doc = "2: Next Descriptor View 2"]
    Ndv2 = 2,
    #[doc = "3: Next Descriptor View 3"]
    Ndv3 = 3,
}
impl From<Ndviewselect> for u8 {
    #[inline(always)]
    fn from(variant: Ndviewselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ndviewselect {
    type Ux = u8;
}
impl crate::IsEnum for Ndviewselect {}
#[doc = "Field `NDVIEW` reader - Channel x Next Descriptor View"]
pub type NdviewR = crate::FieldReader<Ndviewselect>;
impl NdviewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ndviewselect {
        match self.bits {
            0 => Ndviewselect::Ndv0,
            1 => Ndviewselect::Ndv1,
            2 => Ndviewselect::Ndv2,
            3 => Ndviewselect::Ndv3,
            _ => unreachable!(),
        }
    }
    #[doc = "Next Descriptor View 0"]
    #[inline(always)]
    pub fn is_ndv0(&self) -> bool {
        *self == Ndviewselect::Ndv0
    }
    #[doc = "Next Descriptor View 1"]
    #[inline(always)]
    pub fn is_ndv1(&self) -> bool {
        *self == Ndviewselect::Ndv1
    }
    #[doc = "Next Descriptor View 2"]
    #[inline(always)]
    pub fn is_ndv2(&self) -> bool {
        *self == Ndviewselect::Ndv2
    }
    #[doc = "Next Descriptor View 3"]
    #[inline(always)]
    pub fn is_ndv3(&self) -> bool {
        *self == Ndviewselect::Ndv3
    }
}
#[doc = "Field `NDVIEW` writer - Channel x Next Descriptor View"]
pub type NdviewW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ndviewselect, crate::Safe>;
impl<'a, REG> NdviewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Next Descriptor View 0"]
    #[inline(always)]
    pub fn ndv0(self) -> &'a mut crate::W<REG> {
        self.variant(Ndviewselect::Ndv0)
    }
    #[doc = "Next Descriptor View 1"]
    #[inline(always)]
    pub fn ndv1(self) -> &'a mut crate::W<REG> {
        self.variant(Ndviewselect::Ndv1)
    }
    #[doc = "Next Descriptor View 2"]
    #[inline(always)]
    pub fn ndv2(self) -> &'a mut crate::W<REG> {
        self.variant(Ndviewselect::Ndv2)
    }
    #[doc = "Next Descriptor View 3"]
    #[inline(always)]
    pub fn ndv3(self) -> &'a mut crate::W<REG> {
        self.variant(Ndviewselect::Ndv3)
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline(always)]
    pub fn nde(&self) -> NdeR {
        NdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline(always)]
    pub fn ndsup(&self) -> NdsupR {
        NdsupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline(always)]
    pub fn nddup(&self) -> NddupR {
        NddupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline(always)]
    pub fn ndview(&self) -> NdviewR {
        NdviewR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline(always)]
    pub fn nde(&mut self) -> NdeW<CndcSpec> {
        NdeW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline(always)]
    pub fn ndsup(&mut self) -> NdsupW<CndcSpec> {
        NdsupW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline(always)]
    pub fn nddup(&mut self) -> NddupW<CndcSpec> {
        NddupW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline(always)]
    pub fn ndview(&mut self) -> NdviewW<CndcSpec> {
        NdviewW::new(self, 3)
    }
}
#[doc = "Channel Next Descriptor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CndcSpec;
impl crate::RegisterSpec for CndcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndc::R`](R) reader structure"]
impl crate::Readable for CndcSpec {}
#[doc = "`write(|w| ..)` method takes [`cndc::W`](W) writer structure"]
impl crate::Writable for CndcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDC to value 0"]
impl crate::Resettable for CndcSpec {}
