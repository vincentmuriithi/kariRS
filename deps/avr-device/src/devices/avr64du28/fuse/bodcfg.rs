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
    #[doc = "0: BOD disabled"]
    DISABLE = 0,
    #[doc = "1: BOD enabled in continuous mode"]
    ENABLE = 1,
    #[doc = "2: BOD enabled in sampled mode"]
    SAMPLE = 2,
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
            0 => Some(SLEEP_A::DISABLE),
            1 => Some(SLEEP_A::ENABLE),
            2 => Some(SLEEP_A::SAMPLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SLEEP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SLEEP_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `SAMPLE`"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == SLEEP_A::SAMPLE
    }
}
#[doc = "Field `SLEEP` writer - BOD Operation in Sleep Mode"]
pub type SLEEP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BODCFG_SPEC, u8, SLEEP_A, 2, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "BOD disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SLEEP_A::DISABLE)
    }
    #[doc = "BOD enabled in continuous mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SLEEP_A::ENABLE)
    }
    #[doc = "BOD enabled in sampled mode"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut W {
        self.variant(SLEEP_A::SAMPLE)
    }
}
#[doc = "Field `ACTIVE` reader - BOD Operation in Active Mode"]
pub type ACTIVE_R = crate::FieldReader<u8, ACTIVE_A>;
#[doc = "BOD Operation in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: BOD disabled"]
    DISABLE = 0,
    #[doc = "1: BOD enabled in continuous mode"]
    ENABLE = 1,
    #[doc = "2: BOD enabled in sampled mode"]
    SAMPLE = 2,
    #[doc = "3: BOD enabled in continuous mode. Execution is halted at wake-up until BOD is running."]
    ENABLEWAIT = 3,
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
            0 => ACTIVE_A::DISABLE,
            1 => ACTIVE_A::ENABLE,
            2 => ACTIVE_A::SAMPLE,
            3 => ACTIVE_A::ENABLEWAIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACTIVE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACTIVE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `SAMPLE`"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == ACTIVE_A::SAMPLE
    }
    #[doc = "Checks if the value of the field is `ENABLEWAIT`"]
    #[inline(always)]
    pub fn is_enablewait(&self) -> bool {
        *self == ACTIVE_A::ENABLEWAIT
    }
}
#[doc = "Field `ACTIVE` writer - BOD Operation in Active Mode"]
pub type ACTIVE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, BODCFG_SPEC, u8, ACTIVE_A, 2, O>;
impl<'a, const O: u8> ACTIVE_W<'a, O> {
    #[doc = "BOD disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACTIVE_A::DISABLE)
    }
    #[doc = "BOD enabled in continuous mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACTIVE_A::ENABLE)
    }
    #[doc = "BOD enabled in sampled mode"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut W {
        self.variant(ACTIVE_A::SAMPLE)
    }
    #[doc = "BOD enabled in continuous mode. Execution is halted at wake-up until BOD is running."]
    #[inline(always)]
    pub fn enablewait(self) -> &'a mut W {
        self.variant(ACTIVE_A::ENABLEWAIT)
    }
}
#[doc = "Field `SAMPFREQ` reader - BOD Sample Frequency"]
pub type SAMPFREQ_R = crate::BitReader<SAMPFREQ_A>;
#[doc = "BOD Sample Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPFREQ_A {
    #[doc = "0: Sample frequency is 128 Hz"]
    _128HZ = 0,
    #[doc = "1: Sample frequency is 32 Hz"]
    _32HZ = 1,
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
            false => SAMPFREQ_A::_128HZ,
            true => SAMPFREQ_A::_32HZ,
        }
    }
    #[doc = "Checks if the value of the field is `_128HZ`"]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        *self == SAMPFREQ_A::_128HZ
    }
    #[doc = "Checks if the value of the field is `_32HZ`"]
    #[inline(always)]
    pub fn is_32hz(&self) -> bool {
        *self == SAMPFREQ_A::_32HZ
    }
}
#[doc = "Field `SAMPFREQ` writer - BOD Sample Frequency"]
pub type SAMPFREQ_W<'a, const O: u8> = crate::BitWriter<'a, u8, BODCFG_SPEC, SAMPFREQ_A, O>;
impl<'a, const O: u8> SAMPFREQ_W<'a, O> {
    #[doc = "Sample frequency is 128 Hz"]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut W {
        self.variant(SAMPFREQ_A::_128HZ)
    }
    #[doc = "Sample frequency is 32 Hz"]
    #[inline(always)]
    pub fn _32hz(self) -> &'a mut W {
        self.variant(SAMPFREQ_A::_32HZ)
    }
}
#[doc = "Field `LVL` reader - BOD Level"]
pub type LVL_R = crate::FieldReader<u8, LVL_A>;
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.9V"]
    BODLEVEL0 = 0,
    #[doc = "1: 2.45V"]
    BODLEVEL1 = 1,
    #[doc = "2: 2.7V"]
    BODLEVEL2 = 2,
    #[doc = "3: 2.85V"]
    BODLEVEL3 = 3,
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
            1 => Some(LVL_A::BODLEVEL1),
            2 => Some(LVL_A::BODLEVEL2),
            3 => Some(LVL_A::BODLEVEL3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BODLEVEL0`"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "Checks if the value of the field is `BODLEVEL1`"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == LVL_A::BODLEVEL1
    }
    #[doc = "Checks if the value of the field is `BODLEVEL2`"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "Checks if the value of the field is `BODLEVEL3`"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == LVL_A::BODLEVEL3
    }
}
#[doc = "Field `LVL` writer - BOD Level"]
pub type LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BODCFG_SPEC, u8, LVL_A, 3, O>;
impl<'a, const O: u8> LVL_W<'a, O> {
    #[doc = "1.9V"]
    #[inline(always)]
    pub fn bodlevel0(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL0)
    }
    #[doc = "2.45V"]
    #[inline(always)]
    pub fn bodlevel1(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL1)
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn bodlevel2(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL2)
    }
    #[doc = "2.85V"]
    #[inline(always)]
    pub fn bodlevel3(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL3)
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
