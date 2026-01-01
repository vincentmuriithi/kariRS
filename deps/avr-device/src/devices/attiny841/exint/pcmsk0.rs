#[doc = "Register `PCMSK0` reader"]
pub struct R(crate::R<PCMSK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMSK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMSK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMSK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMSK0` writer"]
pub struct W(crate::W<PCMSK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMSK0_SPEC>;
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
impl From<crate::W<PCMSK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMSK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINT0` reader - Pin Change Enable Mask 0 Bit 0"]
pub type PCINT0_R = crate::BitReader<bool>;
#[doc = "Field `PCINT0` writer - Pin Change Enable Mask 0 Bit 0"]
pub type PCINT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT1` reader - Pin Change Enable Mask 0 Bit 1"]
pub type PCINT1_R = crate::BitReader<bool>;
#[doc = "Field `PCINT1` writer - Pin Change Enable Mask 0 Bit 1"]
pub type PCINT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT2` reader - Pin Change Enable Mask 0 Bit 2"]
pub type PCINT2_R = crate::BitReader<bool>;
#[doc = "Field `PCINT2` writer - Pin Change Enable Mask 0 Bit 2"]
pub type PCINT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT3` reader - Pin Change Enable Mask 0 Bit 3"]
pub type PCINT3_R = crate::BitReader<bool>;
#[doc = "Field `PCINT3` writer - Pin Change Enable Mask 0 Bit 3"]
pub type PCINT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT4` reader - Pin Change Enable Mask 0 Bit 4"]
pub type PCINT4_R = crate::BitReader<bool>;
#[doc = "Field `PCINT4` writer - Pin Change Enable Mask 0 Bit 4"]
pub type PCINT4_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT5` reader - Pin Change Enable Mask 0 Bit 5"]
pub type PCINT5_R = crate::BitReader<bool>;
#[doc = "Field `PCINT5` writer - Pin Change Enable Mask 0 Bit 5"]
pub type PCINT5_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT6` reader - Pin Change Enable Mask 0 Bit 6"]
pub type PCINT6_R = crate::BitReader<bool>;
#[doc = "Field `PCINT6` writer - Pin Change Enable Mask 0 Bit 6"]
pub type PCINT6_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
#[doc = "Field `PCINT7` reader - Pin Change Enable Mask 0 Bit 7"]
pub type PCINT7_R = crate::BitReader<bool>;
#[doc = "Field `PCINT7` writer - Pin Change Enable Mask 0 Bit 7"]
pub type PCINT7_W<'a, const O: u8> = crate::BitWriter<'a, u8, PCMSK0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin Change Enable Mask 0 Bit 0"]
    #[inline(always)]
    pub fn pcint0(&self) -> PCINT0_R {
        PCINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 0 Bit 1"]
    #[inline(always)]
    pub fn pcint1(&self) -> PCINT1_R {
        PCINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 0 Bit 2"]
    #[inline(always)]
    pub fn pcint2(&self) -> PCINT2_R {
        PCINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 0 Bit 3"]
    #[inline(always)]
    pub fn pcint3(&self) -> PCINT3_R {
        PCINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin Change Enable Mask 0 Bit 4"]
    #[inline(always)]
    pub fn pcint4(&self) -> PCINT4_R {
        PCINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin Change Enable Mask 0 Bit 5"]
    #[inline(always)]
    pub fn pcint5(&self) -> PCINT5_R {
        PCINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin Change Enable Mask 0 Bit 6"]
    #[inline(always)]
    pub fn pcint6(&self) -> PCINT6_R {
        PCINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin Change Enable Mask 0 Bit 7"]
    #[inline(always)]
    pub fn pcint7(&self) -> PCINT7_R {
        PCINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Enable Mask 0 Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcint0(&mut self) -> PCINT0_W<0> {
        PCINT0_W::new(self)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 0 Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcint1(&mut self) -> PCINT1_W<1> {
        PCINT1_W::new(self)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 0 Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcint2(&mut self) -> PCINT2_W<2> {
        PCINT2_W::new(self)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 0 Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcint3(&mut self) -> PCINT3_W<3> {
        PCINT3_W::new(self)
    }
    #[doc = "Bit 4 - Pin Change Enable Mask 0 Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pcint4(&mut self) -> PCINT4_W<4> {
        PCINT4_W::new(self)
    }
    #[doc = "Bit 5 - Pin Change Enable Mask 0 Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pcint5(&mut self) -> PCINT5_W<5> {
        PCINT5_W::new(self)
    }
    #[doc = "Bit 6 - Pin Change Enable Mask 0 Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pcint6(&mut self) -> PCINT6_W<6> {
        PCINT6_W::new(self)
    }
    #[doc = "Bit 7 - Pin Change Enable Mask 0 Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pcint7(&mut self) -> PCINT7_W<7> {
        PCINT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Change Enable Mask 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmsk0](index.html) module"]
pub struct PCMSK0_SPEC;
impl crate::RegisterSpec for PCMSK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcmsk0::R](R) reader structure"]
impl crate::Readable for PCMSK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmsk0::W](W) writer structure"]
impl crate::Writable for PCMSK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK0 to value 0"]
impl crate::Resettable for PCMSK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
