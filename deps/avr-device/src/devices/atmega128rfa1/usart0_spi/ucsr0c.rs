#[doc = "Register `UCSR0C` reader"]
pub struct R(crate::R<UCSR0C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSR0C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSR0C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSR0C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSR0C` writer"]
pub struct W(crate::W<UCSR0C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSR0C_SPEC>;
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
impl From<crate::W<UCSR0C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSR0C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCPOL0` reader - Clock Polarity"]
pub type UCPOL0_R = crate::BitReader<bool>;
#[doc = "Field `UCPOL0` writer - Clock Polarity"]
pub type UCPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0C_SPEC, bool, O>;
#[doc = "Field `UCPHA0` reader - Clock Phase"]
pub type UCPHA0_R = crate::BitReader<bool>;
#[doc = "Field `UCPHA0` writer - Clock Phase"]
pub type UCPHA0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0C_SPEC, bool, O>;
#[doc = "Field `UDORD0` reader - Data Order"]
pub type UDORD0_R = crate::BitReader<bool>;
#[doc = "Field `UDORD0` writer - Data Order"]
pub type UDORD0_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCSR0C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol0(&self) -> UCPOL0_R {
        UCPOL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ucpha0(&self) -> UCPHA0_R {
        UCPHA0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    pub fn udord0(&self) -> UDORD0_R {
        UDORD0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol0(&mut self) -> UCPOL0_W<0> {
        UCPOL0_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ucpha0(&mut self) -> UCPHA0_W<1> {
        UCPHA0_W::new(self)
    }
    #[doc = "Bit 2 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn udord0(&mut self) -> UDORD0_W<2> {
        UDORD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART0 MSPIM Control and Status Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsr0c](index.html) module"]
pub struct UCSR0C_SPEC;
impl crate::RegisterSpec for UCSR0C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucsr0c::R](R) reader structure"]
impl crate::Readable for UCSR0C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsr0c::W](W) writer structure"]
impl crate::Writable for UCSR0C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0C to value 0"]
impl crate::Resettable for UCSR0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
