#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `HSYNC_POL` reader - Horizontal Synchronization Polarity"]
pub type HsyncPolR = crate::BitReader;
#[doc = "Field `HSYNC_POL` writer - Horizontal Synchronization Polarity"]
pub type HsyncPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_POL` reader - Vertical Synchronization Polarity"]
pub type VsyncPolR = crate::BitReader;
#[doc = "Field `VSYNC_POL` writer - Vertical Synchronization Polarity"]
pub type VsyncPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIXCLK_POL` reader - Pixel Clock Polarity"]
pub type PixclkPolR = crate::BitReader;
#[doc = "Field `PIXCLK_POL` writer - Pixel Clock Polarity"]
pub type PixclkPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRAYLE` reader - Grayscale Little Endian"]
pub type GrayleR = crate::BitReader;
#[doc = "Field `GRAYLE` writer - Grayscale Little Endian"]
pub type GrayleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMB_SYNC` reader - Embedded Synchronization"]
pub type EmbSyncR = crate::BitReader;
#[doc = "Field `EMB_SYNC` writer - Embedded Synchronization"]
pub type EmbSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_SYNC` reader - Embedded Synchronization Correction"]
pub type CrcSyncR = crate::BitReader;
#[doc = "Field `CRC_SYNC` writer - Embedded Synchronization Correction"]
pub type CrcSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRATE` reader - Frame Rate \\[0..7\\]"]
pub type FrateR = crate::FieldReader;
#[doc = "Field `FRATE` writer - Frame Rate \\[0..7\\]"]
pub type FrateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DISCR` reader - Disable Codec Request"]
pub type DiscrR = crate::BitReader;
#[doc = "Field `DISCR` writer - Disable Codec Request"]
pub type DiscrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` reader - Full Mode is Allowed"]
pub type FullR = crate::BitReader;
#[doc = "Field `FULL` writer - Full Mode is Allowed"]
pub type FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Threshold Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Thmaskselect {
    #[doc = "0: Only 4 beats AHB burst allowed"]
    Beats4 = 0,
    #[doc = "1: Only 4 and 8 beats AHB burst allowed"]
    Beats8 = 1,
    #[doc = "2: 4, 8 and 16 beats AHB burst allowed"]
    Beats16 = 2,
}
impl From<Thmaskselect> for u8 {
    #[inline(always)]
    fn from(variant: Thmaskselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Thmaskselect {
    type Ux = u8;
}
impl crate::IsEnum for Thmaskselect {}
#[doc = "Field `THMASK` reader - Threshold Mask"]
pub type ThmaskR = crate::FieldReader<Thmaskselect>;
impl ThmaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Thmaskselect> {
        match self.bits {
            0 => Some(Thmaskselect::Beats4),
            1 => Some(Thmaskselect::Beats8),
            2 => Some(Thmaskselect::Beats16),
            _ => None,
        }
    }
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline(always)]
    pub fn is_beats_4(&self) -> bool {
        *self == Thmaskselect::Beats4
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline(always)]
    pub fn is_beats_8(&self) -> bool {
        *self == Thmaskselect::Beats8
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline(always)]
    pub fn is_beats_16(&self) -> bool {
        *self == Thmaskselect::Beats16
    }
}
#[doc = "Field `THMASK` writer - Threshold Mask"]
pub type ThmaskW<'a, REG> = crate::FieldWriter<'a, REG, 2, Thmaskselect>;
impl<'a, REG> ThmaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_4(self) -> &'a mut crate::W<REG> {
        self.variant(Thmaskselect::Beats4)
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_8(self) -> &'a mut crate::W<REG> {
        self.variant(Thmaskselect::Beats8)
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_16(self) -> &'a mut crate::W<REG> {
        self.variant(Thmaskselect::Beats16)
    }
}
#[doc = "Field `SLD` reader - Start of Line Delay"]
pub type SldR = crate::FieldReader;
#[doc = "Field `SLD` writer - Start of Line Delay"]
pub type SldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SFD` reader - Start of Frame Delay"]
pub type SfdR = crate::FieldReader;
#[doc = "Field `SFD` writer - Start of Frame Delay"]
pub type SfdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HsyncPolR {
        HsyncPolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VsyncPolR {
        VsyncPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&self) -> PixclkPolR {
        PixclkPolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&self) -> GrayleR {
        GrayleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&self) -> EmbSyncR {
        EmbSyncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&self) -> CrcSyncR {
        CrcSyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&self) -> FrateR {
        FrateR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&self) -> DiscrR {
        DiscrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&self) -> ThmaskR {
        ThmaskR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&self) -> SldR {
        SldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&self) -> SfdR {
        SfdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&mut self) -> HsyncPolW<Cfg1Spec> {
        HsyncPolW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&mut self) -> VsyncPolW<Cfg1Spec> {
        VsyncPolW::new(self, 3)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&mut self) -> PixclkPolW<Cfg1Spec> {
        PixclkPolW::new(self, 4)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&mut self) -> GrayleW<Cfg1Spec> {
        GrayleW::new(self, 5)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&mut self) -> EmbSyncW<Cfg1Spec> {
        EmbSyncW::new(self, 6)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&mut self) -> CrcSyncW<Cfg1Spec> {
        CrcSyncW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&mut self) -> FrateW<Cfg1Spec> {
        FrateW::new(self, 8)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&mut self) -> DiscrW<Cfg1Spec> {
        DiscrW::new(self, 11)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&mut self) -> FullW<Cfg1Spec> {
        FullW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&mut self) -> ThmaskW<Cfg1Spec> {
        ThmaskW::new(self, 13)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&mut self) -> SldW<Cfg1Spec> {
        SldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SfdW<Cfg1Spec> {
        SfdW::new(self, 24)
    }
}
#[doc = "ISI Configuration 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
