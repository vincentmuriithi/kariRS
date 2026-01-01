#[doc = "Register `VREGCTRL` reader"]
pub struct R(crate::R<VREGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGCTRL` writer"]
pub struct W(crate::W<VREGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGCTRL_SPEC>;
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
impl From<crate::W<VREGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMODE` reader - Performance Mode"]
pub type PMODE_R = crate::FieldReader<u8, PMODE_A>;
#[doc = "Performance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMODE_A {
    #[doc = "0: No Description."]
    AUTO = 0,
    #[doc = "1: No Description."]
    FULL = 1,
}
impl From<PMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as _
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMODE_A> {
        match self.bits {
            0 => Some(PMODE_A::AUTO),
            1 => Some(PMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == PMODE_A::AUTO
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PMODE_A::FULL
    }
}
#[doc = "Field `PMODE` writer - Performance Mode"]
pub type PMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, VREGCTRL_SPEC, u8, PMODE_A, 3, O>;
impl<'a, const O: u8> PMODE_W<'a, O> {
    #[doc = "No Description."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(PMODE_A::AUTO)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(PMODE_A::FULL)
    }
}
#[doc = "Field `HTLLEN` reader - High Temperature Low Leakage Enable"]
pub type HTLLEN_R = crate::BitReader<HTLLEN_A>;
#[doc = "High Temperature Low Leakage Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTLLEN_A {
    #[doc = "0: Disabled"]
    OFF = 0,
    #[doc = "1: Enabled"]
    ON = 1,
}
impl From<HTLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: HTLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HTLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTLLEN_A {
        match self.bits {
            false => HTLLEN_A::OFF,
            true => HTLLEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HTLLEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HTLLEN_A::ON
    }
}
#[doc = "Field `HTLLEN` writer - High Temperature Low Leakage Enable"]
pub type HTLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, VREGCTRL_SPEC, HTLLEN_A, O>;
impl<'a, const O: u8> HTLLEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HTLLEN_A::OFF)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HTLLEN_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:2 - Performance Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(self.bits & 7)
    }
    #[doc = "Bit 4 - High Temperature Low Leakage Enable"]
    #[inline(always)]
    pub fn htllen(&self) -> HTLLEN_R {
        HTLLEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Performance Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<0> {
        PMODE_W::new(self)
    }
    #[doc = "Bit 4 - High Temperature Low Leakage Enable"]
    #[inline(always)]
    #[must_use]
    pub fn htllen(&mut self) -> HTLLEN_W<4> {
        HTLLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregctrl](index.html) module"]
pub struct VREGCTRL_SPEC;
impl crate::RegisterSpec for VREGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vregctrl::R](R) reader structure"]
impl crate::Readable for VREGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vregctrl::W](W) writer structure"]
impl crate::Writable for VREGCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGCTRL to value 0"]
impl crate::Resettable for VREGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
