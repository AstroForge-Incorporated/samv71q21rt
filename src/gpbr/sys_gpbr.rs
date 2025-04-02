#[doc = "Register `SYS_GPBR[%s]` reader"]
pub type R = crate::R<SysGpbrSpec>;
#[doc = "Register `SYS_GPBR[%s]` writer"]
pub type W = crate::W<SysGpbrSpec>;
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub type GpbrValueR = crate::FieldReader<u32>;
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub type GpbrValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&self) -> GpbrValueR {
        GpbrValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&mut self) -> GpbrValueW<SysGpbrSpec> {
        GpbrValueW::new(self, 0)
    }
}
#[doc = "General Purpose Backup Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_gpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysGpbrSpec;
impl crate::RegisterSpec for SysGpbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_gpbr::R`](R) reader structure"]
impl crate::Readable for SysGpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_gpbr::W`](W) writer structure"]
impl crate::Writable for SysGpbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_GPBR[%s] to value 0"]
impl crate::Resettable for SysGpbrSpec {}
