#[doc = "Register `PINE` reader"]
pub struct R(crate::R<PINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINE` writer"]
pub struct W(crate::W<PINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINE_SPEC>;
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
impl From<crate::W<PINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE2` reader - Pin E2"]
pub type PE2_R = crate::BitReader<bool>;
#[doc = "Field `PE2` writer - Pin E2"]
pub type PE2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINE_SPEC, bool, O>;
#[doc = "Field `PE6` reader - Pin E6"]
pub type PE6_R = crate::BitReader<bool>;
#[doc = "Field `PE6` writer - Pin E6"]
pub type PE6_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    pub fn pe6(&self) -> PE6_R {
        PE6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<2> {
        PE2_W::new(self)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    #[must_use]
    pub fn pe6(&mut self) -> PE6_W<6> {
        PE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Pins, Port E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pine](index.html) module"]
pub struct PINE_SPEC;
impl crate::RegisterSpec for PINE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pine::R](R) reader structure"]
impl crate::Readable for PINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pine::W](W) writer structure"]
impl crate::Writable for PINE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINE to value 0"]
impl crate::Resettable for PINE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
