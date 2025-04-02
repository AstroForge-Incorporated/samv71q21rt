#[doc = "Register `MDWE[%s]` reader"]
pub type R = crate::R<MdweSpec>;
#[doc = "Register `MDWE[%s]` writer"]
pub type W = crate::W<MdweSpec>;
#[doc = "Field `MASK` reader - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<MdweSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "MIF Data Write Enable 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdwe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdwe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdweSpec;
impl crate::RegisterSpec for MdweSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdwe::R`](R) reader structure"]
impl crate::Readable for MdweSpec {}
#[doc = "`write(|w| ..)` method takes [`mdwe::W`](W) writer structure"]
impl crate::Writable for MdweSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDWE[%s] to value 0"]
impl crate::Resettable for MdweSpec {}
