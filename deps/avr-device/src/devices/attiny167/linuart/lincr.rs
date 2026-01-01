#[doc = "Register `LINCR` reader"]
pub struct R(crate::R<LINCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINCR` writer"]
pub struct W(crate::W<LINCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINCR_SPEC>;
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
impl From<crate::W<LINCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCMD` reader - LIN Command and Mode bits"]
pub type LCMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCMD` writer - LIN Command and Mode bits"]
pub type LCMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `LENA` reader - LIN or UART Enable"]
pub type LENA_R = crate::BitReader<bool>;
#[doc = "Field `LENA` writer - LIN or UART Enable"]
pub type LENA_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINCR_SPEC, bool, O>;
#[doc = "Field `LCONF` reader - LIN Configuration bits"]
pub type LCONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCONF` writer - LIN Configuration bits"]
pub type LCONF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LINCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LIN13` reader - LIN Standard"]
pub type LIN13_R = crate::BitReader<bool>;
#[doc = "Field `LIN13` writer - LIN Standard"]
pub type LIN13_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINCR_SPEC, bool, O>;
#[doc = "Field `LSWRES` reader - Software Reset"]
pub type LSWRES_R = crate::BitReader<bool>;
#[doc = "Field `LSWRES` writer - Software Reset"]
pub type LSWRES_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - LIN Command and Mode bits"]
    #[inline(always)]
    pub fn lcmd(&self) -> LCMD_R {
        LCMD_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - LIN or UART Enable"]
    #[inline(always)]
    pub fn lena(&self) -> LENA_R {
        LENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LIN Configuration bits"]
    #[inline(always)]
    pub fn lconf(&self) -> LCONF_R {
        LCONF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - LIN Standard"]
    #[inline(always)]
    pub fn lin13(&self) -> LIN13_R {
        LIN13_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn lswres(&self) -> LSWRES_R {
        LSWRES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LIN Command and Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn lcmd(&mut self) -> LCMD_W<0> {
        LCMD_W::new(self)
    }
    #[doc = "Bit 3 - LIN or UART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lena(&mut self) -> LENA_W<3> {
        LENA_W::new(self)
    }
    #[doc = "Bits 4:5 - LIN Configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn lconf(&mut self) -> LCONF_W<4> {
        LCONF_W::new(self)
    }
    #[doc = "Bit 6 - LIN Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lin13(&mut self) -> LIN13_W<6> {
        LIN13_W::new(self)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lswres(&mut self) -> LSWRES_W<7> {
        LSWRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lincr](index.html) module"]
pub struct LINCR_SPEC;
impl crate::RegisterSpec for LINCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lincr::R](R) reader structure"]
impl crate::Readable for LINCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lincr::W](W) writer structure"]
impl crate::Writable for LINCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINCR to value 0"]
impl crate::Resettable for LINCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
