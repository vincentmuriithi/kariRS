#[doc = "Register `BODCFG` reader"]
pub struct R(crate::R<BODCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCFG` writer"]
pub struct W(crate::W<BODCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BODCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP` reader - BOD Operation in Sleep Mode"]
pub type SLEEP_R = crate::FieldReader<u8, SLEEP_A>;
#[doc = "BOD Operation in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLEEP_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
}
impl From<SLEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as _
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEEP_A> {
        match self.bits {
            0 => Some(SLEEP_A::DIS),
            1 => Some(SLEEP_A::ENABLED),
            2 => Some(SLEEP_A::SAMPLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SLEEP_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `SAMPLED`"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == SLEEP_A::SAMPLED
    }
}
#[doc = "Field `SLEEP` writer - BOD Operation in Sleep Mode"]
pub type SLEEP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BODCFG_SPEC, u8, SLEEP_A, 2, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SLEEP_A::DIS)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLEEP_A::ENABLED)
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn sampled(self) -> &'a mut W {
        self.variant(SLEEP_A::SAMPLED)
    }
}
#[doc = "Field `ACTIVE` reader - BOD Operation in Active Mode"]
pub type ACTIVE_R = crate::FieldReader<u8, ACTIVE_A>;
#[doc = "BOD Operation in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
    #[doc = "3: Enabled with wake-up halted until BOD is ready"]
    ENWAKE = 3,
}
impl From<ACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            0 => ACTIVE_A::DIS,
            1 => ACTIVE_A::ENABLED,
            2 => ACTIVE_A::SAMPLED,
            3 => ACTIVE_A::ENWAKE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACTIVE_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACTIVE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `SAMPLED`"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == ACTIVE_A::SAMPLED
    }
    #[doc = "Checks if the value of the field is `ENWAKE`"]
    #[inline(always)]
    pub fn is_enwake(&self) -> bool {
        *self == ACTIVE_A::ENWAKE
    }
}
#[doc = "Field `ACTIVE` writer - BOD Operation in Active Mode"]
pub type ACTIVE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, BODCFG_SPEC, u8, ACTIVE_A, 2, O>;
impl<'a, const O: u8> ACTIVE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACTIVE_A::DIS)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACTIVE_A::ENABLED)
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn sampled(self) -> &'a mut W {
        self.variant(ACTIVE_A::SAMPLED)
    }
    #[doc = "Enabled with wake-up halted until BOD is ready"]
    #[inline(always)]
    pub fn enwake(self) -> &'a mut W {
        self.variant(ACTIVE_A::ENWAKE)
    }
}
#[doc = "Field `SAMPFREQ` reader - BOD Sample Frequency"]
pub type SAMPFREQ_R = crate::BitReader<SAMPFREQ_A>;
#[doc = "BOD Sample Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPFREQ_A {
    #[doc = "0: 1kHz sampling frequency"]
    _1KHZ = 0,
    #[doc = "1: 125Hz sampling frequency"]
    _125HZ = 1,
}
impl From<SAMPFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPFREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPFREQ_A {
        match self.bits {
            false => SAMPFREQ_A::_1KHZ,
            true => SAMPFREQ_A::_125HZ,
        }
    }
    #[doc = "Checks if the value of the field is `_1KHZ`"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == SAMPFREQ_A::_1KHZ
    }
    #[doc = "Checks if the value of the field is `_125HZ`"]
    #[inline(always)]
    pub fn is_125hz(&self) -> bool {
        *self == SAMPFREQ_A::_125HZ
    }
}
#[doc = "Field `SAMPFREQ` writer - BOD Sample Frequency"]
pub type SAMPFREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, BODCFG_SPEC, SAMPFREQ_A, O>;
impl<'a, const O: u8> SAMPFREQ_W<'a, O> {
    #[doc = "1kHz sampling frequency"]
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut W {
        self.variant(SAMPFREQ_A::_1KHZ)
    }
    #[doc = "125Hz sampling frequency"]
    #[inline(always)]
    pub fn _125hz(self) -> &'a mut W {
        self.variant(SAMPFREQ_A::_125HZ)
    }
}
#[doc = "Field `LVL` reader - BOD Level"]
pub type LVL_R = crate::FieldReader<u8, LVL_A>;
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.8 V"]
    BODLEVEL0 = 0,
    #[doc = "2: 2.6 V"]
    BODLEVEL2 = 2,
    #[doc = "7: 4.2 V"]
    BODLEVEL7 = 7,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVL_A> {
        match self.bits {
            0 => Some(LVL_A::BODLEVEL0),
            2 => Some(LVL_A::BODLEVEL2),
            7 => Some(LVL_A::BODLEVEL7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BODLEVEL0`"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "Checks if the value of the field is `BODLEVEL2`"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "Checks if the value of the field is `BODLEVEL7`"]
    #[inline(always)]
    pub fn is_bodlevel7(&self) -> bool {
        *self == LVL_A::BODLEVEL7
    }
}
#[doc = "Field `LVL` writer - BOD Level"]
pub type LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BODCFG_SPEC, u8, LVL_A, 3, O>;
impl<'a, const O: u8> LVL_W<'a, O> {
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn bodlevel0(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL0)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn bodlevel2(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL2)
    }
    #[doc = "4.2 V"]
    #[inline(always)]
    pub fn bodlevel7(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL7)
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SAMPFREQ_R {
        SAMPFREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<0> {
        SLEEP_W::new(self)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<2> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sampfreq(&mut self) -> SAMPFREQ_W<4> {
        SAMPFREQ_W::new(self)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    #[must_use]
    pub fn lvl(&mut self) -> LVL_W<5> {
        LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcfg](index.html) module"]
pub struct BODCFG_SPEC;
impl crate::RegisterSpec for BODCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bodcfg::R](R) reader structure"]
impl crate::Readable for BODCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodcfg::W](W) writer structure"]
impl crate::Writable for BODCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BODCFG to value 0"]
impl crate::Resettable for BODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
