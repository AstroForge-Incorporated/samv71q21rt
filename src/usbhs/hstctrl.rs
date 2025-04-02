#[doc = "Register `HSTCTRL` reader"]
pub type R = crate::R<HstctrlSpec>;
#[doc = "Register `HSTCTRL` writer"]
pub type W = crate::W<HstctrlSpec>;
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SofeR = crate::BitReader;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SofeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Send USB Reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Send USB Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spdconfselect {
    #[doc = "0: The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    Normal = 0,
    #[doc = "1: For a better consumption, if high speed is not needed."]
    LowPower = 1,
    #[doc = "2: Forced high speed."]
    HighSpeed = 2,
    #[doc = "3: The host remains in Full-speed mode whatever the peripheral speed capability."]
    ForcedFs = 3,
}
impl From<Spdconfselect> for u8 {
    #[inline(always)]
    fn from(variant: Spdconfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spdconfselect {
    type Ux = u8;
}
impl crate::IsEnum for Spdconfselect {}
#[doc = "Field `SPDCONF` reader - Mode Configuration"]
pub type SpdconfR = crate::FieldReader<Spdconfselect>;
impl SpdconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spdconfselect {
        match self.bits {
            0 => Spdconfselect::Normal,
            1 => Spdconfselect::LowPower,
            2 => Spdconfselect::HighSpeed,
            3 => Spdconfselect::ForcedFs,
            _ => unreachable!(),
        }
    }
    #[doc = "The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Spdconfselect::Normal
    }
    #[doc = "For a better consumption, if high speed is not needed."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Spdconfselect::LowPower
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Spdconfselect::HighSpeed
    }
    #[doc = "The host remains in Full-speed mode whatever the peripheral speed capability."]
    #[inline(always)]
    pub fn is_forced_fs(&self) -> bool {
        *self == Spdconfselect::ForcedFs
    }
}
#[doc = "Field `SPDCONF` writer - Mode Configuration"]
pub type SpdconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spdconfselect, crate::Safe>;
impl<'a, REG> SpdconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::Normal)
    }
    #[doc = "For a better consumption, if high speed is not needed."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::LowPower)
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::HighSpeed)
    }
    #[doc = "The host remains in Full-speed mode whatever the peripheral speed capability."]
    #[inline(always)]
    pub fn forced_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconfselect::ForcedFs)
    }
}
impl R {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SpdconfR {
        SpdconfR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SofeW<HstctrlSpec> {
        SofeW::new(self, 8)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<HstctrlSpec> {
        ResetW::new(self, 9)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<HstctrlSpec> {
        ResumeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SpdconfW<HstctrlSpec> {
        SpdconfW::new(self, 12)
    }
}
#[doc = "Host General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstctrlSpec;
impl crate::RegisterSpec for HstctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstctrl::R`](R) reader structure"]
impl crate::Readable for HstctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hstctrl::W`](W) writer structure"]
impl crate::Writable for HstctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTCTRL to value 0"]
impl crate::Resettable for HstctrlSpec {}
