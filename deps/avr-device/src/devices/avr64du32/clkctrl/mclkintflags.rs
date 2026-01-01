#[doc = "Register `MCLKINTFLAGS` reader"]
pub struct R(crate::R<MCLKINTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKINTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKINTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKINTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKINTFLAGS` writer"]
pub struct W(crate::W<MCLKINTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKINTFLAGS_SPEC>;
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
impl From<crate::W<MCLKINTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKINTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFD` reader - Clock Failure Detect Interrupt Flag"]
pub type CFD_R = crate::BitReader<bool>;
#[doc = "Field `CFD` writer - Clock Failure Detect Interrupt Flag"]
pub type CFD_W<'a, const O: u8> = crate::BitWriter<'a, u8, MCLKINTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Flag"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfd(&mut self) -> CFD_W<0> {
        CFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkintflags](index.html) module"]
pub struct MCLKINTFLAGS_SPEC;
impl crate::RegisterSpec for MCLKINTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclkintflags::R](R) reader structure"]
impl crate::Readable for MCLKINTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkintflags::W](W) writer structure"]
impl crate::Writable for MCLKINTFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKINTFLAGS to value 0"]
impl crate::Resettable for MCLKINTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
