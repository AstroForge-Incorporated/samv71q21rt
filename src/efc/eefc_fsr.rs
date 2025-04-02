#[doc = "Register `EEFC_FSR` reader"]
pub type R = crate::R<EefcFsrSpec>;
#[doc = "Field `FRDY` reader - Flash Ready Status (cleared when Flash is busy)"]
pub type FrdyR = crate::BitReader;
#[doc = "Field `FCMDE` reader - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
pub type FcmdeR = crate::BitReader;
#[doc = "Field `FLOCKE` reader - Flash Lock Error Status (cleared on read)"]
pub type FlockeR = crate::BitReader;
#[doc = "Field `FLERR` reader - Flash Error Status (cleared when a programming operation starts)"]
pub type FlerrR = crate::BitReader;
#[doc = "Field `UECCELSB` reader - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type UeccelsbR = crate::BitReader;
#[doc = "Field `MECCELSB` reader - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type MeccelsbR = crate::BitReader;
#[doc = "Field `UECCEMSB` reader - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type UeccemsbR = crate::BitReader;
#[doc = "Field `MECCEMSB` reader - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type MeccemsbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flash Ready Status (cleared when Flash is busy)"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
    #[inline(always)]
    pub fn fcmde(&self) -> FcmdeR {
        FcmdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status (cleared on read)"]
    #[inline(always)]
    pub fn flocke(&self) -> FlockeR {
        FlockeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash Error Status (cleared when a programming operation starts)"]
    #[inline(always)]
    pub fn flerr(&self) -> FlerrR {
        FlerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccelsb(&self) -> UeccelsbR {
        UeccelsbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccelsb(&self) -> MeccelsbR {
        MeccelsbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccemsb(&self) -> UeccemsbR {
        UeccemsbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccemsb(&self) -> MeccemsbR {
        MeccemsbR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "EEFC Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eefc_fsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EefcFsrSpec;
impl crate::RegisterSpec for EefcFsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefc_fsr::R`](R) reader structure"]
impl crate::Readable for EefcFsrSpec {}
#[doc = "`reset()` method sets EEFC_FSR to value 0"]
impl crate::Resettable for EefcFsrSpec {}
