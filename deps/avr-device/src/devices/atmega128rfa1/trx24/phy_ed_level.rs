#[doc = "Register `PHY_ED_LEVEL` reader"]
pub struct R(crate::R<PHY_ED_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_ED_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_ED_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_ED_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_ED_LEVEL` writer"]
pub struct W(crate::W<PHY_ED_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_ED_LEVEL_SPEC>;
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
impl From<crate::W<PHY_ED_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_ED_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ED_LEVEL` reader - Energy Detection Level"]
pub type ED_LEVEL_R = crate::FieldReader<u8, ED_LEVEL_A>;
#[doc = "Energy Detection Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ED_LEVEL_A {
    #[doc = "0: Minimum result of last ED measurement"]
    ED_MIN = 0,
    #[doc = "1: P(RF) = RSSI_BASE_VAL+ED \\[dBm\\]"]
    ED_MIN_PLUS_1D_B = 1,
    #[doc = "2: ..."]
    VAL_0X02 = 2,
    #[doc = "84: Maximum result of last ED measurement"]
    ED_MAX = 84,
    #[doc = "255: Reset value"]
    ED_RESET = 255,
}
impl From<ED_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ED_LEVEL_A) -> Self {
        variant as _
    }
}
impl ED_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ED_LEVEL_A> {
        match self.bits {
            0 => Some(ED_LEVEL_A::ED_MIN),
            1 => Some(ED_LEVEL_A::ED_MIN_PLUS_1D_B),
            2 => Some(ED_LEVEL_A::VAL_0X02),
            84 => Some(ED_LEVEL_A::ED_MAX),
            255 => Some(ED_LEVEL_A::ED_RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ED_MIN`"]
    #[inline(always)]
    pub fn is_ed_min(&self) -> bool {
        *self == ED_LEVEL_A::ED_MIN
    }
    #[doc = "Checks if the value of the field is `ED_MIN_PLUS_1D_B`"]
    #[inline(always)]
    pub fn is_ed_min_plus_1d_b(&self) -> bool {
        *self == ED_LEVEL_A::ED_MIN_PLUS_1D_B
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ED_LEVEL_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `ED_MAX`"]
    #[inline(always)]
    pub fn is_ed_max(&self) -> bool {
        *self == ED_LEVEL_A::ED_MAX
    }
    #[doc = "Checks if the value of the field is `ED_RESET`"]
    #[inline(always)]
    pub fn is_ed_reset(&self) -> bool {
        *self == ED_LEVEL_A::ED_RESET
    }
}
#[doc = "Field `ED_LEVEL` writer - Energy Detection Level"]
pub type ED_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PHY_ED_LEVEL_SPEC, u8, ED_LEVEL_A, 8, O>;
impl<'a, const O: u8> ED_LEVEL_W<'a, O> {
    #[doc = "Minimum result of last ED measurement"]
    #[inline(always)]
    pub fn ed_min(self) -> &'a mut W {
        self.variant(ED_LEVEL_A::ED_MIN)
    }
    #[doc = "P(RF) = RSSI_BASE_VAL+ED \\[dBm\\]"]
    #[inline(always)]
    pub fn ed_min_plus_1d_b(self) -> &'a mut W {
        self.variant(ED_LEVEL_A::ED_MIN_PLUS_1D_B)
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(ED_LEVEL_A::VAL_0X02)
    }
    #[doc = "Maximum result of last ED measurement"]
    #[inline(always)]
    pub fn ed_max(self) -> &'a mut W {
        self.variant(ED_LEVEL_A::ED_MAX)
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn ed_reset(self) -> &'a mut W {
        self.variant(ED_LEVEL_A::ED_RESET)
    }
}
impl R {
    #[doc = "Bits 0:7 - Energy Detection Level"]
    #[inline(always)]
    pub fn ed_level(&self) -> ED_LEVEL_R {
        ED_LEVEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Energy Detection Level"]
    #[inline(always)]
    #[must_use]
    pub fn ed_level(&mut self) -> ED_LEVEL_W<0> {
        ED_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Energy Detection Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_ed_level](index.html) module"]
pub struct PHY_ED_LEVEL_SPEC;
impl crate::RegisterSpec for PHY_ED_LEVEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [phy_ed_level::R](R) reader structure"]
impl crate::Readable for PHY_ED_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_ed_level::W](W) writer structure"]
impl crate::Writable for PHY_ED_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_ED_LEVEL to value 0"]
impl crate::Resettable for PHY_ED_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
