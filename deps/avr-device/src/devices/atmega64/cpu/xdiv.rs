#[doc = "Register `XDIV` reader"]
pub struct R(crate::R<XDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDIV` writer"]
pub struct W(crate::W<XDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDIV_SPEC>;
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
impl From<crate::W<XDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XDIV` reader - XTAl Divide Select Bits"]
pub type XDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XDIV` writer - XTAl Divide Select Bits"]
pub type XDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, XDIV_SPEC, u8, u8, 7, O>;
#[doc = "Field `XDIVEN` reader - XTAL Divide Enable"]
pub type XDIVEN_R = crate::BitReader<bool>;
#[doc = "Field `XDIVEN` writer - XTAL Divide Enable"]
pub type XDIVEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, XDIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - XTAl Divide Select Bits"]
    #[inline(always)]
    pub fn xdiv(&self) -> XDIV_R {
        XDIV_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - XTAL Divide Enable"]
    #[inline(always)]
    pub fn xdiven(&self) -> XDIVEN_R {
        XDIVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - XTAl Divide Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn xdiv(&mut self) -> XDIV_W<0> {
        XDIV_W::new(self)
    }
    #[doc = "Bit 7 - XTAL Divide Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xdiven(&mut self) -> XDIVEN_W<7> {
        XDIVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL Divide Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdiv](index.html) module"]
pub struct XDIV_SPEC;
impl crate::RegisterSpec for XDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xdiv::R](R) reader structure"]
impl crate::Readable for XDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdiv::W](W) writer structure"]
impl crate::Writable for XDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XDIV to value 0"]
impl crate::Resettable for XDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
