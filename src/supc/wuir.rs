#[doc = "Register `WUIR` reader"]
pub type R = crate::R<WuirSpec>;
#[doc = "Register `WUIR` writer"]
pub type W = crate::W<WuirSpec>;
#[doc = "Wake-up Input Enable 0 to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen0select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen0select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN0` reader - Wake-up Input Enable 0 to 0"]
pub type Wkupen0R = crate::BitReader<Wkupen0select>;
impl Wkupen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen0select {
        match self.bits {
            false => Wkupen0select::Disable,
            true => Wkupen0select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen0select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen0select::Enable
    }
}
#[doc = "Field `WKUPEN0` writer - Wake-up Input Enable 0 to 0"]
pub type Wkupen0W<'a, REG> = crate::BitWriter<'a, REG, Wkupen0select>;
impl<'a, REG> Wkupen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen0select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen0select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen1select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen1select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN1` reader - Wake-up Input Enable 0 to 1"]
pub type Wkupen1R = crate::BitReader<Wkupen1select>;
impl Wkupen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen1select {
        match self.bits {
            false => Wkupen1select::Disable,
            true => Wkupen1select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen1select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen1select::Enable
    }
}
#[doc = "Field `WKUPEN1` writer - Wake-up Input Enable 0 to 1"]
pub type Wkupen1W<'a, REG> = crate::BitWriter<'a, REG, Wkupen1select>;
impl<'a, REG> Wkupen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen1select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen1select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen2select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen2select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN2` reader - Wake-up Input Enable 0 to 2"]
pub type Wkupen2R = crate::BitReader<Wkupen2select>;
impl Wkupen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen2select {
        match self.bits {
            false => Wkupen2select::Disable,
            true => Wkupen2select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen2select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen2select::Enable
    }
}
#[doc = "Field `WKUPEN2` writer - Wake-up Input Enable 0 to 2"]
pub type Wkupen2W<'a, REG> = crate::BitWriter<'a, REG, Wkupen2select>;
impl<'a, REG> Wkupen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen2select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen2select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen3select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen3select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN3` reader - Wake-up Input Enable 0 to 3"]
pub type Wkupen3R = crate::BitReader<Wkupen3select>;
impl Wkupen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen3select {
        match self.bits {
            false => Wkupen3select::Disable,
            true => Wkupen3select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen3select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen3select::Enable
    }
}
#[doc = "Field `WKUPEN3` writer - Wake-up Input Enable 0 to 3"]
pub type Wkupen3W<'a, REG> = crate::BitWriter<'a, REG, Wkupen3select>;
impl<'a, REG> Wkupen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen3select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen3select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen4select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen4select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN4` reader - Wake-up Input Enable 0 to 4"]
pub type Wkupen4R = crate::BitReader<Wkupen4select>;
impl Wkupen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen4select {
        match self.bits {
            false => Wkupen4select::Disable,
            true => Wkupen4select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen4select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen4select::Enable
    }
}
#[doc = "Field `WKUPEN4` writer - Wake-up Input Enable 0 to 4"]
pub type Wkupen4W<'a, REG> = crate::BitWriter<'a, REG, Wkupen4select>;
impl<'a, REG> Wkupen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen4select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen4select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen5select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen5select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN5` reader - Wake-up Input Enable 0 to 5"]
pub type Wkupen5R = crate::BitReader<Wkupen5select>;
impl Wkupen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen5select {
        match self.bits {
            false => Wkupen5select::Disable,
            true => Wkupen5select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen5select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen5select::Enable
    }
}
#[doc = "Field `WKUPEN5` writer - Wake-up Input Enable 0 to 5"]
pub type Wkupen5W<'a, REG> = crate::BitWriter<'a, REG, Wkupen5select>;
impl<'a, REG> Wkupen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen5select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen5select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen6select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen6select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN6` reader - Wake-up Input Enable 0 to 6"]
pub type Wkupen6R = crate::BitReader<Wkupen6select>;
impl Wkupen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen6select {
        match self.bits {
            false => Wkupen6select::Disable,
            true => Wkupen6select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen6select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen6select::Enable
    }
}
#[doc = "Field `WKUPEN6` writer - Wake-up Input Enable 0 to 6"]
pub type Wkupen6W<'a, REG> = crate::BitWriter<'a, REG, Wkupen6select>;
impl<'a, REG> Wkupen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen6select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen6select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen7select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen7select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN7` reader - Wake-up Input Enable 0 to 7"]
pub type Wkupen7R = crate::BitReader<Wkupen7select>;
impl Wkupen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen7select {
        match self.bits {
            false => Wkupen7select::Disable,
            true => Wkupen7select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen7select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen7select::Enable
    }
}
#[doc = "Field `WKUPEN7` writer - Wake-up Input Enable 0 to 7"]
pub type Wkupen7W<'a, REG> = crate::BitWriter<'a, REG, Wkupen7select>;
impl<'a, REG> Wkupen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen7select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen7select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen8select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen8select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN8` reader - Wake-up Input Enable 0 to 8"]
pub type Wkupen8R = crate::BitReader<Wkupen8select>;
impl Wkupen8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen8select {
        match self.bits {
            false => Wkupen8select::Disable,
            true => Wkupen8select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen8select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen8select::Enable
    }
}
#[doc = "Field `WKUPEN8` writer - Wake-up Input Enable 0 to 8"]
pub type Wkupen8W<'a, REG> = crate::BitWriter<'a, REG, Wkupen8select>;
impl<'a, REG> Wkupen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen8select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen8select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen9select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen9select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN9` reader - Wake-up Input Enable 0 to 9"]
pub type Wkupen9R = crate::BitReader<Wkupen9select>;
impl Wkupen9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen9select {
        match self.bits {
            false => Wkupen9select::Disable,
            true => Wkupen9select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen9select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen9select::Enable
    }
}
#[doc = "Field `WKUPEN9` writer - Wake-up Input Enable 0 to 9"]
pub type Wkupen9W<'a, REG> = crate::BitWriter<'a, REG, Wkupen9select>;
impl<'a, REG> Wkupen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen9select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen9select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen10select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen10select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN10` reader - Wake-up Input Enable 0 to 10"]
pub type Wkupen10R = crate::BitReader<Wkupen10select>;
impl Wkupen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen10select {
        match self.bits {
            false => Wkupen10select::Disable,
            true => Wkupen10select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen10select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen10select::Enable
    }
}
#[doc = "Field `WKUPEN10` writer - Wake-up Input Enable 0 to 10"]
pub type Wkupen10W<'a, REG> = crate::BitWriter<'a, REG, Wkupen10select>;
impl<'a, REG> Wkupen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen10select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen10select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen11select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen11select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN11` reader - Wake-up Input Enable 0 to 11"]
pub type Wkupen11R = crate::BitReader<Wkupen11select>;
impl Wkupen11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen11select {
        match self.bits {
            false => Wkupen11select::Disable,
            true => Wkupen11select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen11select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen11select::Enable
    }
}
#[doc = "Field `WKUPEN11` writer - Wake-up Input Enable 0 to 11"]
pub type Wkupen11W<'a, REG> = crate::BitWriter<'a, REG, Wkupen11select>;
impl<'a, REG> Wkupen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen11select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen11select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen12select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen12select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN12` reader - Wake-up Input Enable 0 to 12"]
pub type Wkupen12R = crate::BitReader<Wkupen12select>;
impl Wkupen12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen12select {
        match self.bits {
            false => Wkupen12select::Disable,
            true => Wkupen12select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen12select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen12select::Enable
    }
}
#[doc = "Field `WKUPEN12` writer - Wake-up Input Enable 0 to 12"]
pub type Wkupen12W<'a, REG> = crate::BitWriter<'a, REG, Wkupen12select>;
impl<'a, REG> Wkupen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen12select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen12select::Enable)
    }
}
#[doc = "Wake-up Input Enable 0 to 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen13select {
    #[doc = "0: The corresponding wake-up input has no wake-up effect."]
    Disable = 0,
    #[doc = "1: The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen13select> for bool {
    #[inline(always)]
    fn from(variant: Wkupen13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN13` reader - Wake-up Input Enable 0 to 13"]
pub type Wkupen13R = crate::BitReader<Wkupen13select>;
impl Wkupen13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen13select {
        match self.bits {
            false => Wkupen13select::Disable,
            true => Wkupen13select::Enable,
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen13select::Disable
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen13select::Enable
    }
}
#[doc = "Field `WKUPEN13` writer - Wake-up Input Enable 0 to 13"]
pub type Wkupen13W<'a, REG> = crate::BitWriter<'a, REG, Wkupen13select>;
impl<'a, REG> Wkupen13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen13select::Disable)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen13select::Enable)
    }
}
#[doc = "Wake-up Input Type 0 to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt0select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt0select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT0` reader - Wake-up Input Type 0 to 0"]
pub type Wkupt0R = crate::BitReader<Wkupt0select>;
impl Wkupt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt0select {
        match self.bits {
            false => Wkupt0select::Low,
            true => Wkupt0select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt0select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt0select::High
    }
}
#[doc = "Field `WKUPT0` writer - Wake-up Input Type 0 to 0"]
pub type Wkupt0W<'a, REG> = crate::BitWriter<'a, REG, Wkupt0select>;
impl<'a, REG> Wkupt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt0select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt0select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt1select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt1select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT1` reader - Wake-up Input Type 0 to 1"]
pub type Wkupt1R = crate::BitReader<Wkupt1select>;
impl Wkupt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt1select {
        match self.bits {
            false => Wkupt1select::Low,
            true => Wkupt1select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt1select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt1select::High
    }
}
#[doc = "Field `WKUPT1` writer - Wake-up Input Type 0 to 1"]
pub type Wkupt1W<'a, REG> = crate::BitWriter<'a, REG, Wkupt1select>;
impl<'a, REG> Wkupt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt1select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt1select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt2select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt2select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT2` reader - Wake-up Input Type 0 to 2"]
pub type Wkupt2R = crate::BitReader<Wkupt2select>;
impl Wkupt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt2select {
        match self.bits {
            false => Wkupt2select::Low,
            true => Wkupt2select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt2select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt2select::High
    }
}
#[doc = "Field `WKUPT2` writer - Wake-up Input Type 0 to 2"]
pub type Wkupt2W<'a, REG> = crate::BitWriter<'a, REG, Wkupt2select>;
impl<'a, REG> Wkupt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt2select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt2select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt3select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt3select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT3` reader - Wake-up Input Type 0 to 3"]
pub type Wkupt3R = crate::BitReader<Wkupt3select>;
impl Wkupt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt3select {
        match self.bits {
            false => Wkupt3select::Low,
            true => Wkupt3select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt3select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt3select::High
    }
}
#[doc = "Field `WKUPT3` writer - Wake-up Input Type 0 to 3"]
pub type Wkupt3W<'a, REG> = crate::BitWriter<'a, REG, Wkupt3select>;
impl<'a, REG> Wkupt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt3select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt3select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt4select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt4select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT4` reader - Wake-up Input Type 0 to 4"]
pub type Wkupt4R = crate::BitReader<Wkupt4select>;
impl Wkupt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt4select {
        match self.bits {
            false => Wkupt4select::Low,
            true => Wkupt4select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt4select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt4select::High
    }
}
#[doc = "Field `WKUPT4` writer - Wake-up Input Type 0 to 4"]
pub type Wkupt4W<'a, REG> = crate::BitWriter<'a, REG, Wkupt4select>;
impl<'a, REG> Wkupt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt4select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt4select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt5select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt5select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT5` reader - Wake-up Input Type 0 to 5"]
pub type Wkupt5R = crate::BitReader<Wkupt5select>;
impl Wkupt5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt5select {
        match self.bits {
            false => Wkupt5select::Low,
            true => Wkupt5select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt5select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt5select::High
    }
}
#[doc = "Field `WKUPT5` writer - Wake-up Input Type 0 to 5"]
pub type Wkupt5W<'a, REG> = crate::BitWriter<'a, REG, Wkupt5select>;
impl<'a, REG> Wkupt5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt5select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt5select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt6select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt6select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT6` reader - Wake-up Input Type 0 to 6"]
pub type Wkupt6R = crate::BitReader<Wkupt6select>;
impl Wkupt6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt6select {
        match self.bits {
            false => Wkupt6select::Low,
            true => Wkupt6select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt6select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt6select::High
    }
}
#[doc = "Field `WKUPT6` writer - Wake-up Input Type 0 to 6"]
pub type Wkupt6W<'a, REG> = crate::BitWriter<'a, REG, Wkupt6select>;
impl<'a, REG> Wkupt6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt6select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt6select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt7select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt7select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT7` reader - Wake-up Input Type 0 to 7"]
pub type Wkupt7R = crate::BitReader<Wkupt7select>;
impl Wkupt7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt7select {
        match self.bits {
            false => Wkupt7select::Low,
            true => Wkupt7select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt7select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt7select::High
    }
}
#[doc = "Field `WKUPT7` writer - Wake-up Input Type 0 to 7"]
pub type Wkupt7W<'a, REG> = crate::BitWriter<'a, REG, Wkupt7select>;
impl<'a, REG> Wkupt7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt7select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt7select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt8select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt8select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT8` reader - Wake-up Input Type 0 to 8"]
pub type Wkupt8R = crate::BitReader<Wkupt8select>;
impl Wkupt8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt8select {
        match self.bits {
            false => Wkupt8select::Low,
            true => Wkupt8select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt8select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt8select::High
    }
}
#[doc = "Field `WKUPT8` writer - Wake-up Input Type 0 to 8"]
pub type Wkupt8W<'a, REG> = crate::BitWriter<'a, REG, Wkupt8select>;
impl<'a, REG> Wkupt8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt8select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt8select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt9select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt9select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT9` reader - Wake-up Input Type 0 to 9"]
pub type Wkupt9R = crate::BitReader<Wkupt9select>;
impl Wkupt9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt9select {
        match self.bits {
            false => Wkupt9select::Low,
            true => Wkupt9select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt9select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt9select::High
    }
}
#[doc = "Field `WKUPT9` writer - Wake-up Input Type 0 to 9"]
pub type Wkupt9W<'a, REG> = crate::BitWriter<'a, REG, Wkupt9select>;
impl<'a, REG> Wkupt9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt9select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt9select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt10select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt10select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT10` reader - Wake-up Input Type 0 to 10"]
pub type Wkupt10R = crate::BitReader<Wkupt10select>;
impl Wkupt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt10select {
        match self.bits {
            false => Wkupt10select::Low,
            true => Wkupt10select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt10select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt10select::High
    }
}
#[doc = "Field `WKUPT10` writer - Wake-up Input Type 0 to 10"]
pub type Wkupt10W<'a, REG> = crate::BitWriter<'a, REG, Wkupt10select>;
impl<'a, REG> Wkupt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt10select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt10select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt11select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt11select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT11` reader - Wake-up Input Type 0 to 11"]
pub type Wkupt11R = crate::BitReader<Wkupt11select>;
impl Wkupt11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt11select {
        match self.bits {
            false => Wkupt11select::Low,
            true => Wkupt11select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt11select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt11select::High
    }
}
#[doc = "Field `WKUPT11` writer - Wake-up Input Type 0 to 11"]
pub type Wkupt11W<'a, REG> = crate::BitWriter<'a, REG, Wkupt11select>;
impl<'a, REG> Wkupt11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt11select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt11select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt12select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt12select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT12` reader - Wake-up Input Type 0 to 12"]
pub type Wkupt12R = crate::BitReader<Wkupt12select>;
impl Wkupt12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt12select {
        match self.bits {
            false => Wkupt12select::Low,
            true => Wkupt12select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt12select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt12select::High
    }
}
#[doc = "Field `WKUPT12` writer - Wake-up Input Type 0 to 12"]
pub type Wkupt12W<'a, REG> = crate::BitWriter<'a, REG, Wkupt12select>;
impl<'a, REG> Wkupt12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt12select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt12select::High)
    }
}
#[doc = "Wake-up Input Type 0 to 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt13select {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    Low = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    High = 1,
}
impl From<Wkupt13select> for bool {
    #[inline(always)]
    fn from(variant: Wkupt13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT13` reader - Wake-up Input Type 0 to 13"]
pub type Wkupt13R = crate::BitReader<Wkupt13select>;
impl Wkupt13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt13select {
        match self.bits {
            false => Wkupt13select::Low,
            true => Wkupt13select::High,
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt13select::Low
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt13select::High
    }
}
#[doc = "Field `WKUPT13` writer - Wake-up Input Type 0 to 13"]
pub type Wkupt13W<'a, REG> = crate::BitWriter<'a, REG, Wkupt13select>;
impl<'a, REG> Wkupt13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt13select::Low)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt13select::High)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&self) -> Wkupen0R {
        Wkupen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> Wkupen1R {
        Wkupen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> Wkupen2R {
        Wkupen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> Wkupen3R {
        Wkupen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> Wkupen4R {
        Wkupen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> Wkupen5R {
        Wkupen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> Wkupen6R {
        Wkupen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&self) -> Wkupen7R {
        Wkupen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&self) -> Wkupen8R {
        Wkupen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&self) -> Wkupen9R {
        Wkupen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&self) -> Wkupen10R {
        Wkupen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&self) -> Wkupen11R {
        Wkupen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&self) -> Wkupen12R {
        Wkupen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&self) -> Wkupen13R {
        Wkupen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&self) -> Wkupt0R {
        Wkupt0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&self) -> Wkupt1R {
        Wkupt1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&self) -> Wkupt2R {
        Wkupt2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&self) -> Wkupt3R {
        Wkupt3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&self) -> Wkupt4R {
        Wkupt4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&self) -> Wkupt5R {
        Wkupt5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&self) -> Wkupt6R {
        Wkupt6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&self) -> Wkupt7R {
        Wkupt7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&self) -> Wkupt8R {
        Wkupt8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&self) -> Wkupt9R {
        Wkupt9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&self) -> Wkupt10R {
        Wkupt10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&self) -> Wkupt11R {
        Wkupt11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&self) -> Wkupt12R {
        Wkupt12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&self) -> Wkupt13R {
        Wkupt13R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&mut self) -> Wkupen0W<WuirSpec> {
        Wkupen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> Wkupen1W<WuirSpec> {
        Wkupen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> Wkupen2W<WuirSpec> {
        Wkupen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> Wkupen3W<WuirSpec> {
        Wkupen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> Wkupen4W<WuirSpec> {
        Wkupen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> Wkupen5W<WuirSpec> {
        Wkupen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> Wkupen6W<WuirSpec> {
        Wkupen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&mut self) -> Wkupen7W<WuirSpec> {
        Wkupen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&mut self) -> Wkupen8W<WuirSpec> {
        Wkupen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&mut self) -> Wkupen9W<WuirSpec> {
        Wkupen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&mut self) -> Wkupen10W<WuirSpec> {
        Wkupen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&mut self) -> Wkupen11W<WuirSpec> {
        Wkupen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&mut self) -> Wkupen12W<WuirSpec> {
        Wkupen12W::new(self, 12)
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&mut self) -> Wkupen13W<WuirSpec> {
        Wkupen13W::new(self, 13)
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&mut self) -> Wkupt0W<WuirSpec> {
        Wkupt0W::new(self, 16)
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&mut self) -> Wkupt1W<WuirSpec> {
        Wkupt1W::new(self, 17)
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&mut self) -> Wkupt2W<WuirSpec> {
        Wkupt2W::new(self, 18)
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&mut self) -> Wkupt3W<WuirSpec> {
        Wkupt3W::new(self, 19)
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&mut self) -> Wkupt4W<WuirSpec> {
        Wkupt4W::new(self, 20)
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&mut self) -> Wkupt5W<WuirSpec> {
        Wkupt5W::new(self, 21)
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&mut self) -> Wkupt6W<WuirSpec> {
        Wkupt6W::new(self, 22)
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&mut self) -> Wkupt7W<WuirSpec> {
        Wkupt7W::new(self, 23)
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&mut self) -> Wkupt8W<WuirSpec> {
        Wkupt8W::new(self, 24)
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&mut self) -> Wkupt9W<WuirSpec> {
        Wkupt9W::new(self, 25)
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&mut self) -> Wkupt10W<WuirSpec> {
        Wkupt10W::new(self, 26)
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&mut self) -> Wkupt11W<WuirSpec> {
        Wkupt11W::new(self, 27)
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&mut self) -> Wkupt12W<WuirSpec> {
        Wkupt12W::new(self, 28)
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&mut self) -> Wkupt13W<WuirSpec> {
        Wkupt13W::new(self, 29)
    }
}
#[doc = "Supply Controller Wake-up Inputs Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wuir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuirSpec;
impl crate::RegisterSpec for WuirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuir::R`](R) reader structure"]
impl crate::Readable for WuirSpec {}
#[doc = "`write(|w| ..)` method takes [`wuir::W`](W) writer structure"]
impl crate::Writable for WuirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUIR to value 0"]
impl crate::Resettable for WuirSpec {}
