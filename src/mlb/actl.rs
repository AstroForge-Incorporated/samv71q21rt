#[doc = "Register `ACTL` reader"]
pub type R = crate::R<ActlSpec>;
#[doc = "Register `ACTL` writer"]
pub type W = crate::W<ActlSpec>;
#[doc = "Field `SCE` reader - Software Clear Enable"]
pub type SceR = crate::BitReader;
#[doc = "Field `SCE` writer - Software Clear Enable"]
pub type SceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMX` reader - AHB Interrupt Mux Enable"]
pub type SmxR = crate::BitReader;
#[doc = "Field `SMX` writer - AHB Interrupt Mux Enable"]
pub type SmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MODE` reader - DMA Mode"]
pub type DmaModeR = crate::BitReader;
#[doc = "Field `DMA_MODE` writer - DMA Mode"]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Packet Buffering Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpbselect {
    #[doc = "0: Single-packet mode"]
    SinglePacket = 0,
    #[doc = "1: Multiple-packet mode"]
    MultiplePacket = 1,
}
impl From<Mpbselect> for bool {
    #[inline(always)]
    fn from(variant: Mpbselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPB` reader - DMA Packet Buffering Mode"]
pub type MpbR = crate::BitReader<Mpbselect>;
impl MpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpbselect {
        match self.bits {
            false => Mpbselect::SinglePacket,
            true => Mpbselect::MultiplePacket,
        }
    }
    #[doc = "Single-packet mode"]
    #[inline(always)]
    pub fn is_single_packet(&self) -> bool {
        *self == Mpbselect::SinglePacket
    }
    #[doc = "Multiple-packet mode"]
    #[inline(always)]
    pub fn is_multiple_packet(&self) -> bool {
        *self == Mpbselect::MultiplePacket
    }
}
#[doc = "Field `MPB` writer - DMA Packet Buffering Mode"]
pub type MpbW<'a, REG> = crate::BitWriter<'a, REG, Mpbselect>;
impl<'a, REG> MpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-packet mode"]
    #[inline(always)]
    pub fn single_packet(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbselect::SinglePacket)
    }
    #[doc = "Multiple-packet mode"]
    #[inline(always)]
    pub fn multiple_packet(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbselect::MultiplePacket)
    }
}
impl R {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&self) -> SceR {
        SceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&self) -> SmxR {
        SmxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&mut self) -> SceW<ActlSpec> {
        SceW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&mut self) -> SmxW<ActlSpec> {
        SmxW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<ActlSpec> {
        DmaModeW::new(self, 2)
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&mut self) -> MpbW<ActlSpec> {
        MpbW::new(self, 4)
    }
}
#[doc = "AHB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActlSpec;
impl crate::RegisterSpec for ActlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actl::R`](R) reader structure"]
impl crate::Readable for ActlSpec {}
#[doc = "`write(|w| ..)` method takes [`actl::W`](W) writer structure"]
impl crate::Writable for ActlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACTL to value 0"]
impl crate::Resettable for ActlSpec {}
