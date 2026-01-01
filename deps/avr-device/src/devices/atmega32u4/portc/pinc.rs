#[doc = "Register `PINC` reader"]
pub struct R(crate::R<PINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINC` writer"]
pub struct W(crate::W<PINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINC_SPEC>;
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
impl From<crate::W<PINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC6` reader - Pin C6"]
pub type PC6_R = crate::BitReader<bool>;
#[doc = "Field `PC6` writer - Pin C6"]
pub type PC6_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINC_SPEC, bool, O>;
#[doc = "Field `PC7` reader - Pin C7"]
pub type PC7_R = crate::BitReader<bool>;
#[doc = "Field `PC7` writer - Pin C7"]
pub type PC7_W<'a, const O: u8> = crate::BitWriter<'a, u8, PINC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<6> {
        PC6_W::new(self)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<7> {
        PC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Input Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinc](index.html) module"]
pub struct PINC_SPEC;
impl crate::RegisterSpec for PINC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pinc::R](R) reader structure"]
impl crate::Readable for PINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinc::W](W) writer structure"]
impl crate::Writable for PINC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINC to value 0"]
impl crate::Resettable for PINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
