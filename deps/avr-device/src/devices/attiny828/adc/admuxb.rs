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
#[doc = "Field `MUX5` reader - Analog Channel Selection Bit 5"]
pub type MUX5_R = crate::BitReader<bool>;
#[doc = "Field `MUX5` writer - Analog Channel Selection Bit 5"]
pub type MUX5_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADMUXB_SPEC, bool, O>;
#[doc = "Field `REFS` reader - Reference Selection Bit"]
pub type REFS_R = crate::BitReader<REFS_A>;
#[doc = "Reference Selection Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFS_A {
    #[doc = "0: Vcc used as analog reference"]
    VCC = 0,
    #[doc = "1: Internal 1.1V Voltage Reference"]
    INTERNAL = 1,
}
impl From<REFS_A> for bool {
    #[inline(always)]
    fn from(variant: REFS_A) -> Self {
        variant as u8 != 0
    }
}
impl REFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFS_A {
        match self.bits {
            false => REFS_A::VCC,
            true => REFS_A::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `VCC`"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS_A::VCC
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS_A::INTERNAL
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bit"]
pub type REFS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADMUXB_SPEC, REFS_A, O>;
impl<'a, const O: u8> REFS_W<'a, O> {
    #[doc = "Vcc used as analog reference"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut W {
        self.variant(REFS_A::VCC)
    }
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFS_A::INTERNAL)
    }
}
impl R {
    #[doc = "Bit 0 - Analog Channel Selection Bit 5"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Reference Selection Bit"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Channel Selection Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn mux5(&mut self) -> MUX5_W<0> {
        MUX5_W::new(self)
    }
    #[doc = "Bit 5 - Reference Selection Bit"]
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
