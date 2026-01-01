#[doc = "Register `RX_SYN` reader"]
pub struct R(crate::R<RX_SYN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SYN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SYN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SYN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_SYN` writer"]
pub struct W(crate::W<RX_SYN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SYN_SPEC>;
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
impl From<crate::W<RX_SYN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SYN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_PDT_LEVEL` reader - Reduce Receiver Sensitivity"]
pub type RX_PDT_LEVEL_R = crate::FieldReader<u8, RX_PDT_LEVEL_A>;
#[doc = "Reduce Receiver Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_PDT_LEVEL_A {
    #[doc = "0: RX_THRES ≤ RSSI_BASE_VAL (Reset value); RSSI value not considered"]
    RX_PDT_LEVEL_MIN = 0,
    #[doc = "1: RX_THRES > RSSI_BASE_VAL + 0 · 3; RSSI > -90 dBm"]
    VAL_0X1 = 1,
    #[doc = "2: ..."]
    VAL_0X2 = 2,
    #[doc = "14: RX_THRES > RSSI_BASE_VAL + 13 · 3; RSSI > -51 dBm"]
    VAL_0X_E = 14,
    #[doc = "15: RX_THRES > RSSI_BASE_VAL + 14 · 3; RSSI > -48 dBm"]
    RX_PDT_LEVEL_MAX = 15,
}
impl From<RX_PDT_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PDT_LEVEL_A) -> Self {
        variant as _
    }
}
impl RX_PDT_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_PDT_LEVEL_A> {
        match self.bits {
            0 => Some(RX_PDT_LEVEL_A::RX_PDT_LEVEL_MIN),
            1 => Some(RX_PDT_LEVEL_A::VAL_0X1),
            2 => Some(RX_PDT_LEVEL_A::VAL_0X2),
            14 => Some(RX_PDT_LEVEL_A::VAL_0X_E),
            15 => Some(RX_PDT_LEVEL_A::RX_PDT_LEVEL_MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RX_PDT_LEVEL_MIN`"]
    #[inline(always)]
    pub fn is_rx_pdt_level_min(&self) -> bool {
        *self == RX_PDT_LEVEL_A::RX_PDT_LEVEL_MIN
    }
    #[doc = "Checks if the value of the field is `VAL_0X1`"]
    #[inline(always)]
    pub fn is_val_0x1(&self) -> bool {
        *self == RX_PDT_LEVEL_A::VAL_0X1
    }
    #[doc = "Checks if the value of the field is `VAL_0X2`"]
    #[inline(always)]
    pub fn is_val_0x2(&self) -> bool {
        *self == RX_PDT_LEVEL_A::VAL_0X2
    }
    #[doc = "Checks if the value of the field is `VAL_0X_E`"]
    #[inline(always)]
    pub fn is_val_0x_e(&self) -> bool {
        *self == RX_PDT_LEVEL_A::VAL_0X_E
    }
    #[doc = "Checks if the value of the field is `RX_PDT_LEVEL_MAX`"]
    #[inline(always)]
    pub fn is_rx_pdt_level_max(&self) -> bool {
        *self == RX_PDT_LEVEL_A::RX_PDT_LEVEL_MAX
    }
}
#[doc = "Field `RX_PDT_LEVEL` writer - Reduce Receiver Sensitivity"]
pub type RX_PDT_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, RX_SYN_SPEC, u8, RX_PDT_LEVEL_A, 4, O>;
impl<'a, const O: u8> RX_PDT_LEVEL_W<'a, O> {
    #[doc = "RX_THRES ≤ RSSI_BASE_VAL (Reset value); RSSI value not considered"]
    #[inline(always)]
    pub fn rx_pdt_level_min(self) -> &'a mut W {
        self.variant(RX_PDT_LEVEL_A::RX_PDT_LEVEL_MIN)
    }
    #[doc = "RX_THRES > RSSI_BASE_VAL + 0 · 3; RSSI > -90 dBm"]
    #[inline(always)]
    pub fn val_0x1(self) -> &'a mut W {
        self.variant(RX_PDT_LEVEL_A::VAL_0X1)
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn val_0x2(self) -> &'a mut W {
        self.variant(RX_PDT_LEVEL_A::VAL_0X2)
    }
    #[doc = "RX_THRES > RSSI_BASE_VAL + 13 · 3; RSSI > -51 dBm"]
    #[inline(always)]
    pub fn val_0x_e(self) -> &'a mut W {
        self.variant(RX_PDT_LEVEL_A::VAL_0X_E)
    }
    #[doc = "RX_THRES > RSSI_BASE_VAL + 14 · 3; RSSI > -48 dBm"]
    #[inline(always)]
    pub fn rx_pdt_level_max(self) -> &'a mut W {
        self.variant(RX_PDT_LEVEL_A::RX_PDT_LEVEL_MAX)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RX_SYN_SPEC, u8, u8, 3, O>;
#[doc = "Field `RX_PDT_DIS` reader - Prevent Frame Reception"]
pub type RX_PDT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `RX_PDT_DIS` writer - Prevent Frame Reception"]
pub type RX_PDT_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, RX_SYN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Reduce Receiver Sensitivity"]
    #[inline(always)]
    pub fn rx_pdt_level(&self) -> RX_PDT_LEVEL_R {
        RX_PDT_LEVEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Prevent Frame Reception"]
    #[inline(always)]
    pub fn rx_pdt_dis(&self) -> RX_PDT_DIS_R {
        RX_PDT_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reduce Receiver Sensitivity"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdt_level(&mut self) -> RX_PDT_LEVEL_W<0> {
        RX_PDT_LEVEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<4> {
        RES_W::new(self)
    }
    #[doc = "Bit 7 - Prevent Frame Reception"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdt_dis(&mut self) -> RX_PDT_DIS_W<7> {
        RX_PDT_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Receiver Sensitivity Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_syn](index.html) module"]
pub struct RX_SYN_SPEC;
impl crate::RegisterSpec for RX_SYN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rx_syn::R](R) reader structure"]
impl crate::Readable for RX_SYN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_syn::W](W) writer structure"]
impl crate::Writable for RX_SYN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_SYN to value 0"]
impl crate::Resettable for RX_SYN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
