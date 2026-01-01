#[doc = "Register `PART_NUM` reader"]
pub struct R(crate::R<PART_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PART_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PART_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PART_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PART_NUM` writer"]
pub struct W(crate::W<PART_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PART_NUM_SPEC>;
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
impl From<crate::W<PART_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PART_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PART_NUM` reader - Part Number"]
pub type PART_NUM_R = crate::FieldReader<u8, PART_NUM_A>;
#[doc = "Part Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PART_NUM_A {
    #[doc = "131: ATmega128RFA1 part number"]
    P_ATMEGA128RFA1 = 131,
}
impl From<PART_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: PART_NUM_A) -> Self {
        variant as _
    }
}
impl PART_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PART_NUM_A> {
        match self.bits {
            131 => Some(PART_NUM_A::P_ATMEGA128RFA1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P_ATMEGA128RFA1`"]
    #[inline(always)]
    pub fn is_p_atmega128rfa1(&self) -> bool {
        *self == PART_NUM_A::P_ATMEGA128RFA1
    }
}
#[doc = "Field `PART_NUM` writer - Part Number"]
pub type PART_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PART_NUM_SPEC, u8, PART_NUM_A, 8, O>;
impl<'a, const O: u8> PART_NUM_W<'a, O> {
    #[doc = "ATmega128RFA1 part number"]
    #[inline(always)]
    pub fn p_atmega128rfa1(self) -> &'a mut W {
        self.variant(PART_NUM_A::P_ATMEGA128RFA1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn part_num(&self) -> PART_NUM_R {
        PART_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    #[must_use]
    pub fn part_num(&mut self) -> PART_NUM_W<0> {
        PART_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Identification Register (Part Number)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [part_num](index.html) module"]
pub struct PART_NUM_SPEC;
impl crate::RegisterSpec for PART_NUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [part_num::R](R) reader structure"]
impl crate::Readable for PART_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [part_num::W](W) writer structure"]
impl crate::Writable for PART_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PART_NUM to value 0"]
impl crate::Resettable for PART_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
