#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `CIPHER` reader - Processing Mode"]
pub type CipherR = crate::BitReader;
#[doc = "Field `CIPHER` writer - Processing Mode"]
pub type CipherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTAGEN` reader - GCM Automatic Tag Generation Enable"]
pub type GtagenR = crate::BitReader;
#[doc = "Field `GTAGEN` writer - GCM Automatic Tag Generation Enable"]
pub type GtagenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Dual Input Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dualbuffselect {
    #[doc = "0: AES_IDATARx cannot be written during processing of previous block."]
    Inactive = 0,
    #[doc = "1: AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    Active = 1,
}
impl From<Dualbuffselect> for bool {
    #[inline(always)]
    fn from(variant: Dualbuffselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DualbuffR = crate::BitReader<Dualbuffselect>;
impl DualbuffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dualbuffselect {
        match self.bits {
            false => Dualbuffselect::Inactive,
            true => Dualbuffselect::Active,
        }
    }
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dualbuffselect::Inactive
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dualbuffselect::Active
    }
}
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DualbuffW<'a, REG> = crate::BitWriter<'a, REG, Dualbuffselect>;
impl<'a, REG> DualbuffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Dualbuffselect::Inactive)
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Dualbuffselect::Active)
    }
}
#[doc = "Field `PROCDLY` reader - Processing Delay"]
pub type ProcdlyR = crate::FieldReader;
#[doc = "Field `PROCDLY` writer - Processing Delay"]
pub type ProcdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smodselect {
    #[doc = "0: Manual Mode"]
    ManualStart = 0,
    #[doc = "1: Auto Mode"]
    AutoStart = 1,
    #[doc = "2: AES_IDATAR0 access only Auto Mode (DMA)"]
    Idatar0Start = 2,
}
impl From<Smodselect> for u8 {
    #[inline(always)]
    fn from(variant: Smodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smodselect {
    type Ux = u8;
}
impl crate::IsEnum for Smodselect {}
#[doc = "Field `SMOD` reader - Start Mode"]
pub type SmodR = crate::FieldReader<Smodselect>;
impl SmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smodselect> {
        match self.bits {
            0 => Some(Smodselect::ManualStart),
            1 => Some(Smodselect::AutoStart),
            2 => Some(Smodselect::Idatar0Start),
            _ => None,
        }
    }
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn is_manual_start(&self) -> bool {
        *self == Smodselect::ManualStart
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        *self == Smodselect::AutoStart
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn is_idatar0_start(&self) -> bool {
        *self == Smodselect::Idatar0Start
    }
}
#[doc = "Field `SMOD` writer - Start Mode"]
pub type SmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Smodselect>;
impl<'a, REG> SmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn manual_start(self) -> &'a mut crate::W<REG> {
        self.variant(Smodselect::ManualStart)
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut crate::W<REG> {
        self.variant(Smodselect::AutoStart)
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn idatar0_start(self) -> &'a mut crate::W<REG> {
        self.variant(Smodselect::Idatar0Start)
    }
}
#[doc = "Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keysizeselect {
    #[doc = "0: AES Key Size is 128 bits"]
    Aes128 = 0,
    #[doc = "1: AES Key Size is 192 bits"]
    Aes192 = 1,
    #[doc = "2: AES Key Size is 256 bits"]
    Aes256 = 2,
}
impl From<Keysizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Keysizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keysizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Keysizeselect {}
#[doc = "Field `KEYSIZE` reader - Key Size"]
pub type KeysizeR = crate::FieldReader<Keysizeselect>;
impl KeysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keysizeselect> {
        match self.bits {
            0 => Some(Keysizeselect::Aes128),
            1 => Some(Keysizeselect::Aes192),
            2 => Some(Keysizeselect::Aes256),
            _ => None,
        }
    }
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == Keysizeselect::Aes128
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == Keysizeselect::Aes192
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == Keysizeselect::Aes256
    }
}
#[doc = "Field `KEYSIZE` writer - Key Size"]
pub type KeysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Keysizeselect>;
impl<'a, REG> KeysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(Keysizeselect::Aes128)
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut crate::W<REG> {
        self.variant(Keysizeselect::Aes192)
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(Keysizeselect::Aes256)
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opmodselect {
    #[doc = "0: ECB: Electronic Code Book mode"]
    Ecb = 0,
    #[doc = "1: CBC: Cipher Block Chaining mode"]
    Cbc = 1,
    #[doc = "2: OFB: Output Feedback mode"]
    Ofb = 2,
    #[doc = "3: CFB: Cipher Feedback mode"]
    Cfb = 3,
    #[doc = "4: CTR: Counter mode (16-bit internal counter)"]
    Ctr = 4,
    #[doc = "5: GCM: Galois/Counter mode"]
    Gcm = 5,
}
impl From<Opmodselect> for u8 {
    #[inline(always)]
    fn from(variant: Opmodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opmodselect {
    type Ux = u8;
}
impl crate::IsEnum for Opmodselect {}
#[doc = "Field `OPMOD` reader - Operating Mode"]
pub type OpmodR = crate::FieldReader<Opmodselect>;
impl OpmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opmodselect> {
        match self.bits {
            0 => Some(Opmodselect::Ecb),
            1 => Some(Opmodselect::Cbc),
            2 => Some(Opmodselect::Ofb),
            3 => Some(Opmodselect::Cfb),
            4 => Some(Opmodselect::Ctr),
            5 => Some(Opmodselect::Gcm),
            _ => None,
        }
    }
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Opmodselect::Ecb
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Opmodselect::Cbc
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == Opmodselect::Ofb
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == Opmodselect::Cfb
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == Opmodselect::Ctr
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == Opmodselect::Gcm
    }
}
#[doc = "Field `OPMOD` writer - Operating Mode"]
pub type OpmodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Opmodselect>;
impl<'a, REG> OpmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodselect::Ecb)
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodselect::Cbc)
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodselect::Ofb)
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodselect::Cfb)
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodselect::Ctr)
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodselect::Gcm)
    }
}
#[doc = "Field `LOD` reader - Last Output Data Mode"]
pub type LodR = crate::BitReader;
#[doc = "Field `LOD` writer - Last Output Data Mode"]
pub type LodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Cipher Feedback Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfbsselect {
    #[doc = "0: 128-bit"]
    Size128bit = 0,
    #[doc = "1: 64-bit"]
    Size64bit = 1,
    #[doc = "2: 32-bit"]
    Size32bit = 2,
    #[doc = "3: 16-bit"]
    Size16bit = 3,
    #[doc = "4: 8-bit"]
    Size8bit = 4,
}
impl From<Cfbsselect> for u8 {
    #[inline(always)]
    fn from(variant: Cfbsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfbsselect {
    type Ux = u8;
}
impl crate::IsEnum for Cfbsselect {}
#[doc = "Field `CFBS` reader - Cipher Feedback Data Size"]
pub type CfbsR = crate::FieldReader<Cfbsselect>;
impl CfbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfbsselect> {
        match self.bits {
            0 => Some(Cfbsselect::Size128bit),
            1 => Some(Cfbsselect::Size64bit),
            2 => Some(Cfbsselect::Size32bit),
            3 => Some(Cfbsselect::Size16bit),
            4 => Some(Cfbsselect::Size8bit),
            _ => None,
        }
    }
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn is_size_128bit(&self) -> bool {
        *self == Cfbsselect::Size128bit
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn is_size_64bit(&self) -> bool {
        *self == Cfbsselect::Size64bit
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_size_32bit(&self) -> bool {
        *self == Cfbsselect::Size32bit
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_size_16bit(&self) -> bool {
        *self == Cfbsselect::Size16bit
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_size_8bit(&self) -> bool {
        *self == Cfbsselect::Size8bit
    }
}
#[doc = "Field `CFBS` writer - Cipher Feedback Data Size"]
pub type CfbsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfbsselect>;
impl<'a, REG> CfbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn size_128bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::Size128bit)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn size_64bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::Size64bit)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn size_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::Size32bit)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn size_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::Size16bit)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn size_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::Size8bit)
    }
}
#[doc = "Countermeasure Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckeyselect {
    #[doc = "14: This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    Passwd = 14,
}
impl From<Ckeyselect> for u8 {
    #[inline(always)]
    fn from(variant: Ckeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckeyselect {
    type Ux = u8;
}
impl crate::IsEnum for Ckeyselect {}
#[doc = "Field `CKEY` reader - Countermeasure Key"]
pub type CkeyR = crate::FieldReader<Ckeyselect>;
impl CkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckeyselect> {
        match self.bits {
            14 => Some(Ckeyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Ckeyselect::Passwd
    }
}
#[doc = "Field `CKEY` writer - Countermeasure Key"]
pub type CkeyW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ckeyselect>;
impl<'a, REG> CkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Ckeyselect::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CipherR {
        CipherR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&self) -> GtagenR {
        GtagenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DualbuffR {
        DualbuffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&self) -> ProcdlyR {
        ProcdlyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SmodR {
        SmodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&self) -> OpmodR {
        OpmodR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LodR {
        LodR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CfbsR {
        CfbsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&self) -> CkeyR {
        CkeyR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&mut self) -> CipherW<MrSpec> {
        CipherW::new(self, 0)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&mut self) -> GtagenW<MrSpec> {
        GtagenW::new(self, 1)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> DualbuffW<MrSpec> {
        DualbuffW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&mut self) -> ProcdlyW<MrSpec> {
        ProcdlyW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> SmodW<MrSpec> {
        SmodW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KeysizeW<MrSpec> {
        KeysizeW::new(self, 10)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&mut self) -> OpmodW<MrSpec> {
        OpmodW::new(self, 12)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&mut self) -> LodW<MrSpec> {
        LodW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&mut self) -> CfbsW<MrSpec> {
        CfbsW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&mut self) -> CkeyW<MrSpec> {
        CkeyW::new(self, 20)
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
