#[doc = "Register `RAMPZ` reader"]
pub struct R(crate::R<RAMPZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMPZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMPZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMPZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMPZ` writer"]
pub struct W(crate::W<RAMPZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMPZ_SPEC>;
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
impl From<crate::W<RAMPZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMPZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMPZ0` reader - RAM Page Z Select Register Bit 0"]
pub type RAMPZ0_R = crate::BitReader<bool>;
#[doc = "Field `RAMPZ0` writer - RAM Page Z Select Register Bit 0"]
pub type RAMPZ0_W<'a, const O: u8> = crate::BitWriter<'a, u8, RAMPZ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RAM Page Z Select Register Bit 0"]
    #[inline(always)]
    pub fn rampz0(&self) -> RAMPZ0_R {
        RAMPZ0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Page Z Select Register Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rampz0(&mut self) -> RAMPZ0_W<0> {
        RAMPZ0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Page Z Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rampz](index.html) module"]
pub struct RAMPZ_SPEC;
impl crate::RegisterSpec for RAMPZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rampz::R](R) reader structure"]
impl crate::Readable for RAMPZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rampz::W](W) writer structure"]
impl crate::Writable for RAMPZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMPZ to value 0"]
impl crate::Resettable for RAMPZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
