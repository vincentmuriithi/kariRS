#[doc = "Register `MCUCR` reader"]
pub struct R(crate::R<MCUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCR` writer"]
pub struct W(crate::W<MCUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCR_SPEC>;
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
impl From<crate::W<MCUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC00` reader - Interrupt Sense Control 0 Bit 0"]
pub type ISC00_R = crate::BitReader<ISC00_A>;
#[doc = "Interrupt Sense Control 0 Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC00_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Reserved"]
    VAL_0X01 = 1,
}
impl From<ISC00_A> for bool {
    #[inline(always)]
    fn from(variant: ISC00_A) -> Self {
        variant as u8 != 0
    }
}
impl ISC00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISC00_A {
        match self.bits {
            false => ISC00_A::VAL_0X00,
            true => ISC00_A::VAL_0X01,
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC00_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC00_A::VAL_0X01
    }
}
#[doc = "Field `ISC00` writer - Interrupt Sense Control 0 Bit 0"]
pub type ISC00_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, ISC00_A, O>;
impl<'a, const O: u8> ISC00_W<'a, O> {
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(ISC00_A::VAL_0X00)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(ISC00_A::VAL_0X01)
    }
}
#[doc = "Field `ISC01` reader - Interrupt Sense Control 0 Bit 1"]
pub type ISC01_R = crate::BitReader<bool>;
#[doc = "Field `ISC01` writer - Interrupt Sense Control 0 Bit 1"]
pub type ISC01_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCUCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Sense Control 0 Bit 0"]
    #[inline(always)]
    pub fn isc00(&self) -> ISC00_R {
        ISC00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Sense Control 0 Bit 1"]
    #[inline(always)]
    pub fn isc01(&self) -> ISC01_R {
        ISC01_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Control 0 Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn isc00(&mut self) -> ISC00_W<0> {
        ISC00_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Sense Control 0 Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn isc01(&mut self) -> ISC01_W<1> {
        ISC01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucr](index.html) module"]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcucr::R](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucr::W](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
