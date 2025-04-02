#[doc = "Register `EEFC_FCR` writer"]
pub type W = crate::W<EefcFcrSpec>;
#[doc = "Flash Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcmdselect {
    #[doc = "0: Get Flash descriptor"]
    Getd = 0,
    #[doc = "1: Write page"]
    Wp = 1,
    #[doc = "2: Write page and lock"]
    Wpl = 2,
    #[doc = "3: Erase page and write page"]
    Ewp = 3,
    #[doc = "4: Erase page and write page then lock"]
    Ewpl = 4,
    #[doc = "5: Erase all"]
    Ea = 5,
    #[doc = "7: Erase pages"]
    Epa = 7,
    #[doc = "8: Set lock bit"]
    Slb = 8,
    #[doc = "9: Clear lock bit"]
    Clb = 9,
    #[doc = "10: Get lock bit"]
    Glb = 10,
    #[doc = "11: Set GPNVM bit"]
    Sgpb = 11,
    #[doc = "12: Clear GPNVM bit"]
    Cgpb = 12,
    #[doc = "13: Get GPNVM bit"]
    Ggpb = 13,
    #[doc = "14: Start read unique identifier"]
    Stui = 14,
    #[doc = "15: Stop read unique identifier"]
    Spui = 15,
    #[doc = "16: Get CALIB bit"]
    Gcalb = 16,
    #[doc = "17: Erase sector"]
    Es = 17,
    #[doc = "18: Write user signature"]
    Wus = 18,
    #[doc = "19: Erase user signature"]
    Eus = 19,
    #[doc = "20: Start read user signature"]
    Stus = 20,
    #[doc = "21: Stop read user signature"]
    Spus = 21,
}
impl From<Fcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Fcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Fcmdselect {}
#[doc = "Field `FCMD` writer - Flash Command"]
pub type FcmdW<'a, REG> = crate::FieldWriter<'a, REG, 8, Fcmdselect>;
impl<'a, REG> FcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Get Flash descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Getd)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Wp)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Wpl)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Ewp)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Ewpl)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Ea)
    }
    #[doc = "Erase pages"]
    #[inline(always)]
    pub fn epa(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Epa)
    }
    #[doc = "Set lock bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Slb)
    }
    #[doc = "Clear lock bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Clb)
    }
    #[doc = "Get lock bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Glb)
    }
    #[doc = "Set GPNVM bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Sgpb)
    }
    #[doc = "Clear GPNVM bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Cgpb)
    }
    #[doc = "Get GPNVM bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Ggpb)
    }
    #[doc = "Start read unique identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Stui)
    }
    #[doc = "Stop read unique identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Spui)
    }
    #[doc = "Get CALIB bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Gcalb)
    }
    #[doc = "Erase sector"]
    #[inline(always)]
    pub fn es(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Es)
    }
    #[doc = "Write user signature"]
    #[inline(always)]
    pub fn wus(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Wus)
    }
    #[doc = "Erase user signature"]
    #[inline(always)]
    pub fn eus(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Eus)
    }
    #[doc = "Start read user signature"]
    #[inline(always)]
    pub fn stus(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Stus)
    }
    #[doc = "Stop read user signature"]
    #[inline(always)]
    pub fn spus(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmdselect::Spus)
    }
}
#[doc = "Field `FARG` writer - Flash Command Argument"]
pub type FargW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Flash Writing Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fkeyselect {
    #[doc = "90: The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    Passwd = 90,
}
impl From<Fkeyselect> for u8 {
    #[inline(always)]
    fn from(variant: Fkeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fkeyselect {
    type Ux = u8;
}
impl crate::IsEnum for Fkeyselect {}
#[doc = "Field `FKEY` writer - Flash Writing Protection Key"]
pub type FkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Fkeyselect>;
impl<'a, REG> FkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Fkeyselect::Passwd)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    pub fn fcmd(&mut self) -> FcmdW<EefcFcrSpec> {
        FcmdW::new(self, 0)
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    pub fn farg(&mut self) -> FargW<EefcFcrSpec> {
        FargW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    pub fn fkey(&mut self) -> FkeyW<EefcFcrSpec> {
        FkeyW::new(self, 24)
    }
}
#[doc = "EEFC Flash Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefc_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EefcFcrSpec;
impl crate::RegisterSpec for EefcFcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eefc_fcr::W`](W) writer structure"]
impl crate::Writable for EefcFcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEFC_FCR to value 0"]
impl crate::Resettable for EefcFcrSpec {}
