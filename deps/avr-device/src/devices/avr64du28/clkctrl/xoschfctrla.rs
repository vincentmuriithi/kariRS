#[doc = "Register `XOSCHFCTRLA` reader"]
pub struct R(crate::R<XOSCHFCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSCHFCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSCHFCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSCHFCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSCHFCTRLA` writer"]
pub struct W(crate::W<XOSCHFCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSCHFCTRLA_SPEC>;
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
impl From<crate::W<XOSCHFCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSCHFCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, XOSCHFCTRLA_SPEC, bool, O>;
#[doc = "Field `SELHF` reader - External Source Select"]
pub type SELHF_R = crate::BitReader<SELHF_A>;
#[doc = "External Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELHF_A {
    #[doc = "0: External Crystal"]
    XTAL = 0,
    #[doc = "1: External clock on XTALHF1 pin"]
    EXTCLK = 1,
}
impl From<SELHF_A> for bool {
    #[inline(always)]
    fn from(variant: SELHF_A) -> Self {
        variant as u8 != 0
    }
}
impl SELHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELHF_A {
        match self.bits {
            false => SELHF_A::XTAL,
            true => SELHF_A::EXTCLK,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SELHF_A::XTAL
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == SELHF_A::EXTCLK
    }
}
#[doc = "Field `SELHF` writer - External Source Select"]
pub type SELHF_W<'a, const O: u8> = crate::BitWriter<'a, u8, XOSCHFCTRLA_SPEC, SELHF_A, O>;
impl<'a, const O: u8> SELHF_W<'a, O> {
    #[doc = "External Crystal"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SELHF_A::XTAL)
    }
    #[doc = "External clock on XTALHF1 pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(SELHF_A::EXTCLK)
    }
}
#[doc = "Field `FRQRANGE` reader - Frequency Range"]
pub type FRQRANGE_R = crate::FieldReader<u8, FRQRANGE_A>;
#[doc = "Frequency Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQRANGE_A {
    #[doc = "0: Max 8 MHz XTAL Frequency"]
    _8M = 0,
    #[doc = "1: Max 16 MHz XTAL Frequency"]
    _16M = 1,
    #[doc = "2: Max 24 MHz XTAL Frequency"]
    _24M = 2,
    #[doc = "3: Max 32 MHz XTAL Frequency"]
    _32M = 3,
}
impl From<FRQRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQRANGE_A) -> Self {
        variant as _
    }
}
impl FRQRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRQRANGE_A {
        match self.bits {
            0 => FRQRANGE_A::_8M,
            1 => FRQRANGE_A::_16M,
            2 => FRQRANGE_A::_24M,
            3 => FRQRANGE_A::_32M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8M`"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == FRQRANGE_A::_8M
    }
    #[doc = "Checks if the value of the field is `_16M`"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == FRQRANGE_A::_16M
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == FRQRANGE_A::_24M
    }
    #[doc = "Checks if the value of the field is `_32M`"]
    #[inline(always)]
    pub fn is_32m(&self) -> bool {
        *self == FRQRANGE_A::_32M
    }
}
#[doc = "Field `FRQRANGE` writer - Frequency Range"]
pub type FRQRANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, XOSCHFCTRLA_SPEC, u8, FRQRANGE_A, 2, O>;
impl<'a, const O: u8> FRQRANGE_W<'a, O> {
    #[doc = "Max 8 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _8m(self) -> &'a mut W {
        self.variant(FRQRANGE_A::_8M)
    }
    #[doc = "Max 16 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _16m(self) -> &'a mut W {
        self.variant(FRQRANGE_A::_16M)
    }
    #[doc = "Max 24 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(FRQRANGE_A::_24M)
    }
    #[doc = "Max 32 MHz XTAL Frequency"]
    #[inline(always)]
    pub fn _32m(self) -> &'a mut W {
        self.variant(FRQRANGE_A::_32M)
    }
}
#[doc = "Field `CSUTHF` reader - Start-up Time Select"]
pub type CSUTHF_R = crate::FieldReader<u8, CSUTHF_A>;
#[doc = "Start-up Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSUTHF_A {
    #[doc = "0: 256 XOSCHF cycles"]
    _256 = 0,
    #[doc = "1: 1K XOSCHF cycles"]
    _1K = 1,
    #[doc = "2: 4K XOSCHF cycles"]
    _4K = 2,
}
impl From<CSUTHF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSUTHF_A) -> Self {
        variant as _
    }
}
impl CSUTHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSUTHF_A> {
        match self.bits {
            0 => Some(CSUTHF_A::_256),
            1 => Some(CSUTHF_A::_1K),
            2 => Some(CSUTHF_A::_4K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == CSUTHF_A::_256
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == CSUTHF_A::_1K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == CSUTHF_A::_4K
    }
}
#[doc = "Field `CSUTHF` writer - Start-up Time Select"]
pub type CSUTHF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, XOSCHFCTRLA_SPEC, u8, CSUTHF_A, 2, O>;
impl<'a, const O: u8> CSUTHF_W<'a, O> {
    #[doc = "256 XOSCHF cycles"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(CSUTHF_A::_256)
    }
    #[doc = "1K XOSCHF cycles"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut W {
        self.variant(CSUTHF_A::_1K)
    }
    #[doc = "4K XOSCHF cycles"]
    #[inline(always)]
    pub fn _4k(self) -> &'a mut W {
        self.variant(CSUTHF_A::_4K)
    }
}
#[doc = "Field `RUNSTBY` reader - Run Standby"]
pub type RUNSTBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTBY` writer - Run Standby"]
pub type RUNSTBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, XOSCHFCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Source Select"]
    #[inline(always)]
    pub fn selhf(&self) -> SELHF_R {
        SELHF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frequency Range"]
    #[inline(always)]
    pub fn frqrange(&self) -> FRQRANGE_R {
        FRQRANGE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Start-up Time Select"]
    #[inline(always)]
    pub fn csuthf(&self) -> CSUTHF_R {
        CSUTHF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Run Standby"]
    #[inline(always)]
    pub fn runstby(&self) -> RUNSTBY_R {
        RUNSTBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - External Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn selhf(&mut self) -> SELHF_W<1> {
        SELHF_W::new(self)
    }
    #[doc = "Bits 2:3 - Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn frqrange(&mut self) -> FRQRANGE_W<2> {
        FRQRANGE_W::new(self)
    }
    #[doc = "Bits 4:5 - Start-up Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn csuthf(&mut self) -> CSUTHF_W<4> {
        CSUTHF_W::new(self)
    }
    #[doc = "Bit 7 - Run Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstby(&mut self) -> RUNSTBY_W<7> {
        RUNSTBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XOSCHF Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoschfctrla](index.html) module"]
pub struct XOSCHFCTRLA_SPEC;
impl crate::RegisterSpec for XOSCHFCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xoschfctrla::R](R) reader structure"]
impl crate::Readable for XOSCHFCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoschfctrla::W](W) writer structure"]
impl crate::Writable for XOSCHFCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSCHFCTRLA to value 0"]
impl crate::Resettable for XOSCHFCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
