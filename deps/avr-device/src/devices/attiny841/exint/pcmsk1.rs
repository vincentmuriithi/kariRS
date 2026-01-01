#[doc = "Register `PCMSK1` reader"]
pub struct R(crate::R<PCMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMSK1` writer"]
pub struct W(crate::W<PCMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMSK1_SPEC>;
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
impl From<crate::W<PCMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINT8` reader - Pin Change Enable Mask 1 Bit 0"]
pub type PCINT8_R = crate::BitReader<bool>;
#[doc = "Field `PCINT8` writer - Pin Change Enable Mask 1 Bit 0"]
pub type PCINT8_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK1_SPEC, bool, O>;
#[doc = "Field `PCINT9` reader - Pin Change Enable Mask 1 Bit 1"]
pub type PCINT9_R = crate::BitReader<bool>;
#[doc = "Field `PCINT9` writer - Pin Change Enable Mask 1 Bit 1"]
pub type PCINT9_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK1_SPEC, bool, O>;
#[doc = "Field `PCINT10` reader - Pin Change Enable Mask 1 Bit 2"]
pub type PCINT10_R = crate::BitReader<bool>;
#[doc = "Field `PCINT10` writer - Pin Change Enable Mask 1 Bit 2"]
pub type PCINT10_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK1_SPEC, bool, O>;
#[doc = "Field `PCINT11` reader - Pin Change Enable Mask 1 Bit 3"]
pub type PCINT11_R = crate::BitReader<bool>;
#[doc = "Field `PCINT11` writer - Pin Change Enable Mask 1 Bit 3"]
pub type PCINT11_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin Change Enable Mask 1 Bit 0"]
    #[inline(always)]
    pub fn pcint8(&self) -> PCINT8_R {
        PCINT8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 1 Bit 1"]
    #[inline(always)]
    pub fn pcint9(&self) -> PCINT9_R {
        PCINT9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 1 Bit 2"]
    #[inline(always)]
    pub fn pcint10(&self) -> PCINT10_R {
        PCINT10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 1 Bit 3"]
    #[inline(always)]
    pub fn pcint11(&self) -> PCINT11_R {
        PCINT11_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Enable Mask 1 Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcint8(&mut self) -> PCINT8_W<0> {
        PCINT8_W::new(self)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 1 Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcint9(&mut self) -> PCINT9_W<1> {
        PCINT9_W::new(self)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 1 Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcint10(&mut self) -> PCINT10_W<2> {
        PCINT10_W::new(self)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 1 Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcint11(&mut self) -> PCINT11_W<3> {
        PCINT11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Change Enable Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmsk1](index.html) module"]
pub struct PCMSK1_SPEC;
impl crate::RegisterSpec for PCMSK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcmsk1::R](R) reader structure"]
impl crate::Readable for PCMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmsk1::W](W) writer structure"]
impl crate::Writable for PCMSK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK1 to value 0"]
impl crate::Resettable for PCMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
