#[doc = "Register `VLMCTRLA` reader"]
pub struct R(crate::R<VLMCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLMCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLMCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLMCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLMCTRLA` writer"]
pub struct W(crate::W<VLMCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLMCTRLA_SPEC>;
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
impl From<crate::W<VLMCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLMCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLMLVL` reader - voltage level monitor level"]
pub type VLMLVL_R = crate::FieldReader<u8, VLMLVL_A>;
#[doc = "voltage level monitor level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VLMLVL_A {
    #[doc = "0: VLM Disabled"]
    OFF = 0,
    #[doc = "1: VLM threshold 5% above BOD level"]
    _5ABOVE = 1,
    #[doc = "2: VLM threshold 15% above BOD level"]
    _15ABOVE = 2,
    #[doc = "3: VLM threshold 25% above BOD level"]
    _25ABOVE = 3,
}
impl From<VLMLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: VLMLVL_A) -> Self {
        variant as _
    }
}
impl VLMLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLMLVL_A {
        match self.bits {
            0 => VLMLVL_A::OFF,
            1 => VLMLVL_A::_5ABOVE,
            2 => VLMLVL_A::_15ABOVE,
            3 => VLMLVL_A::_25ABOVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == VLMLVL_A::OFF
    }
    #[doc = "Checks if the value of the field is `_5ABOVE`"]
    #[inline(always)]
    pub fn is_5above(&self) -> bool {
        *self == VLMLVL_A::_5ABOVE
    }
    #[doc = "Checks if the value of the field is `_15ABOVE`"]
    #[inline(always)]
    pub fn is_15above(&self) -> bool {
        *self == VLMLVL_A::_15ABOVE
    }
    #[doc = "Checks if the value of the field is `_25ABOVE`"]
    #[inline(always)]
    pub fn is_25above(&self) -> bool {
        *self == VLMLVL_A::_25ABOVE
    }
}
#[doc = "Field `VLMLVL` writer - voltage level monitor level"]
pub type VLMLVL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, VLMCTRLA_SPEC, u8, VLMLVL_A, 2, O>;
impl<'a, const O: u8> VLMLVL_W<'a, O> {
    #[doc = "VLM Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(VLMLVL_A::OFF)
    }
    #[doc = "VLM threshold 5% above BOD level"]
    #[inline(always)]
    pub fn _5above(self) -> &'a mut W {
        self.variant(VLMLVL_A::_5ABOVE)
    }
    #[doc = "VLM threshold 15% above BOD level"]
    #[inline(always)]
    pub fn _15above(self) -> &'a mut W {
        self.variant(VLMLVL_A::_15ABOVE)
    }
    #[doc = "VLM threshold 25% above BOD level"]
    #[inline(always)]
    pub fn _25above(self) -> &'a mut W {
        self.variant(VLMLVL_A::_25ABOVE)
    }
}
impl R {
    #[doc = "Bits 0:1 - voltage level monitor level"]
    #[inline(always)]
    pub fn vlmlvl(&self) -> VLMLVL_R {
        VLMLVL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - voltage level monitor level"]
    #[inline(always)]
    #[must_use]
    pub fn vlmlvl(&mut self) -> VLMLVL_W<0> {
        VLMLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage level monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlmctrla](index.html) module"]
pub struct VLMCTRLA_SPEC;
impl crate::RegisterSpec for VLMCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vlmctrla::R](R) reader structure"]
impl crate::Readable for VLMCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlmctrla::W](W) writer structure"]
impl crate::Writable for VLMCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLMCTRLA to value 0"]
impl crate::Resettable for VLMCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
