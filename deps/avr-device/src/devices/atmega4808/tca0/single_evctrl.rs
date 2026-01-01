#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<SINGLE_EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<SINGLE_EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLE_EVCTRL_SPEC>;
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
impl From<crate::W<SINGLE_EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLE_EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTEI` reader - Count on Event Input"]
pub type CNTEI_R = crate::BitReader<bool>;
#[doc = "Field `CNTEI` writer - Count on Event Input"]
pub type CNTEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_EVCTRL_SPEC, bool, O>;
#[doc = "Field `EVACT` reader - Event Action"]
pub type EVACT_R = crate::FieldReader<u8, EVACT_A>;
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACT_A {
    #[doc = "0: Count on positive edge event"]
    POSEDGE = 0,
    #[doc = "1: Count on any edge event"]
    ANYEDGE = 1,
    #[doc = "2: Count on prescaled clock while event line is 1."]
    HIGHLVL = 2,
    #[doc = "3: Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    UPDOWN = 3,
}
impl From<EVACT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT_A) -> Self {
        variant as _
    }
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT_A {
        match self.bits {
            0 => EVACT_A::POSEDGE,
            1 => EVACT_A::ANYEDGE,
            2 => EVACT_A::HIGHLVL,
            3 => EVACT_A::UPDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EVACT_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `ANYEDGE`"]
    #[inline(always)]
    pub fn is_anyedge(&self) -> bool {
        *self == EVACT_A::ANYEDGE
    }
    #[doc = "Checks if the value of the field is `HIGHLVL`"]
    #[inline(always)]
    pub fn is_highlvl(&self) -> bool {
        *self == EVACT_A::HIGHLVL
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == EVACT_A::UPDOWN
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub type EVACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, SINGLE_EVCTRL_SPEC, u8, EVACT_A, 2, O>;
impl<'a, const O: u8> EVACT_W<'a, O> {
    #[doc = "Count on positive edge event"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EVACT_A::POSEDGE)
    }
    #[doc = "Count on any edge event"]
    #[inline(always)]
    pub fn anyedge(self) -> &'a mut W {
        self.variant(EVACT_A::ANYEDGE)
    }
    #[doc = "Count on prescaled clock while event line is 1."]
    #[inline(always)]
    pub fn highlvl(self) -> &'a mut W {
        self.variant(EVACT_A::HIGHLVL)
    }
    #[doc = "Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1."]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(EVACT_A::UPDOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Count on Event Input"]
    #[inline(always)]
    pub fn cntei(&self) -> CNTEI_R {
        CNTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Count on Event Input"]
    #[inline(always)]
    #[must_use]
    pub fn cntei(&mut self) -> CNTEI_W<0> {
        CNTEI_W::new(self)
    }
    #[doc = "Bits 1:2 - Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<1> {
        EVACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_evctrl](index.html) module"]
pub struct SINGLE_EVCTRL_SPEC;
impl crate::RegisterSpec for SINGLE_EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [single_evctrl::R](R) reader structure"]
impl crate::Readable for SINGLE_EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_evctrl::W](W) writer structure"]
impl crate::Writable for SINGLE_EVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for SINGLE_EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
