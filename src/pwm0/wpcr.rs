#[doc = "Register `WPCR` writer"]
pub type W = crate::W<WpcrSpec>;
#[doc = "Write Protection Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wpcmdselect {
    #[doc = "0: Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    DisableSwProt = 0,
    #[doc = "1: Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    EnableSwProt = 1,
    #[doc = "2: Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    EnableHwProt = 2,
}
impl From<Wpcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Wpcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Wpcmdselect {}
#[doc = "Field `WPCMD` writer - Write Protection Command"]
pub type WpcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wpcmdselect>;
impl<'a, REG> WpcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn disable_sw_prot(self) -> &'a mut crate::W<REG> {
        self.variant(Wpcmdselect::DisableSwProt)
    }
    #[doc = "Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn enable_sw_prot(self) -> &'a mut crate::W<REG> {
        self.variant(Wpcmdselect::EnableSwProt)
    }
    #[doc = "Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    #[inline(always)]
    pub fn enable_hw_prot(self) -> &'a mut crate::W<REG> {
        self.variant(Wpcmdselect::EnableHwProt)
    }
}
#[doc = "Field `WPRG0` writer - Write Protection Register Group 0"]
pub type Wprg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG1` writer - Write Protection Register Group 1"]
pub type Wprg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG2` writer - Write Protection Register Group 2"]
pub type Wprg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG3` writer - Write Protection Register Group 3"]
pub type Wprg3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG4` writer - Write Protection Register Group 4"]
pub type Wprg4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG5` writer - Write Protection Register Group 5"]
pub type Wprg5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wpkeyselect {
    #[doc = "5265229: Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    Passwd = 5265229,
}
impl From<Wpkeyselect> for u32 {
    #[inline(always)]
    fn from(variant: Wpkeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpkeyselect {
    type Ux = u32;
}
impl crate::IsEnum for Wpkeyselect {}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wpkeyselect>;
impl<'a, REG> WpkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Wpkeyselect::Passwd)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Protection Command"]
    #[inline(always)]
    pub fn wpcmd(&mut self) -> WpcmdW<WpcrSpec> {
        WpcmdW::new(self, 0)
    }
    #[doc = "Bit 2 - Write Protection Register Group 0"]
    #[inline(always)]
    pub fn wprg0(&mut self) -> Wprg0W<WpcrSpec> {
        Wprg0W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protection Register Group 1"]
    #[inline(always)]
    pub fn wprg1(&mut self) -> Wprg1W<WpcrSpec> {
        Wprg1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protection Register Group 2"]
    #[inline(always)]
    pub fn wprg2(&mut self) -> Wprg2W<WpcrSpec> {
        Wprg2W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection Register Group 3"]
    #[inline(always)]
    pub fn wprg3(&mut self) -> Wprg3W<WpcrSpec> {
        Wprg3W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protection Register Group 4"]
    #[inline(always)]
    pub fn wprg4(&mut self) -> Wprg4W<WpcrSpec> {
        Wprg4W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protection Register Group 5"]
    #[inline(always)]
    pub fn wprg5(&mut self) -> Wprg5W<WpcrSpec> {
        Wprg5W::new(self, 7)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WpkeyW<WpcrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "PWM Write Protection Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpcrSpec;
impl crate::RegisterSpec for WpcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpcr::W`](W) writer structure"]
impl crate::Writable for WpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPCR to value 0"]
impl crate::Resettable for WpcrSpec {}
