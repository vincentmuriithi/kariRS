#[doc = "Register `ETIMSK` reader"]
pub struct R(crate::R<ETIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETIMSK` writer"]
pub struct W(crate::W<ETIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETIMSK_SPEC>;
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
impl From<crate::W<ETIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCIE1C` reader - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
pub type OCIE1C_R = crate::BitReader<bool>;
#[doc = "Field `OCIE1C` writer - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
pub type OCIE1C_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETIMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1c(&self) -> OCIE1C_R {
        OCIE1C_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1c(&mut self) -> OCIE1C_W<0> {
        OCIE1C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Timer/Counter Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etimsk](index.html) module"]
pub struct ETIMSK_SPEC;
impl crate::RegisterSpec for ETIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [etimsk::R](R) reader structure"]
impl crate::Readable for ETIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etimsk::W](W) writer structure"]
impl crate::Writable for ETIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETIMSK to value 0"]
impl crate::Resettable for ETIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
