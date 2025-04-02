#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "WKUP Wake-up Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupsselect {
    #[doc = "0: No wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Wkupsselect> for bool {
    #[inline(always)]
    fn from(variant: Wkupsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPS` reader - WKUP Wake-up Status (cleared on read)"]
pub type WkupsR = crate::BitReader<Wkupsselect>;
impl WkupsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupsselect {
        match self.bits {
            false => Wkupsselect::No,
            true => Wkupsselect::Present,
        }
    }
    #[doc = "No wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Wkupsselect::No
    }
    #[doc = "At least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Wkupsselect::Present
    }
}
#[doc = "Supply Monitor Detection Wake-up Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smwsselect {
    #[doc = "0: No wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: At least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Smwsselect> for bool {
    #[inline(always)]
    fn from(variant: Smwsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMWS` reader - Supply Monitor Detection Wake-up Status (cleared on read)"]
pub type SmwsR = crate::BitReader<Smwsselect>;
impl SmwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smwsselect {
        match self.bits {
            false => Smwsselect::No,
            true => Smwsselect::Present,
        }
    }
    #[doc = "No wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Smwsselect::No
    }
    #[doc = "At least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Smwsselect::Present
    }
}
#[doc = "Brownout Detector Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodrstsselect {
    #[doc = "0: No core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    No = 0,
    #[doc = "1: At least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    Present = 1,
}
impl From<Bodrstsselect> for bool {
    #[inline(always)]
    fn from(variant: Bodrstsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTS` reader - Brownout Detector Reset Status (cleared on read)"]
pub type BodrstsR = crate::BitReader<Bodrstsselect>;
impl BodrstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrstsselect {
        match self.bits {
            false => Bodrstsselect::No,
            true => Bodrstsselect::Present,
        }
    }
    #[doc = "No core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Bodrstsselect::No
    }
    #[doc = "At least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Bodrstsselect::Present
    }
}
#[doc = "Supply Monitor Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smrstsselect {
    #[doc = "0: No supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    No = 0,
    #[doc = "1: At least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    Present = 1,
}
impl From<Smrstsselect> for bool {
    #[inline(always)]
    fn from(variant: Smrstsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTS` reader - Supply Monitor Reset Status (cleared on read)"]
pub type SmrstsR = crate::BitReader<Smrstsselect>;
impl SmrstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smrstsselect {
        match self.bits {
            false => Smrstsselect::No,
            true => Smrstsselect::Present,
        }
    }
    #[doc = "No supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Smrstsselect::No
    }
    #[doc = "At least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Smrstsselect::Present
    }
}
#[doc = "Supply Monitor Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smsselect {
    #[doc = "0: No supply monitor detection since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: At least one supply monitor detection since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Smsselect> for bool {
    #[inline(always)]
    fn from(variant: Smsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS` reader - Supply Monitor Status (cleared on read)"]
pub type SmsR = crate::BitReader<Smsselect>;
impl SmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smsselect {
        match self.bits {
            false => Smsselect::No,
            true => Smsselect::Present,
        }
    }
    #[doc = "No supply monitor detection since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Smsselect::No
    }
    #[doc = "At least one supply monitor detection since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Smsselect::Present
    }
}
#[doc = "Supply Monitor Output Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smosselect {
    #[doc = "0: The supply monitor detected VDDIO higher than its threshold at its last measurement."]
    High = 0,
    #[doc = "1: The supply monitor detected VDDIO lower than its threshold at its last measurement."]
    Low = 1,
}
impl From<Smosselect> for bool {
    #[inline(always)]
    fn from(variant: Smosselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMOS` reader - Supply Monitor Output Status"]
pub type SmosR = crate::BitReader<Smosselect>;
impl SmosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smosselect {
        match self.bits {
            false => Smosselect::High,
            true => Smosselect::Low,
        }
    }
    #[doc = "The supply monitor detected VDDIO higher than its threshold at its last measurement."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Smosselect::High
    }
    #[doc = "The supply monitor detected VDDIO lower than its threshold at its last measurement."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Smosselect::Low
    }
}
#[doc = "32-kHz Oscillator Selection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscselselect {
    #[doc = "0: The slow clock, SLCK, is generated by the embedded 32 kHz RC oscillator."]
    Rc = 0,
    #[doc = "1: The slow clock, SLCK, is generated by the 32 kHz crystal oscillator."]
    Cryst = 1,
}
impl From<Oscselselect> for bool {
    #[inline(always)]
    fn from(variant: Oscselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSEL` reader - 32-kHz Oscillator Selection Status"]
pub type OscselR = crate::BitReader<Oscselselect>;
impl OscselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscselselect {
        match self.bits {
            false => Oscselselect::Rc,
            true => Oscselselect::Cryst,
        }
    }
    #[doc = "The slow clock, SLCK, is generated by the embedded 32 kHz RC oscillator."]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Oscselselect::Rc
    }
    #[doc = "The slow clock, SLCK, is generated by the 32 kHz crystal oscillator."]
    #[inline(always)]
    pub fn is_cryst(&self) -> bool {
        *self == Oscselselect::Cryst
    }
}
#[doc = "Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcs0select {
    #[doc = "0: No wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Lpdbcs0select> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcs0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCS0` reader - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
pub type Lpdbcs0R = crate::BitReader<Lpdbcs0select>;
impl Lpdbcs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcs0select {
        match self.bits {
            false => Lpdbcs0select::No,
            true => Lpdbcs0select::Present,
        }
    }
    #[doc = "No wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Lpdbcs0select::No
    }
    #[doc = "At least one wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Lpdbcs0select::Present
    }
}
#[doc = "Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpdbcs1select {
    #[doc = "0: No wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    No = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    Present = 1,
}
impl From<Lpdbcs1select> for bool {
    #[inline(always)]
    fn from(variant: Lpdbcs1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCS1` reader - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
pub type Lpdbcs1R = crate::BitReader<Lpdbcs1select>;
impl Lpdbcs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpdbcs1select {
        match self.bits {
            false => Lpdbcs1select::No,
            true => Lpdbcs1select::Present,
        }
    }
    #[doc = "No wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Lpdbcs1select::No
    }
    #[doc = "At least one wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Lpdbcs1select::Present
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis0select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis0select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS0` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis0R = crate::BitReader<Wkupis0select>;
impl Wkupis0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis0select {
        match self.bits {
            false => Wkupis0select::Dis,
            true => Wkupis0select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis0select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis0select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis1select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis1select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS1` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis1R = crate::BitReader<Wkupis1select>;
impl Wkupis1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis1select {
        match self.bits {
            false => Wkupis1select::Dis,
            true => Wkupis1select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis1select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis1select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis2select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis2select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS2` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis2R = crate::BitReader<Wkupis2select>;
impl Wkupis2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis2select {
        match self.bits {
            false => Wkupis2select::Dis,
            true => Wkupis2select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis2select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis2select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis3select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis3select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS3` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis3R = crate::BitReader<Wkupis3select>;
impl Wkupis3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis3select {
        match self.bits {
            false => Wkupis3select::Dis,
            true => Wkupis3select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis3select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis3select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis4select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis4select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS4` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis4R = crate::BitReader<Wkupis4select>;
impl Wkupis4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis4select {
        match self.bits {
            false => Wkupis4select::Dis,
            true => Wkupis4select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis4select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis4select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis5select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis5select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS5` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis5R = crate::BitReader<Wkupis5select>;
impl Wkupis5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis5select {
        match self.bits {
            false => Wkupis5select::Dis,
            true => Wkupis5select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis5select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis5select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis6select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis6select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS6` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis6R = crate::BitReader<Wkupis6select>;
impl Wkupis6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis6select {
        match self.bits {
            false => Wkupis6select::Dis,
            true => Wkupis6select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis6select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis6select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis7select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis7select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS7` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis7R = crate::BitReader<Wkupis7select>;
impl Wkupis7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis7select {
        match self.bits {
            false => Wkupis7select::Dis,
            true => Wkupis7select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis7select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis7select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis8select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis8select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS8` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis8R = crate::BitReader<Wkupis8select>;
impl Wkupis8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis8select {
        match self.bits {
            false => Wkupis8select::Dis,
            true => Wkupis8select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis8select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis8select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis9select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis9select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS9` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis9R = crate::BitReader<Wkupis9select>;
impl Wkupis9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis9select {
        match self.bits {
            false => Wkupis9select::Dis,
            true => Wkupis9select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis9select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis9select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis10select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis10select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS10` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis10R = crate::BitReader<Wkupis10select>;
impl Wkupis10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis10select {
        match self.bits {
            false => Wkupis10select::Dis,
            true => Wkupis10select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis10select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis10select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis11select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis11select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS11` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis11R = crate::BitReader<Wkupis11select>;
impl Wkupis11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis11select {
        match self.bits {
            false => Wkupis11select::Dis,
            true => Wkupis11select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis11select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis11select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis12select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis12select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS12` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis12R = crate::BitReader<Wkupis12select>;
impl Wkupis12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis12select {
        match self.bits {
            false => Wkupis12select::Dis,
            true => Wkupis12select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis12select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis12select::En
    }
}
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupis13select {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    Dis = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    En = 1,
}
impl From<Wkupis13select> for bool {
    #[inline(always)]
    fn from(variant: Wkupis13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS13` reader - WKUPx Input Status (cleared on read)"]
pub type Wkupis13R = crate::BitReader<Wkupis13select>;
impl Wkupis13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupis13select {
        match self.bits {
            false => Wkupis13select::Dis,
            true => Wkupis13select::En,
        }
    }
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wkupis13select::Dis
    }
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wkupis13select::En
    }
}
impl R {
    #[doc = "Bit 1 - WKUP Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn wkups(&self) -> WkupsR {
        WkupsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn smws(&self) -> SmwsR {
        SmwsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn bodrsts(&self) -> BodrstsR {
        BodrstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn smrsts(&self) -> SmrstsR {
        SmrstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Supply Monitor Status (cleared on read)"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline(always)]
    pub fn smos(&self) -> SmosR {
        SmosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline(always)]
    pub fn oscsel(&self) -> OscselR {
        OscselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs0(&self) -> Lpdbcs0R {
        Lpdbcs0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs1(&self) -> Lpdbcs1R {
        Lpdbcs1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis0(&self) -> Wkupis0R {
        Wkupis0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis1(&self) -> Wkupis1R {
        Wkupis1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis2(&self) -> Wkupis2R {
        Wkupis2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis3(&self) -> Wkupis3R {
        Wkupis3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis4(&self) -> Wkupis4R {
        Wkupis4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis5(&self) -> Wkupis5R {
        Wkupis5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis6(&self) -> Wkupis6R {
        Wkupis6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis7(&self) -> Wkupis7R {
        Wkupis7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis8(&self) -> Wkupis8R {
        Wkupis8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis9(&self) -> Wkupis9R {
        Wkupis9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis10(&self) -> Wkupis10R {
        Wkupis10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis11(&self) -> Wkupis11R {
        Wkupis11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis12(&self) -> Wkupis12R {
        Wkupis12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis13(&self) -> Wkupis13R {
        Wkupis13R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Supply Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
