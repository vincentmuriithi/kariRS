#[doc = "Register `DT4` reader"]
pub struct R(crate::R<DT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT4` writer"]
pub struct W(crate::W<DT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT4_SPEC>;
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
impl From<crate::W<DT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT4L` reader - Timer/Counter 4 Dead Time Value Bits"]
pub type DT4L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT4L` writer - Timer/Counter 4 Dead Time Value Bits"]
pub type DT4L_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DT4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter 4 Dead Time Value Bits"]
    #[inline(always)]
    pub fn dt4l(&self) -> DT4L_R {
        DT4L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter 4 Dead Time Value Bits"]
    #[inline(always)]
    #[must_use]
    pub fn dt4l(&mut self) -> DT4L_W<0> {
        DT4L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Timer/Counter 4 Dead Time Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt4](index.html) module"]
pub struct DT4_SPEC;
impl crate::RegisterSpec for DT4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dt4::R](R) reader structure"]
impl crate::Readable for DT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt4::W](W) writer structure"]
impl crate::Writable for DT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT4 to value 0"]
impl crate::Resettable for DT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
