#[doc = "Register `EIMSK` reader"]
pub struct R(crate::R<EIMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMSK` writer"]
pub struct W(crate::W<EIMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMSK_SPEC>;
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
impl From<crate::W<EIMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0` reader - External Interrupt 0 Request Enable"]
pub type INT0_R = crate::BitReader<bool>;
#[doc = "Field `INT0` writer - External Interrupt 0 Request Enable"]
pub type INT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIMSK_SPEC, bool, O>;
#[doc = "Field `INT1` reader - External Interrupt 1 Request Enable"]
pub type INT1_R = crate::BitReader<bool>;
#[doc = "Field `INT1` writer - External Interrupt 1 Request Enable"]
pub type INT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIMSK_SPEC, bool, O>;
#[doc = "Field `INT2` reader - External Interrupt 2 Request Enable"]
pub type INT2_R = crate::BitReader<bool>;
#[doc = "Field `INT2` writer - External Interrupt 2 Request Enable"]
pub type INT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, EIMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Request Enable"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Request Enable"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Request Enable"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<0> {
        INT0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1 Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<1> {
        INT1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2 Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<2> {
        INT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimsk](index.html) module"]
pub struct EIMSK_SPEC;
impl crate::RegisterSpec for EIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eimsk::R](R) reader structure"]
impl crate::Readable for EIMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimsk::W](W) writer structure"]
impl crate::Writable for EIMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMSK to value 0"]
impl crate::Resettable for EIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
