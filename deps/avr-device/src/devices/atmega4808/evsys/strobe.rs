#[doc = "Register `STROBE` writer"]
pub struct W(crate::W<STROBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STROBE_SPEC>;
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
impl From<crate::W<STROBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STROBE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software event on channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STROBE0_AW {
    #[doc = "1: No Description."]
    EV_STROBE_CH0 = 1,
    #[doc = "2: No Description."]
    EV_STROBE_CH1 = 2,
    #[doc = "4: No Description."]
    EV_STROBE_CH2 = 4,
    #[doc = "8: No Description."]
    EV_STROBE_CH3 = 8,
    #[doc = "16: No Description."]
    EV_STROBE_CH4 = 16,
    #[doc = "32: No Description."]
    EV_STROBE_CH5 = 32,
    #[doc = "64: No Description."]
    EV_STROBE_CH6 = 64,
    #[doc = "128: No Description."]
    EV_STROBE_CH7 = 128,
}
impl From<STROBE0_AW> for u8 {
    #[inline(always)]
    fn from(variant: STROBE0_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `STROBE0` writer - Software event on channels"]
pub type STROBE0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, STROBE_SPEC, u8, STROBE0_AW, 8, O>;
impl<'a, const O: u8> STROBE0_W<'a, O> {
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch0(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH0)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch1(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH1)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch2(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH2)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch3(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH3)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch4(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH4)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch5(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH5)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch6(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH6)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch7(self) -> &'a mut W {
        self.variant(STROBE0_AW::EV_STROBE_CH7)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software event on channels"]
    #[inline(always)]
    #[must_use]
    pub fn strobe0(&mut self) -> STROBE0_W<0> {
        STROBE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Strobe\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strobe](index.html) module"]
pub struct STROBE_SPEC;
impl crate::RegisterSpec for STROBE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [strobe::W](W) writer structure"]
impl crate::Writable for STROBE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STROBE to value 0"]
impl crate::Resettable for STROBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
