#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CmdrSpec>;
#[doc = "Field `CMDNB` writer - Command Number"]
pub type CmdnbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsptypselect {
    #[doc = "0: No response"]
    Noresp = 0,
    #[doc = "1: 48-bit response"]
    _48Bit = 1,
    #[doc = "2: 136-bit response"]
    _136Bit = 2,
    #[doc = "3: R1b response type"]
    R1b = 3,
}
impl From<Rsptypselect> for u8 {
    #[inline(always)]
    fn from(variant: Rsptypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsptypselect {
    type Ux = u8;
}
impl crate::IsEnum for Rsptypselect {}
#[doc = "Field `RSPTYP` writer - Response Type"]
pub type RsptypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rsptypselect, crate::Safe>;
impl<'a, REG> RsptypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptypselect::Noresp)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptypselect::_48Bit)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptypselect::_136Bit)
    }
    #[doc = "R1b response type"]
    #[inline(always)]
    pub fn r1b(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptypselect::R1b)
    }
}
#[doc = "Special Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spcmdselect {
    #[doc = "0: Not a special CMD."]
    Std = 0,
    #[doc = "1: Initialization CMD: 74 clock cycles for initialization sequence."]
    Init = 1,
    #[doc = "2: Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    Sync = 2,
    #[doc = "3: CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    CeAta = 3,
    #[doc = "4: Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    ItCmd = 4,
    #[doc = "5: Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    ItResp = 5,
    #[doc = "6: Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    Bor = 6,
    #[doc = "7: End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    Ebo = 7,
}
impl From<Spcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Spcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Spcmdselect {}
#[doc = "Field `SPCMD` writer - Special Command"]
pub type SpcmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Spcmdselect, crate::Safe>;
impl<'a, REG> SpcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not a special CMD."]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::Std)
    }
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::Init)
    }
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::Sync)
    }
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    #[inline(always)]
    pub fn ce_ata(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::CeAta)
    }
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::ItCmd)
    }
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_resp(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::ItResp)
    }
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    #[inline(always)]
    pub fn bor(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::Bor)
    }
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    #[inline(always)]
    pub fn ebo(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmdselect::Ebo)
    }
}
#[doc = "Open Drain Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opdcmdselect {
    #[doc = "0: Push pull command."]
    Pushpull = 0,
    #[doc = "1: Open drain command."]
    Opendrain = 1,
}
impl From<Opdcmdselect> for bool {
    #[inline(always)]
    fn from(variant: Opdcmdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPDCMD` writer - Open Drain Command"]
pub type OpdcmdW<'a, REG> = crate::BitWriter<'a, REG, Opdcmdselect>;
impl<'a, REG> OpdcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push pull command."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Opdcmdselect::Pushpull)
    }
    #[doc = "Open drain command."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut crate::W<REG> {
        self.variant(Opdcmdselect::Opendrain)
    }
}
#[doc = "Max Latency for Command to Response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxlatselect {
    #[doc = "0: 5-cycle max latency."]
    _5 = 0,
    #[doc = "1: 64-cycle max latency."]
    _64 = 1,
}
impl From<Maxlatselect> for bool {
    #[inline(always)]
    fn from(variant: Maxlatselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXLAT` writer - Max Latency for Command to Response"]
pub type MaxlatW<'a, REG> = crate::BitWriter<'a, REG, Maxlatselect>;
impl<'a, REG> MaxlatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "5-cycle max latency."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Maxlatselect::_5)
    }
    #[doc = "64-cycle max latency."]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Maxlatselect::_64)
    }
}
#[doc = "Transfer Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trcmdselect {
    #[doc = "0: No data transfer"]
    NoData = 0,
    #[doc = "1: Start data transfer"]
    StartData = 1,
    #[doc = "2: Stop data transfer"]
    StopData = 2,
}
impl From<Trcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Trcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Trcmdselect {}
#[doc = "Field `TRCMD` writer - Transfer Command"]
pub type TrcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trcmdselect>;
impl<'a, REG> TrcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Trcmdselect::NoData)
    }
    #[doc = "Start data transfer"]
    #[inline(always)]
    pub fn start_data(self) -> &'a mut crate::W<REG> {
        self.variant(Trcmdselect::StartData)
    }
    #[doc = "Stop data transfer"]
    #[inline(always)]
    pub fn stop_data(self) -> &'a mut crate::W<REG> {
        self.variant(Trcmdselect::StopData)
    }
}
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trdirselect {
    #[doc = "0: Write."]
    Write = 0,
    #[doc = "1: Read."]
    Read = 1,
}
impl From<Trdirselect> for bool {
    #[inline(always)]
    fn from(variant: Trdirselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRDIR` writer - Transfer Direction"]
pub type TrdirW<'a, REG> = crate::BitWriter<'a, REG, Trdirselect>;
impl<'a, REG> TrdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Trdirselect::Write)
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Trdirselect::Read)
    }
}
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trtypselect {
    #[doc = "0: MMC/SD Card Single Block"]
    Single = 0,
    #[doc = "1: MMC/SD Card Multiple Block"]
    Multiple = 1,
    #[doc = "2: MMC Stream"]
    Stream = 2,
    #[doc = "4: SDIO Byte"]
    Byte = 4,
    #[doc = "5: SDIO Block"]
    Block = 5,
}
impl From<Trtypselect> for u8 {
    #[inline(always)]
    fn from(variant: Trtypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trtypselect {
    type Ux = u8;
}
impl crate::IsEnum for Trtypselect {}
#[doc = "Field `TRTYP` writer - Transfer Type"]
pub type TrtypW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trtypselect>;
impl<'a, REG> TrtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MMC/SD Card Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Trtypselect::Single)
    }
    #[doc = "MMC/SD Card Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Trtypselect::Multiple)
    }
    #[doc = "MMC Stream"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(Trtypselect::Stream)
    }
    #[doc = "SDIO Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Trtypselect::Byte)
    }
    #[doc = "SDIO Block"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Trtypselect::Block)
    }
}
#[doc = "SDIO Special Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iospcmdselect {
    #[doc = "0: Not an SDIO Special Command"]
    Std = 0,
    #[doc = "1: SDIO Suspend Command"]
    Suspend = 1,
    #[doc = "2: SDIO Resume Command"]
    Resume = 2,
}
impl From<Iospcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Iospcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iospcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Iospcmdselect {}
#[doc = "Field `IOSPCMD` writer - SDIO Special Command"]
pub type IospcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iospcmdselect>;
impl<'a, REG> IospcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not an SDIO Special Command"]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(Iospcmdselect::Std)
    }
    #[doc = "SDIO Suspend Command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Iospcmdselect::Suspend)
    }
    #[doc = "SDIO Resume Command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Iospcmdselect::Resume)
    }
}
#[doc = "ATA with Command Completion Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atacsselect {
    #[doc = "0: Normal operation mode."]
    Normal = 0,
    #[doc = "1: This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    Completion = 1,
}
impl From<Atacsselect> for bool {
    #[inline(always)]
    fn from(variant: Atacsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATACS` writer - ATA with Command Completion Signal"]
pub type AtacsW<'a, REG> = crate::BitWriter<'a, REG, Atacsselect>;
impl<'a, REG> AtacsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Atacsselect::Normal)
    }
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    #[inline(always)]
    pub fn completion(self) -> &'a mut crate::W<REG> {
        self.variant(Atacsselect::Completion)
    }
}
#[doc = "Field `BOOT_ACK` writer - Boot Operation Acknowledge"]
pub type BootAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:5 - Command Number"]
    #[inline(always)]
    pub fn cmdnb(&mut self) -> CmdnbW<CmdrSpec> {
        CmdnbW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Response Type"]
    #[inline(always)]
    pub fn rsptyp(&mut self) -> RsptypW<CmdrSpec> {
        RsptypW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Special Command"]
    #[inline(always)]
    pub fn spcmd(&mut self) -> SpcmdW<CmdrSpec> {
        SpcmdW::new(self, 8)
    }
    #[doc = "Bit 11 - Open Drain Command"]
    #[inline(always)]
    pub fn opdcmd(&mut self) -> OpdcmdW<CmdrSpec> {
        OpdcmdW::new(self, 11)
    }
    #[doc = "Bit 12 - Max Latency for Command to Response"]
    #[inline(always)]
    pub fn maxlat(&mut self) -> MaxlatW<CmdrSpec> {
        MaxlatW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Transfer Command"]
    #[inline(always)]
    pub fn trcmd(&mut self) -> TrcmdW<CmdrSpec> {
        TrcmdW::new(self, 16)
    }
    #[doc = "Bit 18 - Transfer Direction"]
    #[inline(always)]
    pub fn trdir(&mut self) -> TrdirW<CmdrSpec> {
        TrdirW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Transfer Type"]
    #[inline(always)]
    pub fn trtyp(&mut self) -> TrtypW<CmdrSpec> {
        TrtypW::new(self, 19)
    }
    #[doc = "Bits 24:25 - SDIO Special Command"]
    #[inline(always)]
    pub fn iospcmd(&mut self) -> IospcmdW<CmdrSpec> {
        IospcmdW::new(self, 24)
    }
    #[doc = "Bit 26 - ATA with Command Completion Signal"]
    #[inline(always)]
    pub fn atacs(&mut self) -> AtacsW<CmdrSpec> {
        AtacsW::new(self, 26)
    }
    #[doc = "Bit 27 - Boot Operation Acknowledge"]
    #[inline(always)]
    pub fn boot_ack(&mut self) -> BootAckW<CmdrSpec> {
        BootAckW::new(self, 27)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdrSpec;
impl crate::RegisterSpec for CmdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CmdrSpec {}
