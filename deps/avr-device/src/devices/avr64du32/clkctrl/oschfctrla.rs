#[doc = "Register `OSCHFCTRLA` reader"]
pub struct R(crate::R<OSCHFCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCHFCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCHFCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCHFCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCHFCTRLA` writer"]
pub struct W(crate::W<OSCHFCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCHFCTRLA_SPEC>;
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
impl From<crate::W<OSCHFCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCHFCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOTUNE` reader - Autotune"]
pub type AUTOTUNE_R = crate::FieldReader<u8, AUTOTUNE_A>;
#[doc = "Autotune\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUTOTUNE_A {
    #[doc = "0: Automatic tuning disabled"]
    OFF = 0,
    #[doc = "1: Automatic tuning against XOSC32K enabled"]
    _32K = 1,
    #[doc = "2: Automatic tuning against USB SOF enabled"]
    SOF = 2,
}
impl From<AUTOTUNE_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTOTUNE_A) -> Self {
        variant as _
    }
}
impl AUTOTUNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUTOTUNE_A> {
        match self.bits {
            0 => Some(AUTOTUNE_A::OFF),
            1 => Some(AUTOTUNE_A::_32K),
            2 => Some(AUTOTUNE_A::SOF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTOTUNE_A::OFF
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == AUTOTUNE_A::_32K
    }
    #[doc = "Checks if the value of the field is `SOF`"]
    #[inline(always)]
    pub fn is_sof(&self) -> bool {
        *self == AUTOTUNE_A::SOF
    }
}
#[doc = "Field `AUTOTUNE` writer - Autotune"]
pub type AUTOTUNE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, OSCHFCTRLA_SPEC, u8, AUTOTUNE_A, 2, O>;
impl<'a, const O: u8> AUTOTUNE_W<'a, O> {
    #[doc = "Automatic tuning disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AUTOTUNE_A::OFF)
    }
    #[doc = "Automatic tuning against XOSC32K enabled"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(AUTOTUNE_A::_32K)
    }
    #[doc = "Automatic tuning against USB SOF enabled"]
    #[inline(always)]
    pub fn sof(self) -> &'a mut W {
        self.variant(AUTOTUNE_A::SOF)
    }
}
#[doc = "Field `FRQSEL` reader - Frequency select"]
pub type FRQSEL_R = crate::FieldReader<u8, FRQSEL_A>;
#[doc = "Frequency select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQSEL_A {
    #[doc = "0: 1 MHz system clock"]
    _1M = 0,
    #[doc = "1: 2 MHz system clock"]
    _2M = 1,
    #[doc = "2: 3 MHz system clock"]
    _3M = 2,
    #[doc = "3: 4 MHz system clock (default)"]
    _4M = 3,
    #[doc = "5: 8 MHz system clock"]
    _8M = 5,
    #[doc = "6: 12 MHz system clock"]
    _12M = 6,
    #[doc = "7: 16 MHz system clock"]
    _16M = 7,
    #[doc = "8: 20 MHz system clock"]
    _20M = 8,
    #[doc = "9: 24 MHz system clock"]
    _24M = 9,
    #[doc = "10: 28 MHz system clock"]
    _28M = 10,
    #[doc = "11: 32 MHz system clock"]
    _32M = 11,
}
impl From<FRQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQSEL_A) -> Self {
        variant as _
    }
}
impl FRQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRQSEL_A> {
        match self.bits {
            0 => Some(FRQSEL_A::_1M),
            1 => Some(FRQSEL_A::_2M),
            2 => Some(FRQSEL_A::_3M),
            3 => Some(FRQSEL_A::_4M),
            5 => Some(FRQSEL_A::_8M),
            6 => Some(FRQSEL_A::_12M),
            7 => Some(FRQSEL_A::_16M),
            8 => Some(FRQSEL_A::_20M),
            9 => Some(FRQSEL_A::_24M),
            10 => Some(FRQSEL_A::_28M),
            11 => Some(FRQSEL_A::_32M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == FRQSEL_A::_1M
    }
    #[doc = "Checks if the value of the field is `_2M`"]
    #[inline(always)]
    pub fn is_2m(&self) -> bool {
        *self == FRQSEL_A::_2M
    }
    #[doc = "Checks if the value of the field is `_3M`"]
    #[inline(always)]
    pub fn is_3m(&self) -> bool {
        *self == FRQSEL_A::_3M
    }
    #[doc = "Checks if the value of the field is `_4M`"]
    #[inline(always)]
    pub fn is_4m(&self) -> bool {
        *self == FRQSEL_A::_4M
    }
    #[doc = "Checks if the value of the field is `_8M`"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == FRQSEL_A::_8M
    }
    #[doc = "Checks if the value of the field is `_12M`"]
    #[inline(always)]
    pub fn is_12m(&self) -> bool {
        *self == FRQSEL_A::_12M
    }
    #[doc = "Checks if the value of the field is `_16M`"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == FRQSEL_A::_16M
    }
    #[doc = "Checks if the value of the field is `_20M`"]
    #[inline(always)]
    pub fn is_20m(&self) -> bool {
        *self == FRQSEL_A::_20M
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == FRQSEL_A::_24M
    }
    #[doc = "Checks if the value of the field is `_28M`"]
    #[inline(always)]
    pub fn is_28m(&self) -> bool {
        *self == FRQSEL_A::_28M
    }
    #[doc = "Checks if the value of the field is `_32M`"]
    #[inline(always)]
    pub fn is_32m(&self) -> bool {
        *self == FRQSEL_A::_32M
    }
}
#[doc = "Field `FRQSEL` writer - Frequency select"]
pub type FRQSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, OSCHFCTRLA_SPEC, u8, FRQSEL_A, 4, O>;
impl<'a, const O: u8> FRQSEL_W<'a, O> {
    #[doc = "1 MHz system clock"]
    #[inline(always)]
    pub fn _1m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_1M)
    }
    #[doc = "2 MHz system clock"]
    #[inline(always)]
    pub fn _2m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_2M)
    }
    #[doc = "3 MHz system clock"]
    #[inline(always)]
    pub fn _3m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_3M)
    }
    #[doc = "4 MHz system clock (default)"]
    #[inline(always)]
    pub fn _4m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_4M)
    }
    #[doc = "8 MHz system clock"]
    #[inline(always)]
    pub fn _8m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_8M)
    }
    #[doc = "12 MHz system clock"]
    #[inline(always)]
    pub fn _12m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_12M)
    }
    #[doc = "16 MHz system clock"]
    #[inline(always)]
    pub fn _16m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_16M)
    }
    #[doc = "20 MHz system clock"]
    #[inline(always)]
    pub fn _20m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_20M)
    }
    #[doc = "24 MHz system clock"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_24M)
    }
    #[doc = "28 MHz system clock"]
    #[inline(always)]
    pub fn _28m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_28M)
    }
    #[doc = "32 MHz system clock"]
    #[inline(always)]
    pub fn _32m(self) -> &'a mut W {
        self.variant(FRQSEL_A::_32M)
    }
}
#[doc = "Field `ALGSEL` reader - Algorithm Selection"]
pub type ALGSEL_R = crate::BitReader<ALGSEL_A>;
#[doc = "Algorithm Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALGSEL_A {
    #[doc = "0: Binary Search"]
    BIN = 0,
    #[doc = "1: Incremental Search"]
    INCR = 1,
}
impl From<ALGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ALGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ALGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALGSEL_A {
        match self.bits {
            false => ALGSEL_A::BIN,
            true => ALGSEL_A::INCR,
        }
    }
    #[doc = "Checks if the value of the field is `BIN`"]
    #[inline(always)]
    pub fn is_bin(&self) -> bool {
        *self == ALGSEL_A::BIN
    }
    #[doc = "Checks if the value of the field is `INCR`"]
    #[inline(always)]
    pub fn is_incr(&self) -> bool {
        *self == ALGSEL_A::INCR
    }
}
#[doc = "Field `ALGSEL` writer - Algorithm Selection"]
pub type ALGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSCHFCTRLA_SPEC, ALGSEL_A, O>;
impl<'a, const O: u8> ALGSEL_W<'a, O> {
    #[doc = "Binary Search"]
    #[inline(always)]
    pub fn bin(self) -> &'a mut W {
        self.variant(ALGSEL_A::BIN)
    }
    #[doc = "Incremental Search"]
    #[inline(always)]
    pub fn incr(self) -> &'a mut W {
        self.variant(ALGSEL_A::INCR)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSCHFCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Autotune"]
    #[inline(always)]
    pub fn autotune(&self) -> AUTOTUNE_R {
        AUTOTUNE_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:5 - Frequency select"]
    #[inline(always)]
    pub fn frqsel(&self) -> FRQSEL_R {
        FRQSEL_R::new((self.bits >> 2) & 0x0f)
    }
    #[doc = "Bit 6 - Algorithm Selection"]
    #[inline(always)]
    pub fn algsel(&self) -> ALGSEL_R {
        ALGSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Autotune"]
    #[inline(always)]
    #[must_use]
    pub fn autotune(&mut self) -> AUTOTUNE_W<0> {
        AUTOTUNE_W::new(self)
    }
    #[doc = "Bits 2:5 - Frequency select"]
    #[inline(always)]
    #[must_use]
    pub fn frqsel(&mut self) -> FRQSEL_W<2> {
        FRQSEL_W::new(self)
    }
    #[doc = "Bit 6 - Algorithm Selection"]
    #[inline(always)]
    #[must_use]
    pub fn algsel(&mut self) -> ALGSEL_W<6> {
        ALGSEL_W::new(self)
    }
    #[doc = "Bit 7 - Run standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSCHF Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oschfctrla](index.html) module"]
pub struct OSCHFCTRLA_SPEC;
impl crate::RegisterSpec for OSCHFCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [oschfctrla::R](R) reader structure"]
impl crate::Readable for OSCHFCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oschfctrla::W](W) writer structure"]
impl crate::Writable for OSCHFCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCHFCTRLA to value 0"]
impl crate::Resettable for OSCHFCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
