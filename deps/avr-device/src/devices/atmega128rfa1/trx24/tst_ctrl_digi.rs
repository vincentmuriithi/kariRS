#[doc = "Register `TST_CTRL_DIGI` reader"]
pub struct R(crate::R<TST_CTRL_DIGI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_CTRL_DIGI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_CTRL_DIGI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_CTRL_DIGI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST_CTRL_DIGI` writer"]
pub struct W(crate::W<TST_CTRL_DIGI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_CTRL_DIGI_SPEC>;
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
impl From<crate::W<TST_CTRL_DIGI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_CTRL_DIGI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TST_CTRL_DIG` reader - Digital Test Controller Register"]
pub type TST_CTRL_DIG_R = crate::FieldReader<u8, TST_CTRL_DIG_A>;
#[doc = "Digital Test Controller Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TST_CTRL_DIG_A {
    #[doc = "0: NORMAL (no test is active)"]
    NORMAL_NO_TEST_IS_ACTIVE = 0,
    #[doc = "15: TST_CONT_TX (continuous transmit)"]
    TST_CONT_TX_CONTINUOUS_TRANSMIT = 15,
}
impl From<TST_CTRL_DIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TST_CTRL_DIG_A) -> Self {
        variant as _
    }
}
impl TST_CTRL_DIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TST_CTRL_DIG_A> {
        match self.bits {
            0 => Some(TST_CTRL_DIG_A::NORMAL_NO_TEST_IS_ACTIVE),
            15 => Some(TST_CTRL_DIG_A::TST_CONT_TX_CONTINUOUS_TRANSMIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_NO_TEST_IS_ACTIVE`"]
    #[inline(always)]
    pub fn is_normal_no_test_is_active(&self) -> bool {
        *self == TST_CTRL_DIG_A::NORMAL_NO_TEST_IS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `TST_CONT_TX_CONTINUOUS_TRANSMIT`"]
    #[inline(always)]
    pub fn is_tst_cont_tx_continuous_transmit(&self) -> bool {
        *self == TST_CTRL_DIG_A::TST_CONT_TX_CONTINUOUS_TRANSMIT
    }
}
#[doc = "Field `TST_CTRL_DIG` writer - Digital Test Controller Register"]
pub type TST_CTRL_DIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, TST_CTRL_DIGI_SPEC, u8, TST_CTRL_DIG_A, 4, O>;
impl<'a, const O: u8> TST_CTRL_DIG_W<'a, O> {
    #[doc = "NORMAL (no test is active)"]
    #[inline(always)]
    pub fn normal_no_test_is_active(self) -> &'a mut W {
        self.variant(TST_CTRL_DIG_A::NORMAL_NO_TEST_IS_ACTIVE)
    }
    #[doc = "TST_CONT_TX (continuous transmit)"]
    #[inline(always)]
    pub fn tst_cont_tx_continuous_transmit(self) -> &'a mut W {
        self.variant(TST_CTRL_DIG_A::TST_CONT_TX_CONTINUOUS_TRANSMIT)
    }
}
impl R {
    #[doc = "Bits 0:3 - Digital Test Controller Register"]
    #[inline(always)]
    pub fn tst_ctrl_dig(&self) -> TST_CTRL_DIG_R {
        TST_CTRL_DIG_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital Test Controller Register"]
    #[inline(always)]
    #[must_use]
    pub fn tst_ctrl_dig(&mut self) -> TST_CTRL_DIG_W<0> {
        TST_CTRL_DIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Digital Test Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst_ctrl_digi](index.html) module"]
pub struct TST_CTRL_DIGI_SPEC;
impl crate::RegisterSpec for TST_CTRL_DIGI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tst_ctrl_digi::R](R) reader structure"]
impl crate::Readable for TST_CTRL_DIGI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst_ctrl_digi::W](W) writer structure"]
impl crate::Writable for TST_CTRL_DIGI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST_CTRL_DIGI to value 0"]
impl crate::Resettable for TST_CTRL_DIGI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
