#[doc = "Register `DRIVER` reader"]
pub type R = crate::R<DriverSpec>;
#[doc = "Register `DRIVER` writer"]
pub type W = crate::W<DriverSpec>;
#[doc = "Drive of PIO Line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line0select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line0select> for bool {
    #[inline(always)]
    fn from(variant: Line0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE0` reader - Drive of PIO Line 0"]
pub type Line0R = crate::BitReader<Line0select>;
impl Line0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line0select {
        match self.bits {
            false => Line0select::LowDrive,
            true => Line0select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line0select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line0select::HighDrive
    }
}
#[doc = "Field `LINE0` writer - Drive of PIO Line 0"]
pub type Line0W<'a, REG> = crate::BitWriter<'a, REG, Line0select>;
impl<'a, REG> Line0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line0select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line0select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line1select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line1select> for bool {
    #[inline(always)]
    fn from(variant: Line1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE1` reader - Drive of PIO Line 1"]
pub type Line1R = crate::BitReader<Line1select>;
impl Line1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line1select {
        match self.bits {
            false => Line1select::LowDrive,
            true => Line1select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line1select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line1select::HighDrive
    }
}
#[doc = "Field `LINE1` writer - Drive of PIO Line 1"]
pub type Line1W<'a, REG> = crate::BitWriter<'a, REG, Line1select>;
impl<'a, REG> Line1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line1select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line1select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line2select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line2select> for bool {
    #[inline(always)]
    fn from(variant: Line2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE2` reader - Drive of PIO Line 2"]
pub type Line2R = crate::BitReader<Line2select>;
impl Line2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line2select {
        match self.bits {
            false => Line2select::LowDrive,
            true => Line2select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line2select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line2select::HighDrive
    }
}
#[doc = "Field `LINE2` writer - Drive of PIO Line 2"]
pub type Line2W<'a, REG> = crate::BitWriter<'a, REG, Line2select>;
impl<'a, REG> Line2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line2select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line2select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line3select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line3select> for bool {
    #[inline(always)]
    fn from(variant: Line3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE3` reader - Drive of PIO Line 3"]
pub type Line3R = crate::BitReader<Line3select>;
impl Line3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line3select {
        match self.bits {
            false => Line3select::LowDrive,
            true => Line3select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line3select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line3select::HighDrive
    }
}
#[doc = "Field `LINE3` writer - Drive of PIO Line 3"]
pub type Line3W<'a, REG> = crate::BitWriter<'a, REG, Line3select>;
impl<'a, REG> Line3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line3select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line3select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line4select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line4select> for bool {
    #[inline(always)]
    fn from(variant: Line4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE4` reader - Drive of PIO Line 4"]
pub type Line4R = crate::BitReader<Line4select>;
impl Line4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line4select {
        match self.bits {
            false => Line4select::LowDrive,
            true => Line4select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line4select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line4select::HighDrive
    }
}
#[doc = "Field `LINE4` writer - Drive of PIO Line 4"]
pub type Line4W<'a, REG> = crate::BitWriter<'a, REG, Line4select>;
impl<'a, REG> Line4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line4select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line4select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line5select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line5select> for bool {
    #[inline(always)]
    fn from(variant: Line5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE5` reader - Drive of PIO Line 5"]
pub type Line5R = crate::BitReader<Line5select>;
impl Line5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line5select {
        match self.bits {
            false => Line5select::LowDrive,
            true => Line5select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line5select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line5select::HighDrive
    }
}
#[doc = "Field `LINE5` writer - Drive of PIO Line 5"]
pub type Line5W<'a, REG> = crate::BitWriter<'a, REG, Line5select>;
impl<'a, REG> Line5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line5select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line5select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line6select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line6select> for bool {
    #[inline(always)]
    fn from(variant: Line6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE6` reader - Drive of PIO Line 6"]
pub type Line6R = crate::BitReader<Line6select>;
impl Line6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line6select {
        match self.bits {
            false => Line6select::LowDrive,
            true => Line6select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line6select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line6select::HighDrive
    }
}
#[doc = "Field `LINE6` writer - Drive of PIO Line 6"]
pub type Line6W<'a, REG> = crate::BitWriter<'a, REG, Line6select>;
impl<'a, REG> Line6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line6select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line6select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line7select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line7select> for bool {
    #[inline(always)]
    fn from(variant: Line7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE7` reader - Drive of PIO Line 7"]
pub type Line7R = crate::BitReader<Line7select>;
impl Line7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line7select {
        match self.bits {
            false => Line7select::LowDrive,
            true => Line7select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line7select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line7select::HighDrive
    }
}
#[doc = "Field `LINE7` writer - Drive of PIO Line 7"]
pub type Line7W<'a, REG> = crate::BitWriter<'a, REG, Line7select>;
impl<'a, REG> Line7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line7select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line7select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line8select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line8select> for bool {
    #[inline(always)]
    fn from(variant: Line8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE8` reader - Drive of PIO Line 8"]
pub type Line8R = crate::BitReader<Line8select>;
impl Line8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line8select {
        match self.bits {
            false => Line8select::LowDrive,
            true => Line8select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line8select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line8select::HighDrive
    }
}
#[doc = "Field `LINE8` writer - Drive of PIO Line 8"]
pub type Line8W<'a, REG> = crate::BitWriter<'a, REG, Line8select>;
impl<'a, REG> Line8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line8select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line8select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line9select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line9select> for bool {
    #[inline(always)]
    fn from(variant: Line9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE9` reader - Drive of PIO Line 9"]
pub type Line9R = crate::BitReader<Line9select>;
impl Line9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line9select {
        match self.bits {
            false => Line9select::LowDrive,
            true => Line9select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line9select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line9select::HighDrive
    }
}
#[doc = "Field `LINE9` writer - Drive of PIO Line 9"]
pub type Line9W<'a, REG> = crate::BitWriter<'a, REG, Line9select>;
impl<'a, REG> Line9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line9select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line9select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line10select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line10select> for bool {
    #[inline(always)]
    fn from(variant: Line10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE10` reader - Drive of PIO Line 10"]
pub type Line10R = crate::BitReader<Line10select>;
impl Line10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line10select {
        match self.bits {
            false => Line10select::LowDrive,
            true => Line10select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line10select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line10select::HighDrive
    }
}
#[doc = "Field `LINE10` writer - Drive of PIO Line 10"]
pub type Line10W<'a, REG> = crate::BitWriter<'a, REG, Line10select>;
impl<'a, REG> Line10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line10select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line10select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line11select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line11select> for bool {
    #[inline(always)]
    fn from(variant: Line11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE11` reader - Drive of PIO Line 11"]
pub type Line11R = crate::BitReader<Line11select>;
impl Line11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line11select {
        match self.bits {
            false => Line11select::LowDrive,
            true => Line11select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line11select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line11select::HighDrive
    }
}
#[doc = "Field `LINE11` writer - Drive of PIO Line 11"]
pub type Line11W<'a, REG> = crate::BitWriter<'a, REG, Line11select>;
impl<'a, REG> Line11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line11select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line11select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line12select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line12select> for bool {
    #[inline(always)]
    fn from(variant: Line12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE12` reader - Drive of PIO Line 12"]
pub type Line12R = crate::BitReader<Line12select>;
impl Line12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line12select {
        match self.bits {
            false => Line12select::LowDrive,
            true => Line12select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line12select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line12select::HighDrive
    }
}
#[doc = "Field `LINE12` writer - Drive of PIO Line 12"]
pub type Line12W<'a, REG> = crate::BitWriter<'a, REG, Line12select>;
impl<'a, REG> Line12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line12select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line12select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line13select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line13select> for bool {
    #[inline(always)]
    fn from(variant: Line13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE13` reader - Drive of PIO Line 13"]
pub type Line13R = crate::BitReader<Line13select>;
impl Line13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line13select {
        match self.bits {
            false => Line13select::LowDrive,
            true => Line13select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line13select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line13select::HighDrive
    }
}
#[doc = "Field `LINE13` writer - Drive of PIO Line 13"]
pub type Line13W<'a, REG> = crate::BitWriter<'a, REG, Line13select>;
impl<'a, REG> Line13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line13select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line13select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line14select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line14select> for bool {
    #[inline(always)]
    fn from(variant: Line14select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE14` reader - Drive of PIO Line 14"]
pub type Line14R = crate::BitReader<Line14select>;
impl Line14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line14select {
        match self.bits {
            false => Line14select::LowDrive,
            true => Line14select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line14select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line14select::HighDrive
    }
}
#[doc = "Field `LINE14` writer - Drive of PIO Line 14"]
pub type Line14W<'a, REG> = crate::BitWriter<'a, REG, Line14select>;
impl<'a, REG> Line14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line14select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line14select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line15select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line15select> for bool {
    #[inline(always)]
    fn from(variant: Line15select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE15` reader - Drive of PIO Line 15"]
pub type Line15R = crate::BitReader<Line15select>;
impl Line15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line15select {
        match self.bits {
            false => Line15select::LowDrive,
            true => Line15select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line15select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line15select::HighDrive
    }
}
#[doc = "Field `LINE15` writer - Drive of PIO Line 15"]
pub type Line15W<'a, REG> = crate::BitWriter<'a, REG, Line15select>;
impl<'a, REG> Line15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line15select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line15select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line16select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line16select> for bool {
    #[inline(always)]
    fn from(variant: Line16select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE16` reader - Drive of PIO Line 16"]
pub type Line16R = crate::BitReader<Line16select>;
impl Line16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line16select {
        match self.bits {
            false => Line16select::LowDrive,
            true => Line16select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line16select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line16select::HighDrive
    }
}
#[doc = "Field `LINE16` writer - Drive of PIO Line 16"]
pub type Line16W<'a, REG> = crate::BitWriter<'a, REG, Line16select>;
impl<'a, REG> Line16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line16select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line16select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line17select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line17select> for bool {
    #[inline(always)]
    fn from(variant: Line17select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE17` reader - Drive of PIO Line 17"]
pub type Line17R = crate::BitReader<Line17select>;
impl Line17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line17select {
        match self.bits {
            false => Line17select::LowDrive,
            true => Line17select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line17select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line17select::HighDrive
    }
}
#[doc = "Field `LINE17` writer - Drive of PIO Line 17"]
pub type Line17W<'a, REG> = crate::BitWriter<'a, REG, Line17select>;
impl<'a, REG> Line17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line17select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line17select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line18select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line18select> for bool {
    #[inline(always)]
    fn from(variant: Line18select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE18` reader - Drive of PIO Line 18"]
pub type Line18R = crate::BitReader<Line18select>;
impl Line18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line18select {
        match self.bits {
            false => Line18select::LowDrive,
            true => Line18select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line18select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line18select::HighDrive
    }
}
#[doc = "Field `LINE18` writer - Drive of PIO Line 18"]
pub type Line18W<'a, REG> = crate::BitWriter<'a, REG, Line18select>;
impl<'a, REG> Line18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line18select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line18select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line19select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line19select> for bool {
    #[inline(always)]
    fn from(variant: Line19select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE19` reader - Drive of PIO Line 19"]
pub type Line19R = crate::BitReader<Line19select>;
impl Line19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line19select {
        match self.bits {
            false => Line19select::LowDrive,
            true => Line19select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line19select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line19select::HighDrive
    }
}
#[doc = "Field `LINE19` writer - Drive of PIO Line 19"]
pub type Line19W<'a, REG> = crate::BitWriter<'a, REG, Line19select>;
impl<'a, REG> Line19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line19select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line19select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line20select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line20select> for bool {
    #[inline(always)]
    fn from(variant: Line20select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE20` reader - Drive of PIO Line 20"]
pub type Line20R = crate::BitReader<Line20select>;
impl Line20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line20select {
        match self.bits {
            false => Line20select::LowDrive,
            true => Line20select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line20select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line20select::HighDrive
    }
}
#[doc = "Field `LINE20` writer - Drive of PIO Line 20"]
pub type Line20W<'a, REG> = crate::BitWriter<'a, REG, Line20select>;
impl<'a, REG> Line20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line20select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line20select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line21select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line21select> for bool {
    #[inline(always)]
    fn from(variant: Line21select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE21` reader - Drive of PIO Line 21"]
pub type Line21R = crate::BitReader<Line21select>;
impl Line21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line21select {
        match self.bits {
            false => Line21select::LowDrive,
            true => Line21select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line21select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line21select::HighDrive
    }
}
#[doc = "Field `LINE21` writer - Drive of PIO Line 21"]
pub type Line21W<'a, REG> = crate::BitWriter<'a, REG, Line21select>;
impl<'a, REG> Line21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line21select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line21select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line22select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line22select> for bool {
    #[inline(always)]
    fn from(variant: Line22select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE22` reader - Drive of PIO Line 22"]
pub type Line22R = crate::BitReader<Line22select>;
impl Line22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line22select {
        match self.bits {
            false => Line22select::LowDrive,
            true => Line22select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line22select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line22select::HighDrive
    }
}
#[doc = "Field `LINE22` writer - Drive of PIO Line 22"]
pub type Line22W<'a, REG> = crate::BitWriter<'a, REG, Line22select>;
impl<'a, REG> Line22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line22select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line22select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line23select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line23select> for bool {
    #[inline(always)]
    fn from(variant: Line23select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE23` reader - Drive of PIO Line 23"]
pub type Line23R = crate::BitReader<Line23select>;
impl Line23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line23select {
        match self.bits {
            false => Line23select::LowDrive,
            true => Line23select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line23select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line23select::HighDrive
    }
}
#[doc = "Field `LINE23` writer - Drive of PIO Line 23"]
pub type Line23W<'a, REG> = crate::BitWriter<'a, REG, Line23select>;
impl<'a, REG> Line23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line23select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line23select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line24select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line24select> for bool {
    #[inline(always)]
    fn from(variant: Line24select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE24` reader - Drive of PIO Line 24"]
pub type Line24R = crate::BitReader<Line24select>;
impl Line24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line24select {
        match self.bits {
            false => Line24select::LowDrive,
            true => Line24select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line24select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line24select::HighDrive
    }
}
#[doc = "Field `LINE24` writer - Drive of PIO Line 24"]
pub type Line24W<'a, REG> = crate::BitWriter<'a, REG, Line24select>;
impl<'a, REG> Line24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line24select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line24select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line25select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line25select> for bool {
    #[inline(always)]
    fn from(variant: Line25select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE25` reader - Drive of PIO Line 25"]
pub type Line25R = crate::BitReader<Line25select>;
impl Line25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line25select {
        match self.bits {
            false => Line25select::LowDrive,
            true => Line25select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line25select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line25select::HighDrive
    }
}
#[doc = "Field `LINE25` writer - Drive of PIO Line 25"]
pub type Line25W<'a, REG> = crate::BitWriter<'a, REG, Line25select>;
impl<'a, REG> Line25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line25select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line25select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line26select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line26select> for bool {
    #[inline(always)]
    fn from(variant: Line26select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE26` reader - Drive of PIO Line 26"]
pub type Line26R = crate::BitReader<Line26select>;
impl Line26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line26select {
        match self.bits {
            false => Line26select::LowDrive,
            true => Line26select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line26select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line26select::HighDrive
    }
}
#[doc = "Field `LINE26` writer - Drive of PIO Line 26"]
pub type Line26W<'a, REG> = crate::BitWriter<'a, REG, Line26select>;
impl<'a, REG> Line26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line26select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line26select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line27select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line27select> for bool {
    #[inline(always)]
    fn from(variant: Line27select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE27` reader - Drive of PIO Line 27"]
pub type Line27R = crate::BitReader<Line27select>;
impl Line27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line27select {
        match self.bits {
            false => Line27select::LowDrive,
            true => Line27select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line27select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line27select::HighDrive
    }
}
#[doc = "Field `LINE27` writer - Drive of PIO Line 27"]
pub type Line27W<'a, REG> = crate::BitWriter<'a, REG, Line27select>;
impl<'a, REG> Line27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line27select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line27select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line28select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line28select> for bool {
    #[inline(always)]
    fn from(variant: Line28select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE28` reader - Drive of PIO Line 28"]
pub type Line28R = crate::BitReader<Line28select>;
impl Line28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line28select {
        match self.bits {
            false => Line28select::LowDrive,
            true => Line28select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line28select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line28select::HighDrive
    }
}
#[doc = "Field `LINE28` writer - Drive of PIO Line 28"]
pub type Line28W<'a, REG> = crate::BitWriter<'a, REG, Line28select>;
impl<'a, REG> Line28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line28select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line28select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line29select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line29select> for bool {
    #[inline(always)]
    fn from(variant: Line29select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE29` reader - Drive of PIO Line 29"]
pub type Line29R = crate::BitReader<Line29select>;
impl Line29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line29select {
        match self.bits {
            false => Line29select::LowDrive,
            true => Line29select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line29select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line29select::HighDrive
    }
}
#[doc = "Field `LINE29` writer - Drive of PIO Line 29"]
pub type Line29W<'a, REG> = crate::BitWriter<'a, REG, Line29select>;
impl<'a, REG> Line29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line29select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line29select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line30select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line30select> for bool {
    #[inline(always)]
    fn from(variant: Line30select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE30` reader - Drive of PIO Line 30"]
pub type Line30R = crate::BitReader<Line30select>;
impl Line30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line30select {
        match self.bits {
            false => Line30select::LowDrive,
            true => Line30select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line30select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line30select::HighDrive
    }
}
#[doc = "Field `LINE30` writer - Drive of PIO Line 30"]
pub type Line30W<'a, REG> = crate::BitWriter<'a, REG, Line30select>;
impl<'a, REG> Line30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line30select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line30select::HighDrive)
    }
}
#[doc = "Drive of PIO Line 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line31select {
    #[doc = "0: Lowest drive"]
    LowDrive = 0,
    #[doc = "1: Highest drive"]
    HighDrive = 1,
}
impl From<Line31select> for bool {
    #[inline(always)]
    fn from(variant: Line31select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE31` reader - Drive of PIO Line 31"]
pub type Line31R = crate::BitReader<Line31select>;
impl Line31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line31select {
        match self.bits {
            false => Line31select::LowDrive,
            true => Line31select::HighDrive,
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == Line31select::LowDrive
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == Line31select::HighDrive
    }
}
#[doc = "Field `LINE31` writer - Drive of PIO Line 31"]
pub type Line31W<'a, REG> = crate::BitWriter<'a, REG, Line31select>;
impl<'a, REG> Line31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line31select::LowDrive)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Line31select::HighDrive)
    }
}
impl R {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&self) -> Line0R {
        Line0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&self) -> Line1R {
        Line1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&self) -> Line2R {
        Line2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&self) -> Line3R {
        Line3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&self) -> Line4R {
        Line4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&self) -> Line5R {
        Line5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&self) -> Line6R {
        Line6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&self) -> Line7R {
        Line7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&self) -> Line8R {
        Line8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&self) -> Line9R {
        Line9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&self) -> Line10R {
        Line10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&self) -> Line11R {
        Line11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&self) -> Line12R {
        Line12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&self) -> Line13R {
        Line13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&self) -> Line14R {
        Line14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&self) -> Line15R {
        Line15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&self) -> Line16R {
        Line16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&self) -> Line17R {
        Line17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&self) -> Line18R {
        Line18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&self) -> Line19R {
        Line19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&self) -> Line20R {
        Line20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&self) -> Line21R {
        Line21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&self) -> Line22R {
        Line22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&self) -> Line23R {
        Line23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&self) -> Line24R {
        Line24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&self) -> Line25R {
        Line25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&self) -> Line26R {
        Line26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&self) -> Line27R {
        Line27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&self) -> Line28R {
        Line28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&self) -> Line29R {
        Line29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&self) -> Line30R {
        Line30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&self) -> Line31R {
        Line31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&mut self) -> Line0W<DriverSpec> {
        Line0W::new(self, 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&mut self) -> Line1W<DriverSpec> {
        Line1W::new(self, 1)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&mut self) -> Line2W<DriverSpec> {
        Line2W::new(self, 2)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&mut self) -> Line3W<DriverSpec> {
        Line3W::new(self, 3)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&mut self) -> Line4W<DriverSpec> {
        Line4W::new(self, 4)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&mut self) -> Line5W<DriverSpec> {
        Line5W::new(self, 5)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&mut self) -> Line6W<DriverSpec> {
        Line6W::new(self, 6)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&mut self) -> Line7W<DriverSpec> {
        Line7W::new(self, 7)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&mut self) -> Line8W<DriverSpec> {
        Line8W::new(self, 8)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&mut self) -> Line9W<DriverSpec> {
        Line9W::new(self, 9)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&mut self) -> Line10W<DriverSpec> {
        Line10W::new(self, 10)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&mut self) -> Line11W<DriverSpec> {
        Line11W::new(self, 11)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&mut self) -> Line12W<DriverSpec> {
        Line12W::new(self, 12)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&mut self) -> Line13W<DriverSpec> {
        Line13W::new(self, 13)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&mut self) -> Line14W<DriverSpec> {
        Line14W::new(self, 14)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&mut self) -> Line15W<DriverSpec> {
        Line15W::new(self, 15)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&mut self) -> Line16W<DriverSpec> {
        Line16W::new(self, 16)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&mut self) -> Line17W<DriverSpec> {
        Line17W::new(self, 17)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&mut self) -> Line18W<DriverSpec> {
        Line18W::new(self, 18)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&mut self) -> Line19W<DriverSpec> {
        Line19W::new(self, 19)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&mut self) -> Line20W<DriverSpec> {
        Line20W::new(self, 20)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&mut self) -> Line21W<DriverSpec> {
        Line21W::new(self, 21)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&mut self) -> Line22W<DriverSpec> {
        Line22W::new(self, 22)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&mut self) -> Line23W<DriverSpec> {
        Line23W::new(self, 23)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&mut self) -> Line24W<DriverSpec> {
        Line24W::new(self, 24)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&mut self) -> Line25W<DriverSpec> {
        Line25W::new(self, 25)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&mut self) -> Line26W<DriverSpec> {
        Line26W::new(self, 26)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&mut self) -> Line27W<DriverSpec> {
        Line27W::new(self, 27)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&mut self) -> Line28W<DriverSpec> {
        Line28W::new(self, 28)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&mut self) -> Line29W<DriverSpec> {
        Line29W::new(self, 29)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&mut self) -> Line30W<DriverSpec> {
        Line30W::new(self, 30)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&mut self) -> Line31W<DriverSpec> {
        Line31W::new(self, 31)
    }
}
#[doc = "I/O Drive Register\n\nYou can [`read`](crate::Reg::read) this register and get [`driver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`driver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DriverSpec;
impl crate::RegisterSpec for DriverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`driver::R`](R) reader structure"]
impl crate::Readable for DriverSpec {}
#[doc = "`write(|w| ..)` method takes [`driver::W`](W) writer structure"]
impl crate::Writable for DriverSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DRIVER to value 0"]
impl crate::Resettable for DriverSpec {}
