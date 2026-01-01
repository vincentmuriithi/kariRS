#[doc = "Register `RX_CTRL` reader"]
pub struct R(crate::R<RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CTRL` writer"]
pub struct W(crate::W<RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CTRL_SPEC>;
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
impl From<crate::W<RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDT_THRES` reader - Receiver Sensitivity Control"]
pub type PDT_THRES_R = crate::FieldReader<u8, PDT_THRES_A>;
#[doc = "Receiver Sensitivity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDT_THRES_A {
    #[doc = "3: Recommended correlator threshold for Antenna Diversity operation"]
    PDT_THRES_ANT_DIV_ON = 3,
    #[doc = "7: Reset value, to be used if Antenna Diversity algorithm is disabled"]
    PDT_THRES_ANT_DIV_OFF = 7,
}
impl From<PDT_THRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PDT_THRES_A) -> Self {
        variant as _
    }
}
impl PDT_THRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDT_THRES_A> {
        match self.bits {
            3 => Some(PDT_THRES_A::PDT_THRES_ANT_DIV_ON),
            7 => Some(PDT_THRES_A::PDT_THRES_ANT_DIV_OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PDT_THRES_ANT_DIV_ON`"]
    #[inline(always)]
    pub fn is_pdt_thres_ant_div_on(&self) -> bool {
        *self == PDT_THRES_A::PDT_THRES_ANT_DIV_ON
    }
    #[doc = "Checks if the value of the field is `PDT_THRES_ANT_DIV_OFF`"]
    #[inline(always)]
    pub fn is_pdt_thres_ant_div_off(&self) -> bool {
        *self == PDT_THRES_A::PDT_THRES_ANT_DIV_OFF
    }
}
#[doc = "Field `PDT_THRES` writer - Receiver Sensitivity Control"]
pub type PDT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, RX_CTRL_SPEC, u8, PDT_THRES_A, 4, O>;
impl<'a, const O: u8> PDT_THRES_W<'a, O> {
    #[doc = "Recommended correlator threshold for Antenna Diversity operation"]
    #[inline(always)]
    pub fn pdt_thres_ant_div_on(self) -> &'a mut W {
        self.variant(PDT_THRES_A::PDT_THRES_ANT_DIV_ON)
    }
    #[doc = "Reset value, to be used if Antenna Diversity algorithm is disabled"]
    #[inline(always)]
    pub fn pdt_thres_ant_div_off(self) -> &'a mut W {
        self.variant(PDT_THRES_A::PDT_THRES_ANT_DIV_OFF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Receiver Sensitivity Control"]
    #[inline(always)]
    pub fn pdt_thres(&self) -> PDT_THRES_R {
        PDT_THRES_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receiver Sensitivity Control"]
    #[inline(always)]
    #[must_use]
    pub fn pdt_thres(&mut self) -> PDT_THRES_W<0> {
        PDT_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl](index.html) module"]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rx_ctrl::R](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ctrl::W](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CTRL to value 0"]
impl crate::Resettable for RX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
