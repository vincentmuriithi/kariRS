#[doc = "Register `DIDR0` reader"]
pub struct R(crate::R<DIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIDR0` writer"]
pub struct W(crate::W<DIDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIDR0_SPEC>;
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
impl From<crate::W<DIDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIN0D` reader - AIN0 Digital Input Disable"]
pub type AIN0D_R = crate::BitReader<bool>;
#[doc = "Field `AIN0D` writer - AIN0 Digital Input Disable"]
pub type AIN0D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
#[doc = "Field `AIN1D` reader - AIN1 Digital Input Disable"]
pub type AIN1D_R = crate::BitReader<bool>;
#[doc = "Field `AIN1D` writer - AIN1 Digital Input Disable"]
pub type AIN1D_W<'a, const O: u8> = crate::BitWriter<'a, u8, DIDR0_SPEC, bool, O>;
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
#[doc = "Digital Input Disable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [didr0](index.html) module"]
pub struct DIDR0_SPEC;
impl crate::RegisterSpec for DIDR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [didr0::R](R) reader structure"]
impl crate::Readable for DIDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [didr0::W](W) writer structure"]
impl crate::Writable for DIDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR0 to value 0"]
impl crate::Resettable for DIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
