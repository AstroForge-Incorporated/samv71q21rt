#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ActlrSpec>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ActlrSpec>;
#[doc = "Field `DISFOLD` reader - Disables folding of IT instructions"]
pub type DisfoldR = crate::BitReader;
#[doc = "Field `DISFOLD` writer - Disables folding of IT instructions"]
pub type DisfoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPEXCODIS` reader - Disables FPU exception outputs"]
pub type FpexcodisR = crate::BitReader;
#[doc = "Field `FPEXCODIS` writer - Disables FPU exception outputs"]
pub type FpexcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISRAMODE` reader - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
pub type DisramodeR = crate::BitReader;
#[doc = "Field `DISRAMODE` writer - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
pub type DisramodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISITMATBFLUSH` reader - Disables ITM and DWT ATB flush"]
pub type DisitmatbflushR = crate::BitReader;
#[doc = "Field `DISITMATBFLUSH` writer - Disables ITM and DWT ATB flush"]
pub type DisitmatbflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISBTACREAD` reader - "]
pub type DisbtacreadR = crate::BitReader;
#[doc = "Field `DISBTACREAD` writer - "]
pub type DisbtacreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISBTACALLOC` reader - "]
pub type DisbtacallocR = crate::BitReader;
#[doc = "Field `DISBTACALLOC` writer - "]
pub type DisbtacallocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCRITAXIRUR` reader - "]
pub type DiscritaxirurR = crate::BitReader;
#[doc = "Field `DISCRITAXIRUR` writer - "]
pub type DiscritaxirurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISDI` reader - "]
pub type DisdiR = crate::FieldReader;
#[doc = "Field `DISDI` writer - "]
pub type DisdiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DISISSCH1` reader - "]
pub type Disissch1R = crate::FieldReader;
#[doc = "Field `DISISSCH1` writer - "]
pub type Disissch1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DISDYNADD` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub type DisdynaddR = crate::BitReader;
#[doc = "Field `DISDYNADD` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub type DisdynaddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCRITAXIRUW` reader - Disable critical AXI read-under-write"]
pub type DiscritaxiruwR = crate::BitReader;
#[doc = "Field `DISCRITAXIRUW` writer - Disable critical AXI read-under-write"]
pub type DiscritaxiruwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFPUISSOPT` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub type DisfpuissoptR = crate::BitReader;
#[doc = "Field `DISFPUISSOPT` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub type DisfpuissoptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Disables folding of IT instructions"]
    #[inline(always)]
    pub fn disfold(&self) -> DisfoldR {
        DisfoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FpexcodisR {
        FpexcodisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
    #[inline(always)]
    pub fn disramode(&self) -> DisramodeR {
        DisramodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DisitmatbflushR {
        DisitmatbflushR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn disbtacread(&self) -> DisbtacreadR {
        DisbtacreadR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn disbtacalloc(&self) -> DisbtacallocR {
        DisbtacallocR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn discritaxirur(&self) -> DiscritaxirurR {
        DiscritaxirurR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn disdi(&self) -> DisdiR {
        DisdiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn disissch1(&self) -> Disissch1R {
        Disissch1R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&self) -> DisdynaddR {
        DisdynaddR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&self) -> DiscritaxiruwR {
        DiscritaxiruwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disfpuissopt(&self) -> DisfpuissoptR {
        DisfpuissoptR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Disables folding of IT instructions"]
    #[inline(always)]
    pub fn disfold(&mut self) -> DisfoldW<ActlrSpec> {
        DisfoldW::new(self, 2)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&mut self) -> FpexcodisW<ActlrSpec> {
        FpexcodisW::new(self, 10)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
    #[inline(always)]
    pub fn disramode(&mut self) -> DisramodeW<ActlrSpec> {
        DisramodeW::new(self, 11)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush"]
    #[inline(always)]
    pub fn disitmatbflush(&mut self) -> DisitmatbflushW<ActlrSpec> {
        DisitmatbflushW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn disbtacread(&mut self) -> DisbtacreadW<ActlrSpec> {
        DisbtacreadW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn disbtacalloc(&mut self) -> DisbtacallocW<ActlrSpec> {
        DisbtacallocW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn discritaxirur(&mut self) -> DiscritaxirurW<ActlrSpec> {
        DiscritaxirurW::new(self, 15)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn disdi(&mut self) -> DisdiW<ActlrSpec> {
        DisdiW::new(self, 16)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn disissch1(&mut self) -> Disissch1W<ActlrSpec> {
        Disissch1W::new(self, 21)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&mut self) -> DisdynaddW<ActlrSpec> {
        DisdynaddW::new(self, 26)
    }
    #[doc = "Bit 27 - Disable critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&mut self) -> DiscritaxiruwW<ActlrSpec> {
        DiscritaxiruwW::new(self, 27)
    }
    #[doc = "Bit 28 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disfpuissopt(&mut self) -> DisfpuissoptW<ActlrSpec> {
        DisfpuissoptW::new(self, 28)
    }
}
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActlrSpec;
impl crate::RegisterSpec for ActlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ActlrSpec {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ActlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ActlrSpec {}
