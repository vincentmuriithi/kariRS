#[doc = "Register `ACMUX` reader"]
pub struct R(crate::R<ACMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMUX` writer"]
pub struct W(crate::W<ACMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMUX_SPEC>;
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
impl From<crate::W<ACMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMUX` reader - Analog Comparator Selection Bits"]
pub type CMUX_R = crate::FieldReader<u8, CMUX_A>;
#[doc = "Analog Comparator Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMUX_A {
    #[doc = "0: AIN1"]
    AIN1 = 0,
    #[doc = "1: AIN2"]
    AIN2 = 1,
    #[doc = "2: AIN3"]
    AIN3 = 2,
    #[doc = "3: AIN4"]
    AIN4 = 3,
    #[doc = "4: AIN5"]
    AIN5 = 4,
    #[doc = "5: AIN6"]
    AIN6 = 5,
}
impl From<CMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: CMUX_A) -> Self {
        variant as _
    }
}
impl CMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMUX_A> {
        match self.bits {
            0 => Some(CMUX_A::AIN1),
            1 => Some(CMUX_A::AIN2),
            2 => Some(CMUX_A::AIN3),
            3 => Some(CMUX_A::AIN4),
            4 => Some(CMUX_A::AIN5),
            5 => Some(CMUX_A::AIN6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == CMUX_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == CMUX_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == CMUX_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == CMUX_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == CMUX_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == CMUX_A::AIN6
    }
}
#[doc = "Field `CMUX` writer - Analog Comparator Selection Bits"]
pub type CMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACMUX_SPEC, u8, CMUX_A, 3, O>;
impl<'a, const O: u8> CMUX_W<'a, O> {
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(CMUX_A::AIN1)
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(CMUX_A::AIN2)
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(CMUX_A::AIN3)
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(CMUX_A::AIN4)
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(CMUX_A::AIN5)
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(CMUX_A::AIN6)
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog Comparator Selection Bits"]
    #[inline(always)]
    pub fn cmux(&self) -> CMUX_R {
        CMUX_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog Comparator Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cmux(&mut self) -> CMUX_W<0> {
        CMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Input Multiplexer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmux](index.html) module"]
pub struct ACMUX_SPEC;
impl crate::RegisterSpec for ACMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acmux::R](R) reader structure"]
impl crate::Readable for ACMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmux::W](W) writer structure"]
impl crate::Writable for ACMUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMUX to value 0"]
impl crate::Resettable for ACMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
