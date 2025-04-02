#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Last Error Code (set to 111 on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lecselect {
    #[doc = "0: No error occurred since LEC has been reset by successful reception or transmission."]
    NoError = 0,
    #[doc = "1: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    StuffError = 1,
    #[doc = "2: A fixed format part of a received frame has the wrong format."]
    FormError = 2,
    #[doc = "3: The message transmitted by the MCAN was not acknowledged by another node."]
    AckError = 3,
    #[doc = "4: During transmission of a message (with the exception of the arbitration field), the device tried to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant."]
    Bit1Error = 4,
    #[doc = "5: During transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device tried to send a dominant level (data or identifier bit logical value '0'), but the monitored bus value was recessive. During Bus_Off recovery, this status is set each time a sequence of 11 recessive bits has been monitored. This enables the processor to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    Bit0Error = 5,
    #[doc = "6: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match the CRC calculated from the received data."]
    CrcError = 6,
    #[doc = "7: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows value '7', no CAN bus event was detected since the last processor read access to the Protocol Status Register."]
    NoChange = 7,
}
impl From<Lecselect> for u8 {
    #[inline(always)]
    fn from(variant: Lecselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lecselect {
    type Ux = u8;
}
impl crate::IsEnum for Lecselect {}
#[doc = "Field `LEC` reader - Last Error Code (set to 111 on read)"]
pub type LecR = crate::FieldReader<Lecselect>;
impl LecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lecselect {
        match self.bits {
            0 => Lecselect::NoError,
            1 => Lecselect::StuffError,
            2 => Lecselect::FormError,
            3 => Lecselect::AckError,
            4 => Lecselect::Bit1Error,
            5 => Lecselect::Bit0Error,
            6 => Lecselect::CrcError,
            7 => Lecselect::NoChange,
            _ => unreachable!(),
        }
    }
    #[doc = "No error occurred since LEC has been reset by successful reception or transmission."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Lecselect::NoError
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        *self == Lecselect::StuffError
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        *self == Lecselect::FormError
    }
    #[doc = "The message transmitted by the MCAN was not acknowledged by another node."]
    #[inline(always)]
    pub fn is_ack_error(&self) -> bool {
        *self == Lecselect::AckError
    }
    #[doc = "During transmission of a message (with the exception of the arbitration field), the device tried to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn is_bit1_error(&self) -> bool {
        *self == Lecselect::Bit1Error
    }
    #[doc = "During transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device tried to send a dominant level (data or identifier bit logical value '0'), but the monitored bus value was recessive. During Bus_Off recovery, this status is set each time a sequence of 11 recessive bits has been monitored. This enables the processor to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn is_bit0_error(&self) -> bool {
        *self == Lecselect::Bit0Error
    }
    #[doc = "The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match the CRC calculated from the received data."]
    #[inline(always)]
    pub fn is_crc_error(&self) -> bool {
        *self == Lecselect::CrcError
    }
    #[doc = "Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows value '7', no CAN bus event was detected since the last processor read access to the Protocol Status Register."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Lecselect::NoChange
    }
}
#[doc = "Activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actselect {
    #[doc = "0: Node is synchronizing on CAN communication"]
    Synchronizing = 0,
    #[doc = "1: Node is neither receiver nor transmitter"]
    Idle = 1,
    #[doc = "2: Node is operating as receiver"]
    Receiver = 2,
    #[doc = "3: Node is operating as transmitter"]
    Transmitter = 3,
}
impl From<Actselect> for u8 {
    #[inline(always)]
    fn from(variant: Actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actselect {
    type Ux = u8;
}
impl crate::IsEnum for Actselect {}
#[doc = "Field `ACT` reader - Activity"]
pub type ActR = crate::FieldReader<Actselect>;
impl ActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actselect {
        match self.bits {
            0 => Actselect::Synchronizing,
            1 => Actselect::Idle,
            2 => Actselect::Receiver,
            3 => Actselect::Transmitter,
            _ => unreachable!(),
        }
    }
    #[doc = "Node is synchronizing on CAN communication"]
    #[inline(always)]
    pub fn is_synchronizing(&self) -> bool {
        *self == Actselect::Synchronizing
    }
    #[doc = "Node is neither receiver nor transmitter"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Actselect::Idle
    }
    #[doc = "Node is operating as receiver"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == Actselect::Receiver
    }
    #[doc = "Node is operating as transmitter"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == Actselect::Transmitter
    }
}
#[doc = "Field `EP` reader - Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EW` reader - Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BoR = crate::BitReader;
#[doc = "Field `DLEC` reader - Data Phase Last Error Code (set to 111 on read)"]
pub type DlecR = crate::FieldReader;
#[doc = "Field `RESI` reader - ESI Flag of Last Received CAN FD Message (cleared on read)"]
pub type ResiR = crate::BitReader;
#[doc = "Field `RBRS` reader - BRS Flag of Last Received CAN FD Message (cleared on read)"]
pub type RbrsR = crate::BitReader;
#[doc = "Field `RFDF` reader - Received a CAN FD Message (cleared on read)"]
pub type RfdfR = crate::BitReader;
#[doc = "Field `PXE` reader - Protocol Exception Event (cleared on read)"]
pub type PxeR = crate::BitReader;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TdcvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code (set to 111 on read)"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Phase Last Error Code (set to 111 on read)"]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI Flag of Last Received CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS Flag of Last Received CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn rfdf(&self) -> RfdfR {
        RfdfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event (cleared on read)"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Protocol Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PsrSpec {}
