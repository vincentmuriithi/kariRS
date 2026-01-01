#[doc = "Register `MCTRLA` reader"]
pub struct R(crate::R<MCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRLA` writer"]
pub struct W(crate::W<MCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRLA_SPEC>;
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
impl From<crate::W<MCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - Inactive Bus Time-Out"]
pub type TIMEOUT_R = crate::FieldReader<u8, TIMEOUT_A>;
#[doc = "Inactive Bus Time-Out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Bus time-out disabled. I2C."]
    DISABLED = 0,
    #[doc = "1: 50us - SMBus"]
    _50US = 1,
    #[doc = "2: 100us"]
    _100US = 2,
    #[doc = "3: 200us"]
    _200US = 3,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            0 => TIMEOUT_A::DISABLED,
            1 => TIMEOUT_A::_50US,
            2 => TIMEOUT_A::_100US,
            3 => TIMEOUT_A::_200US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMEOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_50US`"]
    #[inline(always)]
    pub fn is_50us(&self) -> bool {
        *self == TIMEOUT_A::_50US
    }
    #[doc = "Checks if the value of the field is `_100US`"]
    #[inline(always)]
    pub fn is_100us(&self) -> bool {
        *self == TIMEOUT_A::_100US
    }
    #[doc = "Checks if the value of the field is `_200US`"]
    #[inline(always)]
    pub fn is_200us(&self) -> bool {
        *self == TIMEOUT_A::_200US
    }
}
#[doc = "Field `TIMEOUT` writer - Inactive Bus Time-Out"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, MCTRLA_SPEC, u8, TIMEOUT_A, 2, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "Bus time-out disabled. I2C."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMEOUT_A::DISABLED)
    }
    #[doc = "50us - SMBus"]
    #[inline(always)]
    pub fn _50us(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_50US)
    }
    #[doc = "100us"]
    #[inline(always)]
    pub fn _100us(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_100US)
    }
    #[doc = "200us"]
    #[inline(always)]
    pub fn _200us(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_200US)
    }
}
#[doc = "Field `QCEN` reader - Quick Command Enable"]
pub type QCEN_R = crate::BitReader<bool>;
#[doc = "Field `QCEN` writer - Quick Command Enable"]
pub type QCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `WIEN` reader - Write Interrupt Enable"]
pub type WIEN_R = crate::BitReader<bool>;
#[doc = "Field `WIEN` writer - Write Interrupt Enable"]
pub type WIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `RIEN` reader - Read Interrupt Enable"]
pub type RIEN_R = crate::BitReader<bool>;
#[doc = "Field `RIEN` writer - Read Interrupt Enable"]
pub type RIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Inactive Bus Time-Out"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Quick Command Enable"]
    #[inline(always)]
    pub fn qcen(&self) -> QCEN_R {
        QCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Interrupt Enable"]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Inactive Bus Time-Out"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<2> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - Quick Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qcen(&mut self) -> QCEN_W<4> {
        QCEN_W::new(self)
    }
    #[doc = "Bit 6 - Write Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wien(&mut self) -> WIEN_W<6> {
        WIEN_W::new(self)
    }
    #[doc = "Bit 7 - Read Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<7> {
        RIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrla](index.html) module"]
pub struct MCTRLA_SPEC;
impl crate::RegisterSpec for MCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mctrla::R](R) reader structure"]
impl crate::Readable for MCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrla::W](W) writer structure"]
impl crate::Writable for MCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRLA to value 0"]
impl crate::Resettable for MCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
