#[doc = "Register `XAH_CTRL_1` reader"]
pub struct R(crate::R<XAH_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XAH_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XAH_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XAH_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XAH_CTRL_1` writer"]
pub struct W(crate::W<XAH_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XAH_CTRL_1_SPEC>;
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
impl From<crate::W<XAH_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XAH_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AACK_PROM_MODE` reader - Enable Promiscuous Mode"]
pub type AACK_PROM_MODE_R = crate::BitReader<bool>;
#[doc = "Field `AACK_PROM_MODE` writer - Enable Promiscuous Mode"]
pub type AACK_PROM_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, XAH_CTRL_1_SPEC, bool, O>;
#[doc = "Field `AACK_ACK_TIME` reader - Reduce Acknowledgment Time"]
pub type AACK_ACK_TIME_R = crate::BitReader<AACK_ACK_TIME_A>;
#[doc = "Reduce Acknowledgment Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AACK_ACK_TIME_A {
    #[doc = "0: 12 symbols acknowledgment time"]
    AACK_ACK_TIME_12_SYM = 0,
    #[doc = "1: 2 symbols acknowledgment time"]
    AACK_ACK_TIME_2_SYM = 1,
}
impl From<AACK_ACK_TIME_A> for bool {
    #[inline(always)]
    fn from(variant: AACK_ACK_TIME_A) -> Self {
        variant as u8 != 0
    }
}
impl AACK_ACK_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AACK_ACK_TIME_A {
        match self.bits {
            false => AACK_ACK_TIME_A::AACK_ACK_TIME_12_SYM,
            true => AACK_ACK_TIME_A::AACK_ACK_TIME_2_SYM,
        }
    }
    #[doc = "Checks if the value of the field is `AACK_ACK_TIME_12_SYM`"]
    #[inline(always)]
    pub fn is_aack_ack_time_12_sym(&self) -> bool {
        *self == AACK_ACK_TIME_A::AACK_ACK_TIME_12_SYM
    }
    #[doc = "Checks if the value of the field is `AACK_ACK_TIME_2_SYM`"]
    #[inline(always)]
    pub fn is_aack_ack_time_2_sym(&self) -> bool {
        *self == AACK_ACK_TIME_A::AACK_ACK_TIME_2_SYM
    }
}
#[doc = "Field `AACK_ACK_TIME` writer - Reduce Acknowledgment Time"]
pub type AACK_ACK_TIME_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, XAH_CTRL_1_SPEC, AACK_ACK_TIME_A, O>;
impl<'a, const O: u8> AACK_ACK_TIME_W<'a, O> {
    #[doc = "12 symbols acknowledgment time"]
    #[inline(always)]
    pub fn aack_ack_time_12_sym(self) -> &'a mut W {
        self.variant(AACK_ACK_TIME_A::AACK_ACK_TIME_12_SYM)
    }
    #[doc = "2 symbols acknowledgment time"]
    #[inline(always)]
    pub fn aack_ack_time_2_sym(self) -> &'a mut W {
        self.variant(AACK_ACK_TIME_A::AACK_ACK_TIME_2_SYM)
    }
}
#[doc = "Field `AACK_UPLD_RES_FT` reader - Process Reserved Frames"]
pub type AACK_UPLD_RES_FT_R = crate::BitReader<bool>;
#[doc = "Field `AACK_UPLD_RES_FT` writer - Process Reserved Frames"]
pub type AACK_UPLD_RES_FT_W<'a, const O: u8> = crate::BitWriter<'a, u8, XAH_CTRL_1_SPEC, bool, O>;
#[doc = "Field `AACK_FLTR_RES_FT` reader - Filter Reserved Frames"]
pub type AACK_FLTR_RES_FT_R = crate::BitReader<bool>;
#[doc = "Field `AACK_FLTR_RES_FT` writer - Filter Reserved Frames"]
pub type AACK_FLTR_RES_FT_W<'a, const O: u8> = crate::BitWriter<'a, u8, XAH_CTRL_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Enable Promiscuous Mode"]
    #[inline(always)]
    pub fn aack_prom_mode(&self) -> AACK_PROM_MODE_R {
        AACK_PROM_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reduce Acknowledgment Time"]
    #[inline(always)]
    pub fn aack_ack_time(&self) -> AACK_ACK_TIME_R {
        AACK_ACK_TIME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Process Reserved Frames"]
    #[inline(always)]
    pub fn aack_upld_res_ft(&self) -> AACK_UPLD_RES_FT_R {
        AACK_UPLD_RES_FT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter Reserved Frames"]
    #[inline(always)]
    pub fn aack_fltr_res_ft(&self) -> AACK_FLTR_RES_FT_R {
        AACK_FLTR_RES_FT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aack_prom_mode(&mut self) -> AACK_PROM_MODE_W<1> {
        AACK_PROM_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Reduce Acknowledgment Time"]
    #[inline(always)]
    #[must_use]
    pub fn aack_ack_time(&mut self) -> AACK_ACK_TIME_W<2> {
        AACK_ACK_TIME_W::new(self)
    }
    #[doc = "Bit 4 - Process Reserved Frames"]
    #[inline(always)]
    #[must_use]
    pub fn aack_upld_res_ft(&mut self) -> AACK_UPLD_RES_FT_W<4> {
        AACK_UPLD_RES_FT_W::new(self)
    }
    #[doc = "Bit 5 - Filter Reserved Frames"]
    #[inline(always)]
    #[must_use]
    pub fn aack_fltr_res_ft(&mut self) -> AACK_FLTR_RES_FT_W<5> {
        AACK_FLTR_RES_FT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Acknowledgment Frame Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xah_ctrl_1](index.html) module"]
pub struct XAH_CTRL_1_SPEC;
impl crate::RegisterSpec for XAH_CTRL_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xah_ctrl_1::R](R) reader structure"]
impl crate::Readable for XAH_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xah_ctrl_1::W](W) writer structure"]
impl crate::Writable for XAH_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XAH_CTRL_1 to value 0"]
impl crate::Resettable for XAH_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
