#[doc = "Register `DLYCTRL` reader"]
pub struct R(crate::R<DLYCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYCTRL` writer"]
pub struct W(crate::W<DLYCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYCTRL_SPEC>;
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
impl From<crate::W<DLYCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYSEL` reader - Delay select"]
pub type DLYSEL_R = crate::FieldReader<u8, DLYSEL_A>;
#[doc = "Delay select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYSEL_A {
    #[doc = "0: No delay"]
    OFF = 0,
    #[doc = "1: Input blanking enabled"]
    INBLANK = 1,
    #[doc = "2: Event delay enabled"]
    EVENT = 2,
}
impl From<DLYSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYSEL_A) -> Self {
        variant as _
    }
}
impl DLYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DLYSEL_A> {
        match self.bits {
            0 => Some(DLYSEL_A::OFF),
            1 => Some(DLYSEL_A::INBLANK),
            2 => Some(DLYSEL_A::EVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DLYSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `INBLANK`"]
    #[inline(always)]
    pub fn is_inblank(&self) -> bool {
        *self == DLYSEL_A::INBLANK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == DLYSEL_A::EVENT
    }
}
#[doc = "Field `DLYSEL` writer - Delay select"]
pub type DLYSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DLYCTRL_SPEC, u8, DLYSEL_A, 2, O>;
impl<'a, const O: u8> DLYSEL_W<'a, O> {
    #[doc = "No delay"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DLYSEL_A::OFF)
    }
    #[doc = "Input blanking enabled"]
    #[inline(always)]
    pub fn inblank(self) -> &'a mut W {
        self.variant(DLYSEL_A::INBLANK)
    }
    #[doc = "Event delay enabled"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(DLYSEL_A::EVENT)
    }
}
#[doc = "Field `DLYTRIG` reader - Delay trigger"]
pub type DLYTRIG_R = crate::FieldReader<u8, DLYTRIG_A>;
#[doc = "Delay trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYTRIG_A {
    #[doc = "0: Compare A set"]
    CMPASET = 0,
    #[doc = "1: Compare A clear"]
    CMPACLR = 1,
    #[doc = "2: Compare B set"]
    CMPBSET = 2,
    #[doc = "3: Compare B clear"]
    CMPBCLR = 3,
}
impl From<DLYTRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYTRIG_A) -> Self {
        variant as _
    }
}
impl DLYTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYTRIG_A {
        match self.bits {
            0 => DLYTRIG_A::CMPASET,
            1 => DLYTRIG_A::CMPACLR,
            2 => DLYTRIG_A::CMPBSET,
            3 => DLYTRIG_A::CMPBCLR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPASET`"]
    #[inline(always)]
    pub fn is_cmpaset(&self) -> bool {
        *self == DLYTRIG_A::CMPASET
    }
    #[doc = "Checks if the value of the field is `CMPACLR`"]
    #[inline(always)]
    pub fn is_cmpaclr(&self) -> bool {
        *self == DLYTRIG_A::CMPACLR
    }
    #[doc = "Checks if the value of the field is `CMPBSET`"]
    #[inline(always)]
    pub fn is_cmpbset(&self) -> bool {
        *self == DLYTRIG_A::CMPBSET
    }
    #[doc = "Checks if the value of the field is `CMPBCLR`"]
    #[inline(always)]
    pub fn is_cmpbclr(&self) -> bool {
        *self == DLYTRIG_A::CMPBCLR
    }
}
#[doc = "Field `DLYTRIG` writer - Delay trigger"]
pub type DLYTRIG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DLYCTRL_SPEC, u8, DLYTRIG_A, 2, O>;
impl<'a, const O: u8> DLYTRIG_W<'a, O> {
    #[doc = "Compare A set"]
    #[inline(always)]
    pub fn cmpaset(self) -> &'a mut W {
        self.variant(DLYTRIG_A::CMPASET)
    }
    #[doc = "Compare A clear"]
    #[inline(always)]
    pub fn cmpaclr(self) -> &'a mut W {
        self.variant(DLYTRIG_A::CMPACLR)
    }
    #[doc = "Compare B set"]
    #[inline(always)]
    pub fn cmpbset(self) -> &'a mut W {
        self.variant(DLYTRIG_A::CMPBSET)
    }
    #[doc = "Compare B clear"]
    #[inline(always)]
    pub fn cmpbclr(self) -> &'a mut W {
        self.variant(DLYTRIG_A::CMPBCLR)
    }
}
#[doc = "Field `DLYPRESC` reader - Delay prescaler"]
pub type DLYPRESC_R = crate::FieldReader<u8, DLYPRESC_A>;
#[doc = "Delay prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYPRESC_A {
    #[doc = "0: No prescaling"]
    DIV1 = 0,
    #[doc = "1: Prescale with 2"]
    DIV2 = 1,
    #[doc = "2: Prescale with 4"]
    DIV4 = 2,
    #[doc = "3: Prescale with 8"]
    DIV8 = 3,
}
impl From<DLYPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYPRESC_A) -> Self {
        variant as _
    }
}
impl DLYPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRESC_A {
        match self.bits {
            0 => DLYPRESC_A::DIV1,
            1 => DLYPRESC_A::DIV2,
            2 => DLYPRESC_A::DIV4,
            3 => DLYPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DLYPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DLYPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DLYPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DLYPRESC_A::DIV8
    }
}
#[doc = "Field `DLYPRESC` writer - Delay prescaler"]
pub type DLYPRESC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, DLYCTRL_SPEC, u8, DLYPRESC_A, 2, O>;
impl<'a, const O: u8> DLYPRESC_W<'a, O> {
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DLYPRESC_A::DIV1)
    }
    #[doc = "Prescale with 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DLYPRESC_A::DIV2)
    }
    #[doc = "Prescale with 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DLYPRESC_A::DIV4)
    }
    #[doc = "Prescale with 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DLYPRESC_A::DIV8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Delay select"]
    #[inline(always)]
    pub fn dlysel(&self) -> DLYSEL_R {
        DLYSEL_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Delay trigger"]
    #[inline(always)]
    pub fn dlytrig(&self) -> DLYTRIG_R {
        DLYTRIG_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Delay prescaler"]
    #[inline(always)]
    pub fn dlypresc(&self) -> DLYPRESC_R {
        DLYPRESC_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Delay select"]
    #[inline(always)]
    #[must_use]
    pub fn dlysel(&mut self) -> DLYSEL_W<0> {
        DLYSEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Delay trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dlytrig(&mut self) -> DLYTRIG_W<2> {
        DLYTRIG_W::new(self)
    }
    #[doc = "Bits 4:5 - Delay prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dlypresc(&mut self) -> DLYPRESC_W<4> {
        DLYPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyctrl](index.html) module"]
pub struct DLYCTRL_SPEC;
impl crate::RegisterSpec for DLYCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dlyctrl::R](R) reader structure"]
impl crate::Readable for DLYCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyctrl::W](W) writer structure"]
impl crate::Writable for DLYCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYCTRL to value 0"]
impl crate::Resettable for DLYCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
