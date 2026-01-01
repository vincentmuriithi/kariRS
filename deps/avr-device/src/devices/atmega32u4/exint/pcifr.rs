#[doc = "Register `PCIFR` reader"]
pub struct R(crate::R<PCIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCIFR` writer"]
pub struct W(crate::W<PCIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIFR_SPEC>;
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
impl From<crate::W<PCIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCIF0` reader - Pin Change Interrupt Flag 0"]
pub type PCIF0_R = crate::BitReader<bool>;
#[doc = "Field `PCIF0` writer - Pin Change Interrupt Flag 0"]
pub type PCIF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin Change Interrupt Flag 0"]
    #[inline(always)]
    pub fn pcif0(&self) -> PCIF0_R {
        PCIF0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Interrupt Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcif0(&mut self) -> PCIF0_W<0> {
        PCIF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Change Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcifr](index.html) module"]
pub struct PCIFR_SPEC;
impl crate::RegisterSpec for PCIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcifr::R](R) reader structure"]
impl crate::Readable for PCIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcifr::W](W) writer structure"]
impl crate::Writable for PCIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCIFR to value 0"]
impl crate::Resettable for PCIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
