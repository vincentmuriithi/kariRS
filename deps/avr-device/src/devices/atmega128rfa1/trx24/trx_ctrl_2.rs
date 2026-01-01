#[doc = "Register `TRX_CTRL_2` reader"]
pub struct R(crate::R<TRX_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRX_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRX_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRX_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRX_CTRL_2` writer"]
pub struct W(crate::W<TRX_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRX_CTRL_2_SPEC>;
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
impl From<crate::W<TRX_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRX_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OQPSK_DATA_RATE` reader - Data Rate Selection"]
pub type OQPSK_DATA_RATE_R = crate::FieldReader<u8, OQPSK_DATA_RATE_A>;
#[doc = "Data Rate Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OQPSK_DATA_RATE_A {
    #[doc = "0: 250 kb/s (IEEE 802.15.4 compliant)"]
    RATE_250KB = 0,
    #[doc = "1: 500 kb/s"]
    RATE_500KB = 1,
    #[doc = "2: 1000 kb/s"]
    RATE_1000KB = 2,
    #[doc = "3: 2000 kb/s"]
    RATE_2000KB = 3,
}
impl From<OQPSK_DATA_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: OQPSK_DATA_RATE_A) -> Self {
        variant as _
    }
}
impl OQPSK_DATA_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OQPSK_DATA_RATE_A {
        match self.bits {
            0 => OQPSK_DATA_RATE_A::RATE_250KB,
            1 => OQPSK_DATA_RATE_A::RATE_500KB,
            2 => OQPSK_DATA_RATE_A::RATE_1000KB,
            3 => OQPSK_DATA_RATE_A::RATE_2000KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATE_250KB`"]
    #[inline(always)]
    pub fn is_rate_250kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_250KB
    }
    #[doc = "Checks if the value of the field is `RATE_500KB`"]
    #[inline(always)]
    pub fn is_rate_500kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_500KB
    }
    #[doc = "Checks if the value of the field is `RATE_1000KB`"]
    #[inline(always)]
    pub fn is_rate_1000kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_1000KB
    }
    #[doc = "Checks if the value of the field is `RATE_2000KB`"]
    #[inline(always)]
    pub fn is_rate_2000kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_2000KB
    }
}
#[doc = "Field `OQPSK_DATA_RATE` writer - Data Rate Selection"]
pub type OQPSK_DATA_RATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, TRX_CTRL_2_SPEC, u8, OQPSK_DATA_RATE_A, 2, O>;
impl<'a, const O: u8> OQPSK_DATA_RATE_W<'a, O> {
    #[doc = "250 kb/s (IEEE 802.15.4 compliant)"]
    #[inline(always)]
    pub fn rate_250kb(self) -> &'a mut W {
        self.variant(OQPSK_DATA_RATE_A::RATE_250KB)
    }
    #[doc = "500 kb/s"]
    #[inline(always)]
    pub fn rate_500kb(self) -> &'a mut W {
        self.variant(OQPSK_DATA_RATE_A::RATE_500KB)
    }
    #[doc = "1000 kb/s"]
    #[inline(always)]
    pub fn rate_1000kb(self) -> &'a mut W {
        self.variant(OQPSK_DATA_RATE_A::RATE_1000KB)
    }
    #[doc = "2000 kb/s"]
    #[inline(always)]
    pub fn rate_2000kb(self) -> &'a mut W {
        self.variant(OQPSK_DATA_RATE_A::RATE_2000KB)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TRX_CTRL_2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RX_SAFE_MODE` reader - RX Safe Mode"]
pub type RX_SAFE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `RX_SAFE_MODE` writer - RX Safe Mode"]
pub type RX_SAFE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TRX_CTRL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Data Rate Selection"]
    #[inline(always)]
    pub fn oqpsk_data_rate(&self) -> OQPSK_DATA_RATE_R {
        OQPSK_DATA_RATE_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x1f)
    }
    #[doc = "Bit 7 - RX Safe Mode"]
    #[inline(always)]
    pub fn rx_safe_mode(&self) -> RX_SAFE_MODE_R {
        RX_SAFE_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn oqpsk_data_rate(&mut self) -> OQPSK_DATA_RATE_W<0> {
        OQPSK_DATA_RATE_W::new(self)
    }
    #[doc = "Bits 2:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<2> {
        RES_W::new(self)
    }
    #[doc = "Bit 7 - RX Safe Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_safe_mode(&mut self) -> RX_SAFE_MODE_W<7> {
        RX_SAFE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_ctrl_2](index.html) module"]
pub struct TRX_CTRL_2_SPEC;
impl crate::RegisterSpec for TRX_CTRL_2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trx_ctrl_2::R](R) reader structure"]
impl crate::Readable for TRX_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trx_ctrl_2::W](W) writer structure"]
impl crate::Writable for TRX_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRX_CTRL_2 to value 0"]
impl crate::Resettable for TRX_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
