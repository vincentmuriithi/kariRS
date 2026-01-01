#[doc = "Register `TRXPR` reader"]
pub struct R(crate::R<TRXPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRXPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRXPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRXPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRXPR` writer"]
pub struct W(crate::W<TRXPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRXPR_SPEC>;
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
impl From<crate::W<TRXPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRXPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRXRST` reader - Force Transceiver Reset"]
pub type TRXRST_R = crate::BitReader<bool>;
#[doc = "Field `TRXRST` writer - Force Transceiver Reset"]
pub type TRXRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, TRXPR_SPEC, bool, O>;
#[doc = "Field `SLPTR` reader - Multi-purpose Transceiver Control Bit"]
pub type SLPTR_R = crate::BitReader<bool>;
#[doc = "Field `SLPTR` writer - Multi-purpose Transceiver Control Bit"]
pub type SLPTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, TRXPR_SPEC, bool, O>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TRXPR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Force Transceiver Reset"]
    #[inline(always)]
    pub fn trxrst(&self) -> TRXRST_R {
        TRXRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multi-purpose Transceiver Control Bit"]
    #[inline(always)]
    pub fn slptr(&self) -> SLPTR_R {
        SLPTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Force Transceiver Reset"]
    #[inline(always)]
    #[must_use]
    pub fn trxrst(&mut self) -> TRXRST_W<0> {
        TRXRST_W::new(self)
    }
    #[doc = "Bit 1 - Multi-purpose Transceiver Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn slptr(&mut self) -> SLPTR_W<1> {
        SLPTR_W::new(self)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<4> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Pin Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trxpr](index.html) module"]
pub struct TRXPR_SPEC;
impl crate::RegisterSpec for TRXPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trxpr::R](R) reader structure"]
impl crate::Readable for TRXPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trxpr::W](W) writer structure"]
impl crate::Writable for TRXPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRXPR to value 0"]
impl crate::Resettable for TRXPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
