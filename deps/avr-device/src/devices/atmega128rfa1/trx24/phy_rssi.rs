#[doc = "Register `PHY_RSSI` reader"]
pub struct R(crate::R<PHY_RSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_RSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_RSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_RSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_RSSI` writer"]
pub struct W(crate::W<PHY_RSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_RSSI_SPEC>;
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
impl From<crate::W<PHY_RSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_RSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSI` reader - Receiver Signal Strength Indicator"]
pub type RSSI_R = crate::FieldReader<u8, RSSI_A>;
#[doc = "Receiver Signal Strength Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSSI_A {
    #[doc = "0: Minimum RSSI value: P(RF) < -90 dBm"]
    RSSI_MIN = 0,
    #[doc = "1: P(RF) = RSSI_BASE_VAL+3 · (RSSI-1) \\[dBm\\]"]
    RSSI_MIN_PLUS_3D_B = 1,
    #[doc = "2: ..."]
    VAL_2 = 2,
    #[doc = "28: Maximum RSSI value: P(RF) ≥ -10 dBm"]
    RSSI_MAX = 28,
}
impl From<RSSI_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_A) -> Self {
        variant as _
    }
}
impl RSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSSI_A> {
        match self.bits {
            0 => Some(RSSI_A::RSSI_MIN),
            1 => Some(RSSI_A::RSSI_MIN_PLUS_3D_B),
            2 => Some(RSSI_A::VAL_2),
            28 => Some(RSSI_A::RSSI_MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_MIN`"]
    #[inline(always)]
    pub fn is_rssi_min(&self) -> bool {
        *self == RSSI_A::RSSI_MIN
    }
    #[doc = "Checks if the value of the field is `RSSI_MIN_PLUS_3D_B`"]
    #[inline(always)]
    pub fn is_rssi_min_plus_3d_b(&self) -> bool {
        *self == RSSI_A::RSSI_MIN_PLUS_3D_B
    }
    #[doc = "Checks if the value of the field is `VAL_2`"]
    #[inline(always)]
    pub fn is_val_2(&self) -> bool {
        *self == RSSI_A::VAL_2
    }
    #[doc = "Checks if the value of the field is `RSSI_MAX`"]
    #[inline(always)]
    pub fn is_rssi_max(&self) -> bool {
        *self == RSSI_A::RSSI_MAX
    }
}
#[doc = "Field `RSSI` writer - Receiver Signal Strength Indicator"]
pub type RSSI_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PHY_RSSI_SPEC, u8, RSSI_A, 5, O>;
impl<'a, const O: u8> RSSI_W<'a, O> {
    #[doc = "Minimum RSSI value: P(RF) < -90 dBm"]
    #[inline(always)]
    pub fn rssi_min(self) -> &'a mut W {
        self.variant(RSSI_A::RSSI_MIN)
    }
    #[doc = "P(RF) = RSSI_BASE_VAL+3 · (RSSI-1) \\[dBm\\]"]
    #[inline(always)]
    pub fn rssi_min_plus_3d_b(self) -> &'a mut W {
        self.variant(RSSI_A::RSSI_MIN_PLUS_3D_B)
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn val_2(self) -> &'a mut W {
        self.variant(RSSI_A::VAL_2)
    }
    #[doc = "Maximum RSSI value: P(RF) ≥ -10 dBm"]
    #[inline(always)]
    pub fn rssi_max(self) -> &'a mut W {
        self.variant(RSSI_A::RSSI_MAX)
    }
}
#[doc = "Field `RND_VALUE` reader - Random Value"]
pub type RND_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RND_VALUE` writer - Random Value"]
pub type RND_VALUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PHY_RSSI_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_CRC_VALID` reader - Received Frame CRC Status"]
pub type RX_CRC_VALID_R = crate::BitReader<RX_CRC_VALID_A>;
#[doc = "Received Frame CRC Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_CRC_VALID_A {
    #[doc = "0: CRC (FCS) not valid"]
    CRC_INVALID = 0,
    #[doc = "1: CRC (FCS) valid"]
    CRC_VALID = 1,
}
impl From<RX_CRC_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CRC_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_CRC_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CRC_VALID_A {
        match self.bits {
            false => RX_CRC_VALID_A::CRC_INVALID,
            true => RX_CRC_VALID_A::CRC_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_INVALID`"]
    #[inline(always)]
    pub fn is_crc_invalid(&self) -> bool {
        *self == RX_CRC_VALID_A::CRC_INVALID
    }
    #[doc = "Checks if the value of the field is `CRC_VALID`"]
    #[inline(always)]
    pub fn is_crc_valid(&self) -> bool {
        *self == RX_CRC_VALID_A::CRC_VALID
    }
}
#[doc = "Field `RX_CRC_VALID` writer - Received Frame CRC Status"]
pub type RX_CRC_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, PHY_RSSI_SPEC, RX_CRC_VALID_A, O>;
impl<'a, const O: u8> RX_CRC_VALID_W<'a, O> {
    #[doc = "CRC (FCS) not valid"]
    #[inline(always)]
    pub fn crc_invalid(self) -> &'a mut W {
        self.variant(RX_CRC_VALID_A::CRC_INVALID)
    }
    #[doc = "CRC (FCS) valid"]
    #[inline(always)]
    pub fn crc_valid(self) -> &'a mut W {
        self.variant(RX_CRC_VALID_A::CRC_VALID)
    }
}
impl R {
    #[doc = "Bits 0:4 - Receiver Signal Strength Indicator"]
    #[inline(always)]
    pub fn rssi(&self) -> RSSI_R {
        RSSI_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:6 - Random Value"]
    #[inline(always)]
    pub fn rnd_value(&self) -> RND_VALUE_R {
        RND_VALUE_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Received Frame CRC Status"]
    #[inline(always)]
    pub fn rx_crc_valid(&self) -> RX_CRC_VALID_R {
        RX_CRC_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receiver Signal Strength Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn rssi(&mut self) -> RSSI_W<0> {
        RSSI_W::new(self)
    }
    #[doc = "Bits 5:6 - Random Value"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_value(&mut self) -> RND_VALUE_W<5> {
        RND_VALUE_W::new(self)
    }
    #[doc = "Bit 7 - Received Frame CRC Status"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_valid(&mut self) -> RX_CRC_VALID_W<7> {
        RX_CRC_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Signal Strength Indicator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_rssi](index.html) module"]
pub struct PHY_RSSI_SPEC;
impl crate::RegisterSpec for PHY_RSSI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [phy_rssi::R](R) reader structure"]
impl crate::Readable for PHY_RSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_rssi::W](W) writer structure"]
impl crate::Writable for PHY_RSSI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_RSSI to value 0"]
impl crate::Resettable for PHY_RSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
