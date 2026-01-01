#[doc = "Register `DTPS` reader"]
pub struct R(crate::R<DTPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTPS` writer"]
pub struct W(crate::W<DTPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTPS_SPEC>;
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
impl From<crate::W<DTPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTPS` reader - No Description."]
pub type DTPS_R = crate::FieldReader<u8, DTPS_A>;
#[doc = "No Description.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPS_A {
    #[doc = "0: No Prescaling"]
    DIRECT = 0,
    #[doc = "1: Division factor 2"]
    PRESCALE_2 = 1,
    #[doc = "2: Division factor 4"]
    PRESCALE_4 = 2,
    #[doc = "3: Division factor 8"]
    PRESCALE_8 = 3,
}
impl From<DTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS_A) -> Self {
        variant as _
    }
}
impl DTPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTPS_A {
        match self.bits {
            0 => DTPS_A::DIRECT,
            1 => DTPS_A::PRESCALE_2,
            2 => DTPS_A::PRESCALE_4,
            3 => DTPS_A::PRESCALE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == DTPS_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == DTPS_A::PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PRESCALE_4`"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == DTPS_A::PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == DTPS_A::PRESCALE_8
    }
}
#[doc = "Field `DTPS` writer - No Description."]
pub type DTPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DTPS_SPEC, u8, DTPS_A, 2, O>;
impl<'a, const O: u8> DTPS_W<'a, O> {
    #[doc = "No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(DTPS_A::DIRECT)
    }
    #[doc = "Division factor 2"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(DTPS_A::PRESCALE_2)
    }
    #[doc = "Division factor 4"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(DTPS_A::PRESCALE_4)
    }
    #[doc = "Division factor 8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(DTPS_A::PRESCALE_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    pub fn dtps(&self) -> DTPS_R {
        DTPS_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtps(&mut self) -> DTPS_W<0> {
        DTPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dead time prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtps](index.html) module"]
pub struct DTPS_SPEC;
impl crate::RegisterSpec for DTPS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dtps::R](R) reader structure"]
impl crate::Readable for DTPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtps::W](W) writer structure"]
impl crate::Writable for DTPS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTPS to value 0"]
impl crate::Resettable for DTPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
