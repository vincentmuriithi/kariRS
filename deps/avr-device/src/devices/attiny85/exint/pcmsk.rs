#[doc = "Register `PCMSK` reader"]
pub struct R(crate::R<PCMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMSK` writer"]
pub struct W(crate::W<PCMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMSK_SPEC>;
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
impl From<crate::W<PCMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINT0` reader - Enable pin change interrupt on pin 0"]
pub type PCINT0_R = crate::BitReader<bool>;
#[doc = "Field `PCINT0` writer - Enable pin change interrupt on pin 0"]
pub type PCINT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK_SPEC, bool, O>;
#[doc = "Field `PCINT1` reader - Enable pin change interrupt on pin 1"]
pub type PCINT1_R = crate::BitReader<bool>;
#[doc = "Field `PCINT1` writer - Enable pin change interrupt on pin 1"]
pub type PCINT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK_SPEC, bool, O>;
#[doc = "Field `PCINT2` reader - Enable pin change interrupt on pin 2"]
pub type PCINT2_R = crate::BitReader<bool>;
#[doc = "Field `PCINT2` writer - Enable pin change interrupt on pin 2"]
pub type PCINT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK_SPEC, bool, O>;
#[doc = "Field `PCINT3` reader - Enable pin change interrupt on pin 3"]
pub type PCINT3_R = crate::BitReader<bool>;
#[doc = "Field `PCINT3` writer - Enable pin change interrupt on pin 3"]
pub type PCINT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK_SPEC, bool, O>;
#[doc = "Field `PCINT4` reader - Enable pin change interrupt on pin 4"]
pub type PCINT4_R = crate::BitReader<bool>;
#[doc = "Field `PCINT4` writer - Enable pin change interrupt on pin 4"]
pub type PCINT4_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK_SPEC, bool, O>;
#[doc = "Field `PCINT5` reader - Enable pin change interrupt on pin 5"]
pub type PCINT5_R = crate::BitReader<bool>;
#[doc = "Field `PCINT5` writer - Enable pin change interrupt on pin 5"]
pub type PCINT5_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable pin change interrupt on pin 0"]
    #[inline(always)]
    pub fn pcint0(&self) -> PCINT0_R {
        PCINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable pin change interrupt on pin 1"]
    #[inline(always)]
    pub fn pcint1(&self) -> PCINT1_R {
        PCINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable pin change interrupt on pin 2"]
    #[inline(always)]
    pub fn pcint2(&self) -> PCINT2_R {
        PCINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable pin change interrupt on pin 3"]
    #[inline(always)]
    pub fn pcint3(&self) -> PCINT3_R {
        PCINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable pin change interrupt on pin 4"]
    #[inline(always)]
    pub fn pcint4(&self) -> PCINT4_R {
        PCINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable pin change interrupt on pin 5"]
    #[inline(always)]
    pub fn pcint5(&self) -> PCINT5_R {
        PCINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable pin change interrupt on pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcint0(&mut self) -> PCINT0_W<0> {
        PCINT0_W::new(self)
    }
    #[doc = "Bit 1 - Enable pin change interrupt on pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcint1(&mut self) -> PCINT1_W<1> {
        PCINT1_W::new(self)
    }
    #[doc = "Bit 2 - Enable pin change interrupt on pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcint2(&mut self) -> PCINT2_W<2> {
        PCINT2_W::new(self)
    }
    #[doc = "Bit 3 - Enable pin change interrupt on pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcint3(&mut self) -> PCINT3_W<3> {
        PCINT3_W::new(self)
    }
    #[doc = "Bit 4 - Enable pin change interrupt on pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pcint4(&mut self) -> PCINT4_W<4> {
        PCINT4_W::new(self)
    }
    #[doc = "Bit 5 - Enable pin change interrupt on pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pcint5(&mut self) -> PCINT5_W<5> {
        PCINT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Pin Change Enable Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmsk](index.html) module"]
pub struct PCMSK_SPEC;
impl crate::RegisterSpec for PCMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcmsk::R](R) reader structure"]
impl crate::Readable for PCMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmsk::W](W) writer structure"]
impl crate::Writable for PCMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK to value 0"]
impl crate::Resettable for PCMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
