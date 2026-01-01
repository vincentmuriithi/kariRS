#[doc = "Register `SYSCFG1` reader"]
pub struct R(crate::R<SYSCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUT` reader - Startup Time"]
pub type SUT_R = crate::FieldReader<u8, SUT_A>;
#[doc = "Startup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUT_A {
    #[doc = "0: 0 ms"]
    _0MS = 0,
    #[doc = "1: 1 ms"]
    _1MS = 1,
    #[doc = "2: 2 ms"]
    _2MS = 2,
    #[doc = "3: 4 ms"]
    _4MS = 3,
    #[doc = "4: 8 ms"]
    _8MS = 4,
    #[doc = "5: 16 ms"]
    _16MS = 5,
    #[doc = "6: 32 ms"]
    _32MS = 6,
    #[doc = "7: 64 ms"]
    _64MS = 7,
}
impl From<SUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUT_A) -> Self {
        variant as _
    }
}
impl SUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUT_A {
        match self.bits {
            0 => SUT_A::_0MS,
            1 => SUT_A::_1MS,
            2 => SUT_A::_2MS,
            3 => SUT_A::_4MS,
            4 => SUT_A::_8MS,
            5 => SUT_A::_16MS,
            6 => SUT_A::_32MS,
            7 => SUT_A::_64MS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0MS`"]
    #[inline(always)]
    pub fn is_0ms(&self) -> bool {
        *self == SUT_A::_0MS
    }
    #[doc = "Checks if the value of the field is `_1MS`"]
    #[inline(always)]
    pub fn is_1ms(&self) -> bool {
        *self == SUT_A::_1MS
    }
    #[doc = "Checks if the value of the field is `_2MS`"]
    #[inline(always)]
    pub fn is_2ms(&self) -> bool {
        *self == SUT_A::_2MS
    }
    #[doc = "Checks if the value of the field is `_4MS`"]
    #[inline(always)]
    pub fn is_4ms(&self) -> bool {
        *self == SUT_A::_4MS
    }
    #[doc = "Checks if the value of the field is `_8MS`"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        *self == SUT_A::_8MS
    }
    #[doc = "Checks if the value of the field is `_16MS`"]
    #[inline(always)]
    pub fn is_16ms(&self) -> bool {
        *self == SUT_A::_16MS
    }
    #[doc = "Checks if the value of the field is `_32MS`"]
    #[inline(always)]
    pub fn is_32ms(&self) -> bool {
        *self == SUT_A::_32MS
    }
    #[doc = "Checks if the value of the field is `_64MS`"]
    #[inline(always)]
    pub fn is_64ms(&self) -> bool {
        *self == SUT_A::_64MS
    }
}
impl R {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn sut(&self) -> SUT_R {
        SUT_R::new(self.bits & 7)
    }
}
#[doc = "System Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg1](index.html) module"]
pub struct SYSCFG1_SPEC;
impl crate::RegisterSpec for SYSCFG1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syscfg1::R](R) reader structure"]
impl crate::Readable for SYSCFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for SYSCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
