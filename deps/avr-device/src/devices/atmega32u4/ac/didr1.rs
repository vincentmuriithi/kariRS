#[doc = "Register `DIDR1` reader"]
pub struct R(crate::R<DIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR1` writer"]
pub struct W(crate::W<DIDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR1_SPEC>;
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
impl From<crate::W<DIDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIN0D` reader - AIN0 Digital Input Disable"]
pub type AIN0D_R = crate::BitReader<bool>;
#[doc = "Field `AIN0D` writer - AIN0 Digital Input Disable"]
pub type AIN0D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR1_SPEC, bool, O>;
#[doc = "Field `AIN1D` reader - AIN1 Digital Input Disable"]
pub type AIN1D_R = crate::BitReader<bool>;
#[doc = "Field `AIN1D` writer - AIN1 Digital Input Disable"]
pub type AIN1D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AIN0 Digital Input Disable"]
    #[inline(always)]
    pub fn ain0d(&self) -> AIN0D_R {
        AIN0D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AIN1 Digital Input Disable"]
    #[inline(always)]
    pub fn ain1d(&self) -> AIN1D_R {
        AIN1D_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AIN0 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain0d(&mut self) -> AIN0D_W<0> {
        AIN0D_W::new(self)
    }
    #[doc = "Bit 1 - AIN1 Digital Input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ain1d(&mut self) -> AIN1D_W<1> {
        AIN1D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr1](index.html) module"]
pub struct DIDR1_SPEC;
impl crate::RegisterSpec for DIDR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [didr1::R](R) reader structure"]
impl crate::Readable for DIDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr1::W](W) writer structure"]
impl crate::Writable for DIDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR1 to value 0"]
impl crate::Resettable for DIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
