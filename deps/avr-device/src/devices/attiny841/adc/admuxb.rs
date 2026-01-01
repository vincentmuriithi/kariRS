#[doc = "Register `ADMUXB` reader"]
pub struct R(crate::R<ADMUXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMUXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMUXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMUXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADMUXB` writer"]
pub struct W(crate::W<ADMUXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADMUXB_SPEC>;
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
impl From<crate::W<ADMUXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADMUXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GSEL` reader - Gain Selection Bits"]
pub type GSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GSEL` writer - Gain Selection Bits"]
pub type GSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUXB_SPEC, u8, u8, 2, O>;
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<u8, REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: Vcc"]
    VCC = 0,
    #[doc = "1: Internal 1.1V Voltage Reference with AREF disconnected"]
    INTERNAL_1 = 1,
    #[doc = "2: Internal 2.2V Voltage Reference with AREF disconnected"]
    INTERNAL_2 = 2,
    #[doc = "3: Internal 4.096V Voltage Reference with AREF disconnected"]
    INTERNAL_4 = 3,
    #[doc = "4: AREF with internal reference off"]
    AREF = 4,
    #[doc = "5: Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    AREF_INTERNAL_1 = 5,
    #[doc = "6: Internal 2.2V Voltage Reference with external capacitor at AREF pin"]
    AREF_INTERNAL_2 = 6,
    #[doc = "7: Internal 4.096V Voltage Reference with external capacitor at AREF pin"]
    AREF_INTERNAL_4 = 7,
}
impl From<REFS_A> for u8 {
    #[inline(always)]
    fn from(variant: REFS_A) -> Self {
        variant as _
    }
}
impl REFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFS_A {
        match self.bits {
            0 => REFS_A::VCC,
            1 => REFS_A::INTERNAL_1,
            2 => REFS_A::INTERNAL_2,
            3 => REFS_A::INTERNAL_4,
            4 => REFS_A::AREF,
            5 => REFS_A::AREF_INTERNAL_1,
            6 => REFS_A::AREF_INTERNAL_2,
            7 => REFS_A::AREF_INTERNAL_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VCC`"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS_A::VCC
    }
    #[doc = "Checks if the value of the field is `INTERNAL_1`"]
    #[inline(always)]
    pub fn is_internal_1(&self) -> bool {
        *self == REFS_A::INTERNAL_1
    }
    #[doc = "Checks if the value of the field is `INTERNAL_2`"]
    #[inline(always)]
    pub fn is_internal_2(&self) -> bool {
        *self == REFS_A::INTERNAL_2
    }
    #[doc = "Checks if the value of the field is `INTERNAL_4`"]
    #[inline(always)]
    pub fn is_internal_4(&self) -> bool {
        *self == REFS_A::INTERNAL_4
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFS_A::AREF
    }
    #[doc = "Checks if the value of the field is `AREF_INTERNAL_1`"]
    #[inline(always)]
    pub fn is_aref_internal_1(&self) -> bool {
        *self == REFS_A::AREF_INTERNAL_1
    }
    #[doc = "Checks if the value of the field is `AREF_INTERNAL_2`"]
    #[inline(always)]
    pub fn is_aref_internal_2(&self) -> bool {
        *self == REFS_A::AREF_INTERNAL_2
    }
    #[doc = "Checks if the value of the field is `AREF_INTERNAL_4`"]
    #[inline(always)]
    pub fn is_aref_internal_4(&self) -> bool {
        *self == REFS_A::AREF_INTERNAL_4
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ADMUXB_SPEC, u8, REFS_A, 3, O>;
impl<'a, const O: u8> REFS_W<'a, O> {
    #[doc = "Vcc"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut W {
        self.variant(REFS_A::VCC)
    }
    #[doc = "Internal 1.1V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn internal_1(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL_1)
    }
    #[doc = "Internal 2.2V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn internal_2(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL_2)
    }
    #[doc = "Internal 4.096V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn internal_4(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL_4)
    }
    #[doc = "AREF with internal reference off"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFS_A::AREF)
    }
    #[doc = "Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn aref_internal_1(self) -> &'a mut W {
        self.variant(REFS_A::AREF_INTERNAL_1)
    }
    #[doc = "Internal 2.2V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn aref_internal_2(self) -> &'a mut W {
        self.variant(REFS_A::AREF_INTERNAL_2)
    }
    #[doc = "Internal 4.096V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn aref_internal_4(self) -> &'a mut W {
        self.variant(REFS_A::AREF_INTERNAL_4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Gain Selection Bits"]
    #[inline(always)]
    pub fn gsel(&self) -> GSEL_R {
        GSEL_R::new(self.bits & 3)
    }
    #[doc = "Bits 5:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn gsel(&mut self) -> GSEL_W<0> {
        GSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - Reference Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<5> {
        REFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC multiplexer Selection Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admuxb](index.html) module"]
pub struct ADMUXB_SPEC;
impl crate::RegisterSpec for ADMUXB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [admuxb::R](R) reader structure"]
impl crate::Readable for ADMUXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [admuxb::W](W) writer structure"]
impl crate::Writable for ADMUXB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUXB to value 0"]
impl crate::Resettable for ADMUXB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
