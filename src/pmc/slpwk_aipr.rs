#[doc = "Register `SLPWK_AIPR` reader"]
pub type R = crate::R<SlpwkAiprSpec>;
#[doc = "Field `AIP` reader - Activity In Progress"]
pub type AipR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Activity In Progress"]
    #[inline(always)]
    pub fn aip(&self) -> AipR {
        AipR::new((self.bits & 1) != 0)
    }
}
#[doc = "SleepWalking Activity In Progress Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slpwk_aipr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpwkAiprSpec;
impl crate::RegisterSpec for SlpwkAiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slpwk_aipr::R`](R) reader structure"]
impl crate::Readable for SlpwkAiprSpec {}
#[doc = "`reset()` method sets SLPWK_AIPR to value 0"]
impl crate::Resettable for SlpwkAiprSpec {}
