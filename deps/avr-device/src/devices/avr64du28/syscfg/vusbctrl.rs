#[doc = "Register `VUSBCTRL` reader"]
pub struct R(crate::R<VUSBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VUSBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VUSBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VUSBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VUSBCTRL` writer"]
pub struct W(crate::W<VUSBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VUSBCTRL_SPEC>;
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
impl From<crate::W<VUSBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VUSBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBVREG` reader - USB Voltage Regulator"]
pub type USBVREG_R = crate::BitReader<USBVREG_A>;
#[doc = "USB Voltage Regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBVREG_A {
    #[doc = "0: USBVREG is disabled"]
    DISABLE = 0,
    #[doc = "1: USBVREG is enabled"]
    ENABLE = 1,
}
impl From<USBVREG_A> for bool {
    #[inline(always)]
    fn from(variant: USBVREG_A) -> Self {
        variant as u8 != 0
    }
}
impl USBVREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBVREG_A {
        match self.bits {
            false => USBVREG_A::DISABLE,
            true => USBVREG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBVREG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USBVREG_A::ENABLE
    }
}
#[doc = "Field `USBVREG` writer - USB Voltage Regulator"]
pub type USBVREG_W<'a, const O: u8> = crate::BitWriter<'a, u8, VUSBCTRL_SPEC, USBVREG_A, O>;
impl<'a, const O: u8> USBVREG_W<'a, O> {
    #[doc = "USBVREG is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBVREG_A::DISABLE)
    }
    #[doc = "USBVREG is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBVREG_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - USB Voltage Regulator"]
    #[inline(always)]
    pub fn usbvreg(&self) -> USBVREG_R {
        USBVREG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Voltage Regulator"]
    #[inline(always)]
    #[must_use]
    pub fn usbvreg(&mut self) -> USBVREG_W<0> {
        USBVREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Voltage System Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vusbctrl](index.html) module"]
pub struct VUSBCTRL_SPEC;
impl crate::RegisterSpec for VUSBCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vusbctrl::R](R) reader structure"]
impl crate::Readable for VUSBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vusbctrl::W](W) writer structure"]
impl crate::Writable for VUSBCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VUSBCTRL to value 0"]
impl crate::Resettable for VUSBCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
